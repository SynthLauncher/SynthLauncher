import { Maximize, Minus, X } from "lucide-react";
import { Button } from "../ui/button";
import {
  handleWindowClose,
  handleWindowMinimize,
  handleWinndowMaximize,
} from "@/lib/commands/window";

export const Navbar = () => {
  return (
    <nav
      className="bg-[#1B1D21] w-full flex h-[3.3rem] justify-end items-center p-1"
      data-tauri-drag-region
    >
      <div className="flex gap-[0.1rem]">
        <Button
          variant="ghost"
          onClick={handleWindowMinimize}
          className="group hover:bg-[#89C733] rounded-full p-0 flex items-center justify-center w-11 h-11 transition-all duration-300"
        >
          <Minus className="text-white group-hover:text-black transition-all duration-300 transform group-hover:scale-105" />
        </Button>
{/*hover:bg-slate-500/20 */}
        <Button
          variant="ghost"
          onClick={handleWinndowMaximize}
          className="group hover:bg-[#FCB335] rounded-full p-0 flex items-center justify-center w-11 h-11 transition-all duration-300"
        >
          <Maximize className="text-white group-hover:text-black transition-all duration-300 transform group-hover:scale-105" />
        </Button>
        <Button
          variant="ghost"
          onClick={handleWindowClose}
          className="group hover:bg-red-400 rounded-full p-0 flex items-center justify-center w-11 h-11 transition-all duration-300"
        >
          <X className="text-white group-hover:text-black transition-all duration-300 transform group-hover:scale-105" />
        </Button>
      </div>
    </nav>
  );
};
