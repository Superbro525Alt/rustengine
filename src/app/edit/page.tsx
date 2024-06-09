"use client";
import { useState, useEffect, useRef } from "react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { Checkbox } from "@/components/ui/checkbox";
import { ChevronRight, ChevronDown, Plus, Trash, Play, StopCircle, Square, Pause } from "lucide-react";
import { Command, CommandItem, CommandList, CommandInput, CommandGroup, CommandEmpty } from "@/components/ui/command";
import { Dialog, DialogOverlay, DialogContent } from "@/components/ui/dialog";

const initialHierarchy = [
  {
    id: 1,
    name: "GameObject1",
    expanded: true,
    components: [
      { id: 1, type: "Transform", properties: { Position: [0, 0, 0], Rotation: [0, 0, 0], Scale: [1, 1, 1] } },
      { id: 2, type: "MeshRenderer", properties: { Material: "Default", Enabled: true } },
    ],
    children: [
      {
        id: 2,
        name: "ChildObject1",
        expanded: true,
        components: [{ id: 3, type: "Transform", properties: { Position: [0, 0, 0], Rotation: [0, 0, 0], Scale: [1, 1, 1] } }],
        children: [],
      },
    ],
  },
  {
    id: 3,
    name: "GameObject2",
    expanded: true,
    components: [
      { id: 4, type: "Transform", properties: { Position: [0, 0, 0], Rotation: [0, 0, 0], Scale: [1, 1, 1] } },
      { id: 5, type: "Light", properties: { Color: "#FFFFFF", Intensity: 1.0 } },
    ],
    children: [],
  },
];

const initialComponents = {
  Transform: [
    { name: "Position", type: "Vector3" },
    { name: "Rotation", type: "Vector3" },
    { name: "Scale", type: "Vector3" },
  ],
  MeshRenderer: [
    { name: "Material", type: "String" },
    { name: "Enabled", type: "Boolean" },
  ],
  Light: [
    { name: "Color", type: "Color" },
    { name: "Intensity", type: "Float" },
  ],
};

const initialStaticComponentsList = {
  Physics: [
    { name: "Mass", type: "Float" },
    { name: "Drag", type: "Float" },
    { name: "AngularDrag", type: "Float" },
  ],
  AudioSource: [
    { name: "Clip", type: "String" },
    { name: "Volume", type: "Float" },
    { name: "Loop", type: "Boolean" },
  ],
};

const initialStaticComponents = [];

export default function Edit() {
  const [hierarchy, setHierarchy] = useState(initialHierarchy);
  const [selectedObject, setSelectedObject] = useState(null);
  const [components] = useState(initialComponents);
  const [staticComponents, setStaticComponents] = useState(initialStaticComponents);
  const [staticComponentsList] = useState(initialStaticComponentsList);
  const [selectedStaticComponent, setSelectedStaticComponent] = useState(null);
  const [showCommandMenu, setShowCommandMenu] = useState(false);
  const [commandMenuType, setCommandMenuType] = useState("normal"); // "normal" or "static"
  const commandMenuRef = useRef();
  const [forceRenderKey, setForceRenderKey] = useState(0); // New state to force re-render

  useEffect(() => {
    const handleClickOutside = (event) => {
      if (commandMenuRef.current && !commandMenuRef.current.contains(event.target)) {
        setShowCommandMenu(false);
      }
    };

    const handleEscapeKey = (event) => {
      if (event.key === "Escape") {
        setShowCommandMenu(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    document.addEventListener("keydown", handleEscapeKey);

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("keydown", handleEscapeKey);
    };
  }, []);

  useEffect(() => {
    setForceRenderKey(forceRenderKey + 1);
  }, [selectedObject, selectedStaticComponent]);

  const toggleExpand = (object) => {
    object.expanded = !object.expanded;
    setHierarchy([...hierarchy]);
  };

  const addObject = (parent) => {
    const newObject = {
      id: Date.now(),
      name: `New GameObject`,
      expanded: true,
      components: [{ id: Date.now(), type: "Transform", properties: { Position: [0, 0, 0], Rotation: [0, 0, 0], Scale: [1, 1, 1] } }],
      children: [],
    };
    if (parent) {
      parent.children.push(newObject);
    } else {
      hierarchy.push(newObject);
    }
    setHierarchy([...hierarchy]);
  };

  const deleteObject = (object) => {
    const removeObject = (list, id) => {
      return list.filter((item) => {
        if (item.id === id) return false;
        item.children = removeObject(item.children, id);
        return true;
      });
    };
    setHierarchy(removeObject(hierarchy, object.id));
    if (selectedObject && selectedObject.id === object.id) {
      setSelectedObject(null);
    }
  };

  const deleteStaticComponent = (component) => {
    setStaticComponents(staticComponents.filter((comp) => comp.id !== component.id));
    if (selectedStaticComponent && selectedStaticComponent.id === component.id) {
      setSelectedStaticComponent(null);
    }
  };

  const renameObject = (object, newName) => {
    object.name = newName;
    setHierarchy([...hierarchy]);
  };

  const handleComponentPropertyChange = (component, property, value) => {
    component.properties[property] = value;
    setHierarchy([...hierarchy]);
  };

  const handleStaticComponentPropertyChange = (component, property, value) => {
    component.properties[property] = value;
    setStaticComponents([...staticComponents]);
  };

  const renderPropertyInput = (component, property, type) => {
    switch (type) {
      case "Vector3":
        return (
          <div className="flex space-x-2">
            {["X", "Y", "Z"].map((axis, index) => (
              <Input
                key={axis}
                value={component.properties[property][index]}
                onChange={(e) => handleComponentPropertyChange(component, property, [
                  ...component.properties[property].slice(0, index),
                  parseFloat(e.target.value),
                  ...component.properties[property].slice(index + 1),
                ])}
                placeholder={axis}
              />
            ))}
          </div>
        );
      case "String":
        return (
          <Input
            value={component.properties[property]}
            onChange={(e) => handleComponentPropertyChange(component, property, e.target.value)}
          />
        );
      case "Boolean":
        return (
          <>
            <br />
            <Checkbox
              checked={component.properties[property]}
              onCheckedChange={(e) => handleComponentPropertyChange(component, property, e)}
            />
          </>
        );
      case "Color":
        return (
          <Input
            type="color"
            value={component.properties[property]}
            onChange={(e) => handleComponentPropertyChange(component, property, e.target.value)}
          />
        );
      case "Float":
        return (
          <Input
            type="number"
            step="0.01"
            value={component.properties[property]}
            onChange={(e) => handleComponentPropertyChange(component, property, parseFloat(e.target.value))}
          />
        );
      default:
        return (
          <Input
            value={component.properties[property]}
            onChange={(e) => handleComponentPropertyChange(component, property, e.target.value)}
          />
        );
    }
  };

  const renderComponents = (componentsList) =>
    componentsList.map((component) => (
      <Card key={component.id} className="mb-2">
        <CardHeader>
          <CardTitle>{component.type}</CardTitle>
        </CardHeader>
        <CardContent>
          {components[component.type] &&
            components[component.type].map((property) => (
              <div key={property.name} className="mb-2">
                <Label>{property.name}</Label>
                {renderPropertyInput(component, property.name, property.type)}
              </div>
            ))}
        </CardContent>
      </Card>
    ));

  const addComponentToSelectedObject = (type) => {
    const newComponent = {
      id: Date.now(),
      type,
      properties: Object.fromEntries(components[type].map((prop) => [prop.name, prop.type === "Vector3" ? [0, 0, 0] : prop.type === "Boolean" ? false : prop.type === "Float" ? 0.0 : ""])),
    };
    selectedObject.components.push(newComponent);
    setHierarchy([...hierarchy]);
    setShowCommandMenu(false);
  };

  const addStaticComponent = (type) => {
    const newComponent = {
      id: Date.now(),
      type,
      properties: Object.fromEntries(staticComponentsList[type].map((prop) => [prop.name, prop.type === "Vector3" ? [0, 0, 0] : prop.type === "Boolean" ? false : prop.type === "Float" ? 0.0 : ""])),
    };
    setStaticComponents([...staticComponents, newComponent]);
    setShowCommandMenu(false);
  };

  const getAvailableComponents = () => {
    const existingComponentTypes = selectedObject ? selectedObject.components.map((component) => component.type) : [];
    return Object.keys(components).filter((type) => !existingComponentTypes.includes(type));
  };

  const getAvailableStaticComponents = () => {
    const existingComponentTypes = staticComponents.map((component) => component.type);
    return Object.keys(staticComponentsList).filter((type) => !existingComponentTypes.includes(type));
  };

  const renderHierarchy = (hierarchy) =>
    hierarchy.map((object) => (
      <div key={object.id} className="pl-2">
        <div className="flex items-center cursor-pointer hover:bg-muted" onClick={() => {
          setSelectedObject(object);
          setSelectedStaticComponent(null);
        }}>
          <Button variant="ghost" size="icon" onClick={() => toggleExpand(object)}>
            {object.expanded ? <ChevronDown size={16} /> : <ChevronRight size={16} />}
          </Button>
          <Input
            className="bg-transparent border-none p-0 m-0 w-full"
            defaultValue={object.name}
            onBlur={(e) => renameObject(object, e.target.value)}
          />
          <Button variant="ghost" size="icon" onClick={() => addObject(object)}>
            <Plus size={16} />
          </Button>
          <Button variant="ghost" size="icon" onClick={() => deleteObject(object)}>
            <Trash size={16} />
          </Button>
        </div>
        {object.expanded && object.children.length > 0 && (
          <div className="pl-4">{renderHierarchy(object.children)}</div>
        )}
      </div>
    ));

  const renderStaticComponents = (componentsList) =>
    componentsList.map((component) => (
      <div key={component.id} className="pl-2">
        <div className="flex items-center cursor-pointer hover:bg-muted" onClick={() => {
          setSelectedStaticComponent(component);
          setSelectedObject(null);
        }}>
          <Button variant="ghost" size="icon">
            <ChevronRight size={16} />
          </Button>
          <span className="w-full">{component.type}</span>
          <Button variant="ghost" size="icon" onClick={() => deleteStaticComponent(component)}>
            <Trash size={16} />
          </Button>
        </div>
      </div>
    ));

  const start_preview = () => {};
  const pause_preview = () => {};
  const stop_preview = () => {};

  return (
    <div className="flex h-screen" key={forceRenderKey}>
      <div className="w-1/4 border-r flex flex-col">
        <ScrollArea className="h-1/2">
          <h1 className="p-4 text-xl font-semibold">Hierarchy</h1>
          <div className="p-4">
            {renderHierarchy(hierarchy)}
            <Button onClick={() => addObject(null)} className="mt-4 w-full">
              <Plus size={16} className="mr-2" />
              Add GameObject
            </Button>
          </div>
        </ScrollArea>
        <Separator />
        <ScrollArea className="h-1/2">
          <h1 className="p-4 text-xl font-semibold">Static Components</h1>
          <div className="p-4">
            {renderStaticComponents(staticComponents)}
            <Button onClick={() => { setShowCommandMenu(true); setCommandMenuType("static"); }} className="mt-4 w-full">
              <Plus size={16} className="mr-2" />
              Add Static Component
            </Button>
          </div>
        </ScrollArea>
        <Separator/>
        <div className="flex flex-row gap-5 h-1/12 p-5 items-center w-full justify-center">
          <Button onClick={() => {start_preview()}}>
            <Play size={16}/>
          </Button>
          <Button onClick={() => {pause_preview()}}>
            <Pause size={16}/>
          </Button>
          <Button onClick={() => {stop_preview()}}>
            <Square size={16}/>
          </Button>
        </div>
      </div>
      <ScrollArea className="w-3/4 h-full">
        {selectedObject ? (
          <div className="p-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-2xl">{selectedObject.name}</CardTitle>
                <CardDescription>Components</CardDescription>
              </CardHeader>
              <CardContent>
                {renderComponents(selectedObject.components)}
                <Separator className="my-4" />
                <Button onClick={() => { setShowCommandMenu(true); setCommandMenuType("normal"); }} className="w-full">
                  <Plus size={16} className="mr-2" />
                  Add Component
                </Button>
              </CardContent>
            </Card>
          </div>
        ) : selectedStaticComponent ? (
          <div className="p-4">
            <Card>
              <CardHeader>
                <CardTitle>{selectedStaticComponent.type}</CardTitle>
              </CardHeader>
              <CardContent>
                {staticComponentsList[selectedStaticComponent.type] &&
                  staticComponentsList[selectedStaticComponent.type].map((property) => (
                    <div key={property.name} className="mb-2">
                      <Label>{property.name}</Label>
                      {renderPropertyInput(selectedStaticComponent, property.name, property.type)}
                    </div>
                  ))}
              </CardContent>
            </Card>
          </div>
        ) : (
          <div className="text-center text-xl p-4">Nothing selected</div>
        )}
        {showCommandMenu && (
          <Dialog open={showCommandMenu} onOpenChange={setShowCommandMenu}>
            <DialogOverlay className="fixed bg-black bg-opacity-50" />
            <DialogContent className="fixed bg-gray-800 p-4 rounded-lg shadow-lg flex flex-col items-center" ref={commandMenuRef}>
              <Command className="dark w-full">
                <div className="items-center w-full mb-2">
                  <CommandInput placeholder="Search components..." autoFocus className="flex-1 w-full" />
                </div>
                <CommandList>
                  <CommandGroup heading="Components">
                    {commandMenuType === "normal" ? (
                      getAvailableComponents().length > 0 ? (
                        getAvailableComponents().map((type) => (
                          <CommandItem key={type} onSelect={() => addComponentToSelectedObject(type)}>
                            {type}
                          </CommandItem>
                        ))
                      ) : (
                        <CommandEmpty>No components left to add</CommandEmpty>
                      )
                    ) : (
                      getAvailableStaticComponents().length > 0 ? (
                        getAvailableStaticComponents().map((type) => (
                          <CommandItem key={type} onSelect={() => addStaticComponent(type)}>
                            {type}
                          </CommandItem>
                        ))
                      ) : (
                        <CommandEmpty>No static components left to add</CommandEmpty>
                      )
                    )}
                  </CommandGroup>
                </CommandList>
              </Command>
            </DialogContent>
          </Dialog>
        )}
      </ScrollArea>
    </div>
  );
}
