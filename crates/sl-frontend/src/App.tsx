import { useCallback } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import { Maximize, Minus, X } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { Input } from "./components/ui/input";

function App() {
  async function launch() {
    await invoke("launch");
  }

  const handleMinimize = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();
    await appWindow.minimize();
  }, []);

  const handleMaximize = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();
    const isMax = await appWindow.isMaximized();
    if (isMax) {
      await appWindow.unmaximize();
    } else {
      await appWindow.maximize();
    }
  }, []);

  const handleClose = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();
    await appWindow.close();
  }, []);

  return (
    <div className="bg-[#282c32] h-screen flex flex-col overflow-hidden">
      <nav
        className="w-full flex h-15 justify-between items-center p-1"
        data-tauri-drag-region
      >
        <div className="flex pl-2">
          <h1 className="text-2xl text-white font-semibold font-sans">
            synthlauncher
          </h1>
        </div>

        <div className="flex">
          <Button
            variant="ghost"
            onClick={handleMinimize}
            className="group hover:bg-slate-500/20 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
          >
            <Minus className="text-white transition-all duration-200 transform group-hover:scale-105" />
          </Button>

          <Button
            variant="ghost"
            onClick={handleMaximize}
            className="group hover:bg-slate-500/20 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
          >
            <Maximize className="text-white transition-all duration-200 transform group-hover:scale-105" />
          </Button>
          <Button
            variant="ghost"
            onClick={handleClose}
            className="group hover:bg-red-400 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
          >
            <X className="text-white group-hover:text-black transition-all duration-200 transform group-hover:scale-105" />
          </Button>
        </div>
      </nav>

      <div className="flex flex-1 overflow-hidden">
        <div className="w-20"></div>

        <div className="bg-[#181a1e] border-[#4e4c57] border-l-2 border-t-2 w-full h-full  rounded-tl-2xl p-4">
          <div className="min-h-screen">
            <Button className="bg-sky-400 text-black rounded-lg" onClick={launch}>
              Create an installation
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
