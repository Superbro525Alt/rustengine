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
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { useState, useEffect } from "react";

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

  useEffect(() => {
    const fetchProjects = async () => {
      // Example data for now
      const exampleProjects = [
        { name: "Asteroids", path: "projects/asteroids.json" },
        // { name: "Project 2", path: "/path/to/project2" },
        // { name: "Project 3", path: "/path/to/project3" },
      ];
      setProjects(exampleProjects);
    };

    fetchProjects();
  }, []);

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
    window.location.href = url.toString();
  };

  const handleDeleteProject = (project) => {
    alert(`Deleting project: ${project.name}`);
  };

  const handleDuplicateProject = (project) => {
    alert(`Duplicating project: ${project.name}`);
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
            <h1 className="text-lg font-semibold md:text-2xl">Projects</h1>
            {projects.map((project) => (
              <Card key={project.name} className="mb-4">
                <CardHeader>
                  <CardTitle>{project.name}</CardTitle>
                  <CardDescription>{project.path}</CardDescription>
                </CardHeader>
                <CardContent>
                  <div className="flex gap-2">
                    <Button onClick={() => handleOpenProject(project)}>Open</Button>
                    <Button variant="destructive" onClick={() => handleDeleteProject(project)}>Delete</Button>
                    <Button onClick={() => handleDuplicateProject(project)}>Duplicate</Button>
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
            <div className="rounded-lg border border-dashed p-4 text-center">
              Browse plugins and marketplace items.
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
    const baseClass =
      "flex items-center gap-3 rounded-lg px-3 py-2 transition-all mb-2";
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
              <Package2 className="h-6 w-6" />
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
                    6
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
      </div>
    </div>
  );
}
