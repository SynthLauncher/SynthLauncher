import { useCallback } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import { Maximize, Minus, X } from "lucide-react";

function App() {
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
    <div className="bg-[#282c32] h-screen flex-col">
      <nav
        className="w-full flex h-15 justify-end items-center p-1"
        data-tauri-drag-region
      >
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
      </nav>
      <div className="flex">
        <div className="w-20"></div>
        <div className="bg-[#181a1e] border-[#4e4c57] border-2 w-full min-h-screen rounded-tl-2xl"></div>
      </div>
    </div>
  );
}

export default App;
