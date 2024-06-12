"use client";
import { useState, useEffect, useRef } from "react";
import { useSearchParams } from "next/navigation"; // For fetching URL parameters
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
import { ChevronRight, ChevronDown, Plus, Trash, Play, Pause, Square } from "lucide-react";
import { Command, CommandItem, CommandList, CommandInput, CommandGroup, CommandEmpty } from "@/components/ui/command";
import { Dialog, DialogOverlay, DialogContent } from "@/components/ui/dialog";
import { invoke } from '@tauri-apps/api/tauri';
import { readTextFile } from '@tauri-apps/api/fs';
import { v4 } from "uuid";

const initialComponents = {
  Transform: [
    { name: "pos", type: "Vector3" },
    { name: "rot", type: "Vector3" },
  ],
  CharacterController2D: [
    { name: "bounds", type: "Object" },
    { name: "moveamt", type: "Float" },
    { name: "rotamt", type: "Float" },
  ],
  RenderComponent: [
    { name: "obj", type: "Object" },
  ],
};

const registerComponents = (data) => {
  const components = { ...initialComponents };
  const staticComponents = {};

  data.objects.forEach((object) => {
    object.components.forEach((component) => {
      if (!components[component.id]) {
        components[component.id] = Object.keys(component.data)
          .filter((key) => key !== "state" && key !== "uuid")
          .map((key) => ({
            name: key,
            type: Array.isArray(component.data[key]) && component.data[key].length === 3 ? "Vector3" : typeof component.data[key],
          }));
      }
    });
  });

  data.static_components.forEach((component) => {
    if (!staticComponents[component.id]) {
      staticComponents[component.id] = Object.keys(component.data)
        .filter((key) => key !== "state" && key !== "uuid")
        .map((key) => ({
          name: key,
          type: Array.isArray(component.data[key]) && component.data[key].length === 3 ? "Vector3" : typeof component.data[key],
        }));
    }
  });

  return { components, staticComponents };
};

export default function Edit() {
  const [hierarchy, setHierarchy] = useState([]);
  const [selectedObject, setSelectedObject] = useState(null);
  const [components, setComponents] = useState({});
  const [staticComponents, setStaticComponents] = useState([]);
  const [staticComponentsTypes, setStaticComponentsTypes] = useState({});
  const [selectedStaticComponent, setSelectedStaticComponent] = useState(null);
  const [showCommandMenu, setShowCommandMenu] = useState(false);
  const [commandMenuType, setCommandMenuType] = useState("normal"); // "normal" or "static"
  const commandMenuRef = useRef();
  const [forceRenderKey, setForceRenderKey] = useState(0); // New state to force re-render

  const searchParams = useSearchParams();

  useEffect(() => {
    const loadInitialData = async () => {
      const path = searchParams.get('path');
      if (path) {
        try {
          let data = await readTextFile(path);
          data = data.replace(/\\/g, "");
          const parsedData = JSON.parse(data);

          const { components: registeredComponents, staticComponents: registeredStaticComponents } = registerComponents(parsedData);

          setComponents({ ...registeredComponents });
          setStaticComponentsTypes({ ...registeredStaticComponents });
          setHierarchy(parsedData.objects);
          setStaticComponents(parsedData.static_components);
        } catch (error) {
          console.error('Failed to load initial data:', error);
        }
      }
    };

    loadInitialData();
  }, [searchParams]);

  const toggleExpand = (object) => {
    object.expanded = !object.expanded;
    setHierarchy([...hierarchy]);
  };

  const getNestedListLength = (list) => {
    let totalLength = 0;

    for (const element of list) {
      if (Array.isArray(element)) {
        totalLength += getNestedListLength(element);
      } else {
        totalLength++;
      }
    }

    return totalLength;
  };

  const addObject = (parent) => {
    let id = getNestedListLength(hierarchy);
    const newObject = {
      id: id,
      name: `New GameObject`,
      expanded: true,
      components: [],
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

  const deleteComponent = (object, component) => {
    object.components = object.components.filter((comp) => comp.data.uuid !== component.data.uuid);
    setHierarchy([...hierarchy]);
  };

  const deleteCollider = (object, collider) => {
    object.colliders = object.colliders.filter((col) => col !== collider);
    setHierarchy([...hierarchy]);
  };

  const deleteStaticComponent = (component) => {
    setStaticComponents(staticComponents.filter((comp) => comp.data.uuid !== component.data.uuid));
    if (selectedStaticComponent && selectedStaticComponent.data.uuid === component.data.uuid) {
      setSelectedStaticComponent(null);
    }
  };

  const renameObject = (object, newName) => {
    object.name = newName;
    setHierarchy([...hierarchy]);
  };

  const handleComponentPropertyChange = (component, property, value) => {
    component.data[property] = value;
    setHierarchy([...hierarchy]);
  };

  const handleStaticComponentPropertyChange = (component, property, value) => {
    component.data[property] = value;
    setStaticComponents([...staticComponents]);
  };

  const renderPropertyInput = (component, property, type) => {
    if (component.data[property] == null) {
      return null;
    }

    if (type === "Object" || type === "Array") {
      return (
        <Card key={property} className="mb-2">
          <CardHeader>
            <CardTitle>{property}</CardTitle>
          </CardHeader>
          <CardContent>
            {Object.entries(component.data[property]).map(([key, val]) => (
              <div key={key} className="mb-2">
                <Label>{key}</Label>
                {typeof val === 'object' && val !== null ? (
                  renderPropertyInput({ data: val }, key, "Object")
                ) : (
                  <Input
                    value={val}
                    onChange={(e) => {
                      handleComponentPropertyChange(component, property, { ...component.data[property], [key]: e.target.value });
                    }}
                  />
                )}
              </div>
            ))}
          </CardContent>
        </Card>
      );
    }

    if (type === "Vector3") {
      return (
        <div className="mb-2">
          <Label>{property}</Label>
          <div className="flex space-x-2">
            {["X", "Y", "Z"].map((axis, index) => (
              <div key={axis} className="flex flex-col">
                <Label>{axis}</Label>
                <Input
                  type="number"
                  value={component.data[property][index]}
                  onChange={(e) => handleComponentPropertyChange(component, property, [
                    ...component.data[property].slice(0, index),
                    parseFloat(e.target.value),
                    ...component.data[property].slice(index + 1),
                  ])}
                  placeholder={axis}
                />
              </div>
            ))}
          </div>
        </div>
      );
    }

    if (type === "Float") {
      return (
        <>
          <Label>{property}</Label>
          <Input
            type="number"
            value={component.data[property]}
            onChange={(e) => handleComponentPropertyChange(component, property, parseFloat(e.target.value))}
          />
        </>
      );
    }

    if (type === "Boolean") {
      return (
        <>
          <Label>{property}</Label>
          <Checkbox
            checked={component.data[property]}
            onCheckedChange={(e) => handleComponentPropertyChange(component, property, e)}
          />
        </>
      );
    }

    return (
      <>
        <Label>{property}</Label>
        <Input
          value={component.data[property]}
          onChange={(e) => handleComponentPropertyChange(component, property, e.target.value)}
        />
      </>
    );
  };

  const renderRenderComponent = (component, property) => {
    const obj = component.data[property];
    const shape = Object.keys(obj)[0];
    const [sideLength, color] = obj[shape];

    return (
      <Card key={property} className="mb-2">
        <CardHeader>
          <CardTitle>{property}</CardTitle>
        </CardHeader>
        <CardContent>
          <Label>Shape</Label>
          <Input value={shape} readOnly />
          <Label>Side Length</Label>
          <Input
            type="number"
            value={sideLength}
            onChange={(e) => {
              const newSideLength = parseFloat(e.target.value);
              handleComponentPropertyChange(component, property, { ...obj, [shape]: [newSideLength, color] });
            }}
          />
          <Label>Color (RGB)</Label>
          <div className="flex space-x-2">
            {["R", "G", "B"].map((c, index) => (
              <div key={c} className="flex flex-col">
                <Label>{c}</Label>
                <Input
                  type="number"
                  value={color[index]}
                  onChange={(e) => {
                    const newColor = color.map((v, i) => (i === index ? parseFloat(e.target.value) : v));
                    handleComponentPropertyChange(component, property, { ...obj, [shape]: [sideLength, newColor] });
                  }}
                />
              </div>
            ))}
          </div>
        </CardContent>
      </Card>
    );
  };

  const renderBounds = (component, property) => {
    const bounds = component.data[property];
    const limits = bounds.limits;
    return (
      <Card key={property} className="mb-2">
        <CardHeader>
          <CardTitle>{property}</CardTitle>
        </CardHeader>
        <CardContent>
          {Object.entries(limits).map(([axis, value]) => (
            <div key={axis} className="mb-2">
              <Label>{axis.toUpperCase()}</Label>
              <Input
                type="number"
                value={value[axis]}
                onChange={(e) => {
                  const newLimits = {
                    ...limits,
                    [axis]: { [axis]: parseFloat(e.target.value) }
                  };
                  handleComponentPropertyChange(component, property, { ...bounds, limits: newLimits });
                }}
              />
            </div>
          ))}
        </CardContent>
      </Card>
    );
  };

  const renderCooldown = (component, property) => {
    const cooldown = component.data[property];
    return (
      <Card key={property} className="mb-2">
        <CardHeader>
          <CardTitle>{property}</CardTitle>
        </CardHeader>
        <CardContent>
          {Object.entries(cooldown).map(([key, value]) => (
            <div key={key} className="mb-2">
              <Label>{key}</Label>
              <Input
                type="number"
                value={value}
                onChange={(e) => {
                  const newCooldown = {
                    ...cooldown,
                    [key]: parseFloat(e.target.value)
                  };
                  handleComponentPropertyChange(component, property, newCooldown);
                }}
              />
            </div>
          ))}
        </CardContent>
      </Card>
    );
  };

  const renderComponents = (componentsList, object) =>
    componentsList.map((component) => (
      <Card key={component.data.uuid} className="mb-2">
        <CardHeader>
          <CardTitle>
            <div className="flex flex-row gap-5">
              <span className="pt-1 w-full">{component.id}</span>
              <Button variant="ghost" className="h-1/4" onClick={() => deleteComponent(object, component)}>
                <Trash size={16}/>
              </Button>
            </div>
          </CardTitle>
          <CardDescription>UUID: {component.data.uuid}</CardDescription>
        </CardHeader>
        <CardContent>
          {components[component.id] &&
            components[component.id].map((property) => {
              if (component.id === "RenderComponent" && property.name === "obj") {
                return renderRenderComponent(component, property.name);
              }
              if (component.id === "CharacterController2D" && property.name === "bounds") {
                return renderBounds(component, property.name);
              }
              if (property.name === "cooldown") {
                return renderCooldown(component, property.name);
              }
              return (
                <div key={property.name} className="mb-2">
                  {renderPropertyInput(component, property.name, property.type)}
                </div>
              );
            })}
        </CardContent>
      </Card>
    ));

  const renderColliders = (collidersList, object) =>
    (collidersList || []).map((collider, index) => (
      <Card key={index} className="mb-2">
        <CardHeader>
          <CardTitle>
            <div className="flex flex-row gap-5">
              <span className="pt-1 w-full">{Object.keys(collider.collider)[0]}</span>
              <Button variant="ghost" className="h-1/4" onClick={() => deleteCollider(object, collider)}>
                <Trash size={16}/>
              </Button>
            </div>
          </CardTitle>
        </CardHeader>
        <CardContent>
          {Object.entries(collider.collider[Object.keys(collider.collider)[0]]).map(([key, value]) => (
            <div key={key} className="mb-2">
              <Label>{key}</Label>
              {typeof value === 'object' && value !== null ? (
                renderPropertyInput({ data: value }, key, "Object")
              ) : (
                <Input value={JSON.stringify(value)} readOnly />
              )}
            </div>
          ))}
        </CardContent>
      </Card>
    ));

  const renderStaticComponentsList = (componentsList) =>
    componentsList.map((component) => (
      <div key={component.data.uuid} className="pl-2 flex items-center justify-between cursor-pointer hover:bg-muted">
        <span onClick={() => {
          setSelectedStaticComponent(component);
          setSelectedObject(null);
        }}>
          {component.id}
        </span>
        <Button variant="ghost" size="icon" onClick={() => deleteStaticComponent(component)}>
          <Trash size={16} />
        </Button>
      </div>
    ));

  const renderStaticComponent = (component) => (
    <>
        {staticComponentsTypes[component.id] &&
          staticComponentsTypes[component.id].map((property) => {
            if (property.name === "cooldown") {
              return renderCooldown(component, property.name);
            }
            return (
              <div key={property.name} className="mb-2">
                {renderPropertyInput(component, property.name, property.type)}
              </div>
            );
          })}
      </>
  );

  const addComponentToSelectedObject = (type) => {
    const newComponent = {
      id: type,
      data: Object.fromEntries(components[type].map((prop) => {
        if (prop.name === "bounds") {
          return [prop.name, { limits: { x: { x: 0 }, y: { y: 0 } } }];
        }
        if (prop.name === "obj") {
          return [prop.name, { Triangle: [0, [0, 0, 0]] }];
        }
        if (prop.name === "cooldown") {
          return [prop.name, { nanos: 0, secs: 0 }];
        }
        if (prop.name === "enemies") {
          return [prop.name, []];
        }
        if (prop.name === "last_spawn") {
          return [prop.name, null];
        }
        return [
          prop.name,
          prop.type === "Vector3"
            ? [0, 0, 0]
            : prop.type === "Boolean"
            ? false
            : prop.type === "Float"
            ? 0.0
            : prop.type === "Object" || prop.type === "Array"
            ? {}
            : "",
        ];
      })),
    };
    newComponent.data["uuid"] = v4();
    newComponent.data["state"] = { _state: null };

    selectedObject.components.push(newComponent);
    setHierarchy([...hierarchy]);
    setShowCommandMenu(false);
  };

  const addStaticComponent = (type) => {
    const data = Object.fromEntries(staticComponentsTypes[type].map((prop) => {
      if (prop.name === "cooldown") {
        return [prop.name, { nanos: 0, secs: 0 }];
      }
      if (prop.name === "enemies") {
        return [prop.name, []];
      }
      if (prop.name === "last_spawn") {
        return [prop.name, null];
      }
      return [
        prop.name,
        prop.type === "Vector3"
          ? [0, 0, 0]
          : prop.type === "Boolean"
          ? false
          : prop.type === "Float"
          ? 0.0
          : prop.type === "Object" || prop.type === "Array"
          ? {}
          : "",
      ];
    }));
    data["uuid"] = v4();
    const newComponent = {
      id: type,
      data: data,
    };
    setStaticComponents([...staticComponents, newComponent]);
    setShowCommandMenu(false);
  };

  const addCollider = (type) => {
    const newCollider = {
      collider: { [type]: { side_length: 0.1 } },
    };
    selectedObject.colliders.push(newCollider);
    setHierarchy([...hierarchy]);
    setShowCommandMenu(false);
  };

  const getAvailableComponents = () => {
    const existingComponentTypes = selectedObject ? selectedObject.components.map((component) => component.id) : [];
    return Object.keys(components).filter((type) => !existingComponentTypes.includes(type));
  };

  const getAvailableStaticComponents = () => {
    const existingComponentTypes = staticComponents.map((component) => component.id);
    return Object.keys(staticComponentsTypes).filter((type) => !existingComponentTypes.includes(type));
  };

  const getAvailableColliders = () => {
    return ["CubeCollider"]; // Add more collider types as needed
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

  const exportData = () => {
    const cleanData = (data) => {
      if (typeof data === "string" && data === "") {
        return null;
      }
      if (Array.isArray(data)) {
        return data.map(cleanData);
      }
      if (typeof data === "object" && data !== null) {
        return Object.fromEntries(
          Object.entries(data).map(([key, value]) => [key, cleanData(value)])
        );
      }
      return data;
    };

    const data = {
      objects: cleanData(hierarchy),
      static_components: cleanData(staticComponents),
      graphics: true,
    };
    console.log(JSON.stringify(data));
    return JSON.stringify(data);
  };

  const start_preview = () => {
    invoke("start_preview", { data: exportData() });
  };

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
            {renderStaticComponentsList(staticComponents)}
            <Button onClick={() => { setShowCommandMenu(true); setCommandMenuType("static"); }} className="mt-4 w-full">
              <Plus size={16} className="mr-2" />
              Add Static Component
            </Button>
          </div>
        </ScrollArea>
        <Separator />
        <div className="flex flex-row gap-5 h-1/12 p-5 items-center w-full justify-center">
          <Button onClick={() => { start_preview() }}>
            <Play size={16} />
          </Button>
          <Button onClick={() => { pause_preview() }}>
            <Pause size={16} />
          </Button>
          <Button onClick={() => { stop_preview() }}>
            <Square size={16} />
          </Button>
        </div>
      </div>
      <ScrollArea className="w-3/4 h-full">
        {selectedObject ? (
          <div className="p-4">
            <Card>
              <CardHeader>
                <CardTitle className="text-2xl">{selectedObject.name}</CardTitle>
                <CardDescription>ID: {selectedObject.id}</CardDescription>
              </CardHeader>
              <CardContent>
                {renderComponents(selectedObject.components, selectedObject)}
                <Separator className="my-4" />
                {renderColliders(selectedObject.colliders, selectedObject)}
                <Separator className="my-4" />
                <Button onClick={() => { setShowCommandMenu(true); setCommandMenuType("normal"); }} className="w-full">
                  <Plus size={16} className="mr-2" />
                  Add Component
                </Button>
                <Button onClick={() => { setShowCommandMenu(true); setCommandMenuType("collider"); }} className="w-full mt-2">
                  <Plus size={16} className="mr-2" />
                  Add Collider
                </Button>
              </CardContent>
            </Card>
          </div>
        ) : selectedStaticComponent ? (
          <div className="p-4">
            <Card>
              <CardHeader>
                <CardTitle>{selectedStaticComponent.id}</CardTitle>
                <CardDescription>UUID: {selectedStaticComponent.data.uuid}</CardDescription>
              </CardHeader>
              <CardContent>
                {renderStaticComponent(selectedStaticComponent)}
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
                    ) : commandMenuType === "static" ? (
                      getAvailableStaticComponents().length > 0 ? (
                        getAvailableStaticComponents().map((type) => (
                          <CommandItem key={type} onSelect={() => addStaticComponent(type)}>
                            {type}
                          </CommandItem>
                        ))
                      ) : (
                        <CommandEmpty>No static components left to add</CommandEmpty>
                      )
                    ) : commandMenuType === "collider" ? (
                      getAvailableColliders().length > 0 ? (
                        getAvailableColliders().map((type) => (
                          <CommandItem key={type} onSelect={() => addCollider(type)}>
                            {type}
                          </CommandItem>
                        ))
                      ) : (
                        <CommandEmpty>No colliders left to add</CommandEmpty>
                      )
                    ) : null}
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
