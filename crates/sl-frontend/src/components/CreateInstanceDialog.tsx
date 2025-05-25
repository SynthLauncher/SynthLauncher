import React, { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../components/ui/dialog";

interface CreateInstanceDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreate: (name: string, version: string) => void;
}

const CreateInstanceDialog: React.FC<CreateInstanceDialogProps> = ({
  open,
  onOpenChange,
  onCreate,
}) => {
  const [name, setName] = useState("");
  const [version, setVersion] = useState("1.21.5");

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px] bg-gray-900 border-gray-700 border-2">
        <DialogHeader>
          <DialogTitle className="text-white">Create New Instance</DialogTitle>
          <DialogDescription className="text-gray-300">
            Configure your new Minecraft instance.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <label htmlFor="name" className="text-sm font-medium text-gray-200">
              Instance Name
            </label>
            <input
              id="name"
              className="flex h-9 w-full rounded-md border border-gray-700 bg-gray-800 px-3 py-1 text-sm text-gray-200 focus:outline-none focus:ring-2 focus:ring-emerald-500"
              placeholder="My New Instance"
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </div>

          <div className="grid gap-2">
            <label
              htmlFor="version"
              className="text-sm font-medium text-gray-200"
            >
              Minecraft Version
            </label>
            <select
              id="version"
              className="flex h-9 w-full rounded-md border border-gray-700 bg-gray-800 px-3 py-1 text-sm text-gray-200 focus:outline-none focus:ring-2 focus:ring-emerald-500"
              onChange={(e) => setVersion(e.target.value)}
              value={version}
            >
              <option value="1.21.5">1.21.5</option>
              <option value="1.19.4">1.19.4</option>
              <option value="1.18.2">1.18.2</option>
            </select>
          </div>

          <div className="grid gap-2">
            <label
              htmlFor="modloader"
              className="text-sm font-medium text-gray-200"
            >
              Mod Loader
            </label>
            <select
              id="modloader"
              className="flex h-9 w-full rounded-md border border-gray-700 bg-gray-800 px-3 py-1 text-sm text-gray-200 focus:outline-none focus:ring-2 focus:ring-emerald-500"
            >
              <option value="forge">Forge</option>
              <option value="fabric">Fabric</option>
              <option value="vanilla">Vanilla</option>
            </select>
          </div>

          <div className="grid gap-2">
            <label htmlFor="icon" className="text-sm font-medium text-gray-200">
              Instance Icon
            </label>
            <div className="flex items-center gap-4">
              <div className="w-16 h-16 bg-gray-700 rounded-lg"></div>
              <button className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-gray-200 rounded-lg transition-colors">
                Choose Icon
              </button>
            </div>
          </div>
        </div>
        <DialogFooter>
          <button
            onClick={() => onOpenChange(false)}
            className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-gray-200 rounded-lg transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={() => {
              onOpenChange(false);
              onCreate(name, version);
            }}
            className="px-4 py-2 bg-emerald-600 hover:bg-emerald-500 text-white rounded-lg transition-colors"
          >
            Create Instance
          </button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default CreateInstanceDialog;
