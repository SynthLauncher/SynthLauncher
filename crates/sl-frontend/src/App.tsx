import { useCallback } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import { Maximize, Minus, X } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./components/sidebar";
import HomePage from "./pages/HomePage";
import ProfileSidebar from "./components/profilesidebar";

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
    <div className="bg-[#0b0b22] h-screen flex-col overflow-hidden">
      <nav
        className="bg-gray-900 w-full flex h-15 justify-end items-center p-1"
        data-tauri-drag-region
      >
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

      <div className="flex overflow-hidden h-full">
        <Sidebar setActiveTab={() => {}} activeTab="home" />

        <div className="flex w-full border-l-2 border-t-2 border-gray-800 rounded-tl-2xl">
          <HomePage />

          <ProfileSidebar />
        </div>
      </div>
    </div>
  );
}

export default App;
