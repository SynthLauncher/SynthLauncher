"use client";

import { useState } from "react";
import Image from "next/image";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "../ui/dialog";
import { Button } from "../ui/button";
import { Plus, Camera, Upload, Check, ChevronsUpDown } from "lucide-react";
import { Input } from "../ui/input";

import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { cn } from "@/lib/utils";
import React from "react";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList } from "../ui/command";

const minecraftVersions = [
  {
    value: "1.20.4",
    label: "Minecraft 1.20.4",
  },
  {
    value: "1.19.2",
    label: "Minecraft 1.19.2",
  },
  {
    value: "1.18.2",
    label: "Minecraft 1.18.2",
  },
  {
    value: "1.17.1",
    label: "Minecraft 1.17.1",
  },
  {
    value: "1.16.5",
    label: "Minecraft 1.16.5",
  },
  {
    value: "1.12.2",
    label: "Minecraft 1.12.2",
  },
  {
    value: "1.8.9",
    label: "Minecraft 1.8.9",
  },
];

export function ComboboxDemo() {
  const [open, setOpen] = React.useState(false);
  const [value, setValue] = React.useState("");

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between border-2 border-neutral-700/50 bg-transparent text-white hover:bg-neutral-700/50 hover:text-white h-12"
        >
          {value
            ? minecraftVersions.find((minecraftVersion) => minecraftVersion.value === value)?.label
            : "Select Minecraft Version..."}
          <ChevronsUpDown className="opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-full p-0 bg-[#141414] border-2 transition duration-300 border-neutral-700/50 text-white">
        <Command className="bg-transparent text-white">
          <CommandInput
            placeholder="Search Minecraft Version..."
            className="text-neutral-700/50 border-b-2 border-neutral-700/50"
          />
          <CommandList className="text-white">
            <CommandEmpty className="text-neutral-400 m-2">No Version found.</CommandEmpty>
            <CommandGroup>
              {minecraftVersions.map((minecraftVersion) => (
                <CommandItem
                  key={minecraftVersion.value}
                  value={minecraftVersion.value}
                  onSelect={(currentValue) => {
                    setValue(currentValue === value ? "" : currentValue);
                    setOpen(false);
                  }}
                  className="text-white hover:bg-neutral-800"
                >
                  {minecraftVersion.label}
                  <Check
                    className={cn(
                      "ml-auto",
                      value === minecraftVersion.value ? "opacity-100" : "opacity-0"
                    )}
                  />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}

export const Sidebar = () => {
  return (
    <div className="bg-neutral-900 h-screen w-[95px] flex flex-col items-center p-1 border-r-2 border-neutral-700/50">
      <Dialog>
        <DialogTrigger className="focus:outline-none">
          <Image
            className="rounded-full object-cover mt-5 ml-[2px] mb-5"
            src="/icon1.png"
            alt="icon"
            width={50}
            height={50}
          />
          <div className="w-[55px] bg-neutral-700/50 h-[2px]" />
        </DialogTrigger>
        <DialogContent className="bg-[#141414] border-2 border-neutral-700/50 text-white">
          <DialogHeader className="bg-[#141414]">
            <DialogTitle className="text-white">Are you absolutely sure?</DialogTitle>
            <DialogDescription className="text-white">
              This action cannot be undone. This will permanently delete your account and remove
              your data from our servers.
            </DialogDescription>
          </DialogHeader>
          <Button className="bg-white text-black hover:bg-neutral-100">
            Yes, Delete my account
          </Button>
        </DialogContent>
      </Dialog>
      <Dialog>
        <DialogTrigger className="focus:outline-none">
          <Plus
            className="mt-5 text-white bg-neutral-800 rounded-full p-2 border-2 border-neutral-700/50"
            width={50}
            height={50}
          />
        </DialogTrigger>
        <DialogContent className="bg-[#141414] border-2 border-neutral-700/50 text-white">
          <DialogHeader className="bg-[#141414]">
            <DialogTitle className="text-white">Add a new installation</DialogTitle>
            <DialogDescription className="text-white">
              Add a new Minecraft installation.
            </DialogDescription>
          </DialogHeader>

          {/* Image Picker */}
          {/* <div className="flex flex-col items-center justify-center my-4 p-6 border-2 border-dashed border-neutral-600 rounded-lg">
            <div className="p-4 rounded-full bg-neutral-800">
              <Camera size={32} className="text-neutral-300" />
            </div>
            <p className="font-medium mt-3">Choose an image</p>
            <p className="text-sm text-neutral-400 mt-1">PNG, JPG or GIF</p>
            <Button variant="outline" className="mt-4 bg-white text-black hover:bg-neutral-100">
              <Upload size={16} />
              Browse files
            </Button>
          </div> */}
            <Input
            placeholder="Name of installation..."
            className="border-2 border-neutral-700/50 h-12"
          />
          <ComboboxDemo />
          <Button className="bg-white text-black hover:bg-neutral-100">Add the installation</Button>
        </DialogContent>
      </Dialog>
    </div>
  );
};
