import { Maximize, Minus, X } from "lucide-react";
import { Button } from "../ui/button";
import {
  handleWindowClose,
  handleWindowMinimize,
  handleWinndowMaximize,
} from "@/lib/commands";

export const Navbar = () => {
  return (
    <nav
      className="bg-gray-900 w-full flex h-15 justify-end items-center p-1"
      data-tauri-drag-region
    >
      <div className="flex">
        <Button
          variant="ghost"
          onClick={handleWindowMinimize}
          className="group hover:bg-slate-500/20 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
        >
          <Minus className="text-white transition-all duration-200 transform group-hover:scale-105" />
        </Button>

        <Button
          variant="ghost"
          onClick={handleWinndowMaximize}
          className="group hover:bg-slate-500/20 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
        >
          <Maximize className="text-white transition-all duration-200 transform group-hover:scale-105" />
        </Button>
        <Button
          variant="ghost"
          onClick={handleWindowClose}
          className="group hover:bg-red-400 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
        >
          <X className="text-white group-hover:text-black transition-all duration-200 transform group-hover:scale-105" />
        </Button>
      </div>
    </nav>
  );
};
