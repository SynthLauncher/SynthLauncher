import { useCallback } from "react";
import "./App.css";
import { Button } from "./components/ui/button";
import { VscChromeMinimize } from "react-icons/vsc";
import { IoClose } from "react-icons/io5";
import { X } from "lucide-react";

function App() {
  const handleMinimize = useCallback(async () => {
    const { getCurrentWindow } = await import("@tauri-apps/api/window");
    const appWindow = getCurrentWindow();
    await appWindow.close();
  }, []);

  return (
    <div className="bg-[#2c323b] min-h-screen">
      <nav className="w-full flex p-1">
        <Button
          onClick={handleMinimize}
          variant="ghost"
          className="group hover:bg-red-400 hover:scale-105 rounded-full p-0 flex items-center justify-center w-16 h-16 transition-all duration-200"
        >
          <X className="text-white group-hover:text-black" size={128} />
        </Button>
      </nav>
    </div>
  );
}

export default App;
