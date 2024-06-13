//@ts-nocheck
"use client";

import Link from "next/link";
import {
  Bell,
  CircleUser,
  Home as HomeIcon,
  Menu,
  Package,
  Package2,
  Folder,
} from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState, useEffect } from "react";
import { readDir, readTextFile, writeTextFile, removeFile, createDir } from '@tauri-apps/api/fs';
// import { join } from '@tauri-apps/api/path';
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import { useNavigate } from 'react-router-dom';
// use the path module from Node.js
import path from 'path';
// import { join } from "path";
import { documentDir, homeDir } from "@tauri-apps/api/path";
import { useToast } from "@/components/ui/use-toast"

async function basename(filePath: string): Promise<string> {
    return path.basename(filePath);
}
const examplePlugins = [
  { name: "Plugin 1", description: "This is the first plugin.", img: "https://via.placeholder.com/150" },
  { name: "Plugin 2", description: "This is the second plugin.", img: "https://via.placeholder.com/150" },
  { name: "Plugin 3", description: "This is the third plugin.", img: "https://via.placeholder.com/150" },
  { name: "Plugin 4", description: "This is the fourth plugin.", img: "https://via.placeholder.com/150" },
  { name: "Plugin 5", description: "This is the fifth plugin.", img: "https://via.placeholder.com/150" },
  { name: "Plugin 6", description: "This is the sixth plugin.", img: "https://via.placeholder.com/150" },
];

export default function Home() {
  const [selectedMenu, setSelectedMenu] = useState("Dashboard");
  const [projects, setProjects] = useState([]);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [isSupportOpen, setIsSupportOpen] = useState(false);
  const [isSignInOpen, setIsSignInOpen] = useState(false);
  const [isSignUpOpen, setIsSignUpOpen] = useState(false);
  const [isSignedIn, setIsSignedIn] = useState(false);
  const [users, setUsers] = useState([]);
  const [currentUser, setCurrentUser] = useState(null);
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false);
  const [isDuplicateDialogOpen, setIsDuplicateDialogOpen] = useState(false);
  const [isInstallDialogOpen, setIsInstallDialogOpen] = useState(false);
  const [selectedProject, setSelectedProject] = useState(null);
  const [selectedPlugin, setSelectedPlugin] = useState(null);
  const [isNewProjectDialogOpen, setIsNewProjectDialogOpen] = useState(false);
  const [newProjectName, setNewProjectName] = useState("");

  const { toast } = useToast();

  function join(...parts: string[]): Promise<string> {
    return new Promise((resolve) => {
        const separator = '/';
        const result = parts.reduce((acc, part) => {
            if (acc.endsWith(separator)) {
                if (part.startsWith(separator)) {
                    return acc + part.slice(1);
                }
                return acc + part;
            } else {
                if (part.startsWith(separator)) {
                    return acc + part;
                }
                return acc + separator + part;
            }
        }, '');
        resolve(result);
    });
  }

  
  const getDocumentDir = async () => {
    try {
      const dir = await invoke('get_document_dir');
      return dir;
    } catch (err) {
      console.error("Error retrieving document directory:", err);
      return "./";
    }
  }

  const fetchProjects = async () => {
    const documentsDir = await getDocumentDir();
    const projectsDir = documentsDir + 'oxidized/projects';

    // if folder does not exist, create it. Use tauri fs operations
    try {
      await createDir(projectsDir, { recursive: true });
    } catch (error) {
      console.error('Failed to create projects directory:', error);
    }

    try {
      const entries = await readDir(projectsDir);
      const projectFiles = entries.filter(entry => entry.children === undefined);
      const projects = await Promise.all(projectFiles.map(async file => ({
        name: await basename(file.path),
        path: file.path
      })));
      setProjects(projects);
    } catch (error) {
      console.error('Failed to load projects:', error);
    }
  };
  useEffect(() => {
    fetchProjects();
  }, []);

  const refreshProjects = async () => {
    fetchProjects();
  };

  const openSettingsPopup = () => setIsSettingsOpen(true);
  const closeSettingsPopup = () => setIsSettingsOpen(false);

  const openSupportPopup = () => setIsSupportOpen(true);
  const closeSupportPopup = () => setIsSupportOpen(false);

  const openSignInPopup = () => setIsSignInOpen(true);
  const closeSignInPopup = () => setIsSignInOpen(false);

  const openSignUpPopup = () => setIsSignUpOpen(true);
  const closeSignUpPopup = () => setIsSignUpOpen(false);

  const handleSignOut = () => {
    setCurrentUser(null);
    setIsSignedIn(false);
  };

  const handleSignIn = (e) => {
    e.preventDefault();
    const existingUser = users.find((user) => user.username === username && user.password === password);
    if (existingUser) {
      setCurrentUser(existingUser);
      setIsSignedIn(true);
      closeSignInPopup();
    } else {
      alert("Invalid username or password");
    }
  };

  const handleSignUp = (e) => {
    e.preventDefault();
    const newUser = { username, password };
    setUsers([...users, newUser]);
    setCurrentUser(newUser);
    setIsSignedIn(true);
    closeSignUpPopup();
  };

  const handleOpenProject = (project) => {
    const url = new URL('/edit', window.location.origin);
    url.searchParams.set('name', project.name);
    url.searchParams.set('path', project.path);
    window.location.assign(url);
  };

  const handleDeleteProject = async () => {
    if (selectedProject) {
      try {
        await removeFile(selectedProject.path);
        setIsDeleteDialogOpen(false);
        setSelectedProject(null);
        refreshProjects();
      } catch (error) {
        console.error(`Failed to delete project: ${selectedProject.name}`, error);
      }
    }
  };

  const handleDuplicateProject = async () => {
    if (selectedProject) {
      try {
        const projectContent = await readTextFile(selectedProject.path);
        const projectName = selectedProject.name.replace('.json', '');
        let newProjectName = `${projectName}_copy.json`;
        let newProjectPath = "projects/" + newProjectName;

        // Check if the file already exists, if so, keep appending '_copy'
        while (projects.some(project => project.path.endsWith(newProjectName))) {
          const baseName = newProjectName.replace('.json', '');
          newProjectName = `${baseName}_copy.json`;
          newProjectPath = "projects/" + newProjectNam;
        }
        //
        await writeTextFile(newProjectPath, projectContent);
        setIsDuplicateDialogOpen(false);
        setSelectedProject(null);
        refreshProjects();
      } catch (error) {
        console.error(`Failed to duplicate project: ${selectedProject.name}`, error);
      }
    }
  };

  const handleInstallPlugin = (plugin) => {
    setSelectedPlugin(plugin);
    setIsInstallDialogOpen(true);
  };

  const handleNewProject = async () => {
    setIsNewProjectDialogOpen(true);
  };

  const createNewProject = async () => {
    if (!newProjectName) {
      toast({
        title: "Error",
        description: "Project name cannot be empty.",
        status: "error",
        duration: 3000,
      });
      return;
    }

    const documentsDir = await getDocumentDir();
    const projectsDir = await join(documentsDir, 'oxidized/projects');
    const newProjectPath = await join(projectsDir, `${newProjectName}.json`);

    try {
      const existingProjects = await readDir(projectsDir);
      const projectExists = existingProjects.some(project => project.path === newProjectPath);

      if (projectExists) {
        toast({
          title: "Error",
          description: "A project with this name already exists.",
          status: "error",
          duration: 3000,
        });
        return;
      }

      console.log('Creating new project:', newProjectPath);

      const defaultContent = JSON.stringify({
        objects: [],
        static_components: [],
        graphics: true
      }, null, 2);

      console.log('Creating new project:', newProjectPath);

      await writeTextFile(newProjectPath, defaultContent);

      setIsNewProjectDialogOpen(false);
      setNewProjectName("");
      refreshProjects();

      toast({
        title: "Success",
        description: "Project created successfully.",
        status: "success",
        duration: 3000,
      });
    } catch (error) {
      console.error("Failed to create a new project: ", error);
      toast({
        title: "Error",
        description: "Failed to create a new project. Please try again.",
        status: "error",
        duration: 3000,
      });
    }
  };

  const renderContent = () => {
    switch (selectedMenu) {
      case "Dashboard":
        return (
          <div className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
            <h1 className="text-lg font-semibold md:text-2xl">Dashboard</h1>
            <div className="rounded-lg border border-dashed p-4 text-center">
              Welcome to the Dashboard!
            </div>
          </div>
        );
      case "Projects":
        return (
          <div className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
            <div className="flex flex-row gap-5">
              <h1 className="text-lg font-semibold md:text-2xl pt-1 w-full">Projects</h1> 
              <Button onClick={() => handleNewProject()} className="w-1/2">New Project</Button>
            </div>
            {projects.map((project) => (
              <Card key={project.name} className="mb-4">
                <CardHeader>
                  <CardTitle>{project.name}</CardTitle>
                  <CardDescription>{project.path}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex gap-2">
                    <Button onClick={() => handleOpenProject(project)}>Open</Button>
                    <Button variant="destructive" onClick={() => { setSelectedProject(project); setIsDeleteDialogOpen(true); }}>Delete</Button>
                    <Button onClick={() => { setSelectedProject(project); setIsDuplicateDialogOpen(true); }}>Duplicate</Button>
                  </div>
                </CardContent>
              </Card>
            ))}
          </div>
        );
      case "Plugins & Marketplace":
        return (
          <div className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
            <h1 className="text-lg font-semibold md:text-2xl">Plugins & Marketplace</h1>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {examplePlugins.map((plugin) => (
                <Card key={plugin.name} className="mb-4">
                  <CardHeader className="">
                    <div>
                      <CardTitle className="flex flex-row gap-5 mb-3">
                        <Avatar>
                          <AvatarImage src={plugin.img} />
                          <AvatarFallback>PL</AvatarFallback>
                        </Avatar>
                        <span className="pt-2">
                          {plugin.name}
                        </span>
                      </CardTitle>
                      <CardDescription>{plugin.description}</CardDescription>
                    </div>
                  </CardHeader>
                  <CardContent>
                    <Button className="w-full mt-2" onClick={() => handleInstallPlugin(plugin)}>Install</Button>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        );
      default:
        return (
          <div className="flex flex-1 flex-col gap-4 p-4 lg:gap-6 lg:p-6">
            <h1 className="text-lg font-semibold md:text-2xl">Dashboard</h1>
            <div className="rounded-lg border border-dashed p-4 text-center">
              Welcome to the Dashboard!
            </div>
          </div>
        );
    }
  };

  const getButtonClass = (menu) => {
    const baseClass = "flex items-center gap-3 rounded-lg px-3 py-2 transition-all mb-2";
    const hoverClass = "hover:bg-primary/10";
    const activeClass = "bg-secondary text-secondary-foreground";

    if (selectedMenu === menu) {
      return `${baseClass} ${activeClass}`;
    }
    return `${baseClass} text-muted-foreground ${hoverClass}`;
  };

  return (
    <div className="grid min-h-screen w-full md:grid-cols-[220px_1fr] lg:grid-cols-[280px_1fr]">
      <div className="sticky top-0 hidden h-screen border-r bg-muted/40 md:block">
        <div className="flex h-full max-h-screen flex-col gap-2">
          <div className="flex h-14 items-center border-b px-4 lg:h-[60px] lg:px-6">
            <Link href="/" className="flex items-center gap-2 font-semibold">
              <Avatar>
                <AvatarImage src="logo.png"/>
                <AvatarFallback>OX</AvatarFallback>
              </Avatar>
              <span className="">Oxidized</span>
            </Link>
            <Button variant="outline" size="icon" className="ml-auto h-8 w-8">
              <Bell className="h-4 w-4" />
              <span className="sr-only">Toggle notifications</span>
            </Button>
          </div>
          <div className="flex-1">
            <nav className="grid items-start px-2 text-sm font-medium lg:px-4">
              <button
                onClick={() => setSelectedMenu("Dashboard")}
                className={getButtonClass("Dashboard")}
              >
                <HomeIcon className="h-4 w-4" />
                Dashboard
              </button>
              <button
                onClick={() => setSelectedMenu("Projects")}
                className={getButtonClass("Projects")}
              >
                <Folder className="h-4 w-4" />
                Projects
                <Badge className="ml-auto flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-white">
                  {projects.length}
                </Badge>
              </button>
              <button
                onClick={() => setSelectedMenu("Plugins & Marketplace")}
                className={getButtonClass("Plugins & Marketplace")}
              >
                <Package className="h-4 w-4" />
                Plugins & Marketplace
              </button>
            </nav>
          </div>
        </div>
      </div>
      <div className="flex flex-col">
        <header className="sticky top-0 z-50 flex h-14 items-center gap-4 border-b bg-muted/40 px-4 lg:h-[60px] lg:px-6">
          <Sheet>
            <SheetTrigger asChild>
              <Button variant="outline" size="icon" className="shrink-0 md:hidden">
                <Menu className="h-5 w-5" />
                <span className="sr-only">Toggle navigation menu</span>
              </Button>
            </SheetTrigger>
            <SheetContent side="left" className="flex flex-col">
              <nav className="grid gap-2 text-lg font-medium">
                <Link
                  href="#"
                  className="flex items-center gap-2 text-lg font-semibold"
                >
                  <Package2 className="h-6 w-6" />
                  <span className="sr-only">Oxidized</span>
                </Link>
                <button
                  onClick={() => setSelectedMenu("Dashboard")}
                  className={getButtonClass("Dashboard")}
                >
                  <HomeIcon className="h-5 w-5" />
                  Dashboard
                </button>
                <button
                  onClick={() => setSelectedMenu("Projects")}
                  className={getButtonClass("Projects")}
                >
                  <Folder className="h-5 w-5" />
                  Projects
                  <Badge className="ml-auto flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-white">
                    {projects.length}
                  </Badge>
                </button>
                <button
                  onClick={() => setSelectedMenu("Plugins & Marketplace")}
                  className={getButtonClass("Plugins & Marketplace")}
                >
                  <Package className="h-5 w-5" />
                  Plugins & Marketplace
                </button>
              </nav>
              <div className="mt-auto">
                <Card>
                  <CardHeader>
                    <CardTitle>Upgrade to Pro</CardTitle>
                    <CardDescription>
                      Unlock all features and get unlimited access to our support team.
                    </CardDescription>
                  </CardHeader>
                  <CardContent>
                    <Button size="sm" className="w-full">
                      Upgrade
                    </Button>
                  </CardContent>
                </Card>
              </div>
            </SheetContent>
          </Sheet>
          <div className="w-full flex-1"></div>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="secondary" size="icon" className="rounded-full">
                <CircleUser className="h-5 w-5" />
                <span className="sr-only">Toggle user menu</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuLabel>My Account</DropdownMenuLabel>
              <DropdownMenuSeparator />
              {isSignedIn ? (
                <>
                  <DropdownMenuItem onClick={openSettingsPopup}>Settings</DropdownMenuItem>
                  <DropdownMenuItem onClick={openSupportPopup}>Support</DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem onClick={handleSignOut}>Logout</DropdownMenuItem>
                </>
              ) : (
                <>
                  <DropdownMenuItem onClick={openSignInPopup}>Sign In</DropdownMenuItem>
                  <DropdownMenuItem disabled className="line-through">Settings</DropdownMenuItem>
                  <DropdownMenuItem disabled className="line-through">Support</DropdownMenuItem>
                </>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        </header>
        {renderContent()}
        <Dialog open={isSettingsOpen} onOpenChange={setIsSettingsOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Settings</DialogTitle>
            </DialogHeader>
            <p>Settings content goes here...</p>
            <Button onClick={closeSettingsPopup}>Close</Button>
          </DialogContent>
        </Dialog>
        <Dialog open={isSupportOpen} onOpenChange={setIsSupportOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Support</DialogTitle>
            </DialogHeader>
            <p>Support content goes here...</p>
            <Button onClick={closeSupportPopup}>Close</Button>
          </DialogContent>
        </Dialog>
        <Dialog open={isSignInOpen} onOpenChange={setIsSignInOpen}>
          <DialogContent>
            <Card className="mx-auto max-w-sm">
              <CardHeader>
                <CardTitle className="text-2xl">Login</CardTitle>
                <CardDescription>
                  Enter your email below to login to your account
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleSignIn} className="grid gap-4">
                  <div className="grid gap-2">
                    <Label htmlFor="email">Email</Label>
                    <Input
                      id="email"
                      type="email"
                      placeholder="m@example.com"
                      value={username}
                      onChange={(e) => setUsername(e.target.value)}
                      required
                    />
                  </div>
                  <div className="grid gap-2">
                    <div className="flex items-center">
                      <Label htmlFor="password">Password</Label>
                      <Link href="#" className="ml-auto inline-block text-sm underline">
                        Forgot your password?
                      </Link>
                    </div>
                    <Input
                      id="password"
                      type="password"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                      required
                    />
                  </div>
                  <Button type="submit" className="w-full">
                    Login
                  </Button>
                </form>
                <div className="mt-4 text-center text-sm">
                  Don&apos;t have an account?{" "}
                  <button
                    type="button"
                    className="text-blue-500 hover:underline"
                    onClick={() => {
                      closeSignInPopup();
                      openSignUpPopup();
                    }}
                  >
                    Sign up
                  </button>
                </div>
                <Button onClick={closeSignInPopup} className="mt-4">
                  Close
                </Button>
              </CardContent>
            </Card>
          </DialogContent>
        </Dialog>
        <Dialog open={isSignUpOpen} onOpenChange={setIsSignUpOpen}>
          <DialogContent>
            <Card className="mx-auto max-w-sm">
              <CardHeader>
                <CardTitle className="text-xl">Sign Up</CardTitle>
                <CardDescription>
                  Enter your information to create an account
                </CardDescription>
              </CardHeader>
              <CardContent>
                <form onSubmit={handleSignUp} className="grid gap-4">
                  <div className="grid gap-2">
                    <Label htmlFor="username">Username</Label>
                    <Input
                      id="username"
                      type="text"
                      placeholder="Username"
                      value={username}
                      onChange={(e) => setUsername(e.target.value)}
                      required
                    />
                  </div>
                  <div className="grid gap-2">
                    <Label htmlFor="password">Password</Label>
                    <Input
                      id="password"
                      type="password"
                      placeholder="Password"
                      value={password}
                      onChange={(e) => setPassword(e.target.value)}
                      required
                    />
                  </div>
                  <Button type="submit" className="w-full">
                    Sign Up
                  </Button>
                </form>
                <div className="mt-4 text-center text-sm">
                  Already have an account?{" "}
                  <button
                    type="button"
                    className="text-blue-500 hover:underline"
                    onClick={() => {
                      closeSignUpPopup();
                      openSignInPopup();
                    }}
                  >
                    Sign in
                  </button>
                </div>
                <Button onClick={closeSignUpPopup} className="mt-4">
                  Close
                </Button>
              </CardContent>
            </Card>
          </DialogContent>
        </Dialog>
        <Dialog open={isDeleteDialogOpen} onOpenChange={setIsDeleteDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Delete Project</DialogTitle>
            </DialogHeader>
            <p>Are you sure you want to delete the project: {selectedProject?.name}?</p>
            <DialogFooter>
              <Button onClick={handleDeleteProject}>Delete</Button>
              <Button variant="outline" onClick={() => setIsDeleteDialogOpen(false)}>Cancel</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <Dialog open={isDuplicateDialogOpen} onOpenChange={setIsDuplicateDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Duplicate Project</DialogTitle>
            </DialogHeader>
            <p>Are you sure you want to duplicate the project: {selectedProject?.name}?</p>
            <DialogFooter>
              <Button onClick={handleDuplicateProject}>Duplicate</Button>
              <Button variant="outline" onClick={() => setIsDuplicateDialogOpen(false)}>Cancel</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <Dialog open={isInstallDialogOpen} onOpenChange={setIsInstallDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>Install Plugin</DialogTitle>
            </DialogHeader>
            <p>Are you sure you want to install the plugin: {selectedPlugin?.name}?</p>
            <DialogFooter>
              <Button onClick={() => setIsInstallDialogOpen(false)}>Confirm</Button>
              <Button variant="outline" onClick={() => setIsInstallDialogOpen(false)}>Cancel</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <Dialog open={isNewProjectDialogOpen} onOpenChange={setIsNewProjectDialogOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>New Project</DialogTitle>
            </DialogHeader>
            <div className="grid gap-2">
              <Label htmlFor="projectName">Project Name</Label>
              <Input
                id="projectName"
                type="text"
                placeholder="Enter project name"
                value={newProjectName}
                onChange={(e) => setNewProjectName(e.target.value)}
                required
              />
            </div>
            <DialogFooter>
              <Button onClick={createNewProject}>Create</Button>
              <Button variant="outline" onClick={() => setIsNewProjectDialogOpen(false)}>Cancel</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
      </div>
    </div>
  );
}
