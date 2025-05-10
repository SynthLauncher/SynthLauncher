import { useCallback } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import { X } from "lucide-react";

function App() {
  const handleMinimize = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();
    await appWindow.close();
  }, []);

  return (
    <div className="bg-[#2c323b] min-h-screen">
      <nav className="w-full flex p-1 bg-slate-700 justify-end" data-tauri-drag-region>
        <Button
          variant="ghost"
          onClick={handleMinimize}
          className="group hover:bg-red-400 rounded-full p-0 flex items-center justify-center w-14 h-14 transition-all duration-200"
        >
          <X className="text-white group-hover:text-black transition-all duration-200 transform group-hover:scale-105" />
        </Button>
      </nav>
    </div>
  );
}

export default App;