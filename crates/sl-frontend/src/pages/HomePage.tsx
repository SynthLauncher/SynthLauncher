import { Progress } from "@/components/ui/progress";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import React, { useEffect, useState } from "react";
// import InstanceCard from "../components/InstanceCard";

export const HomePage: React.FC = () => {
  const [state, setState] = useState<obj>();
  
  interface obj {
    name: string,
    downloaded: number,
    total?: number
  }

  useEffect(() => {
    const unlisten = listen('download-progress', (event) => {
      setState(event.payload as obj);
    })

      return () => {
    unlisten.then((fn) => fn());
  };
  }, [])

  const test_progress = async () => {
    await invoke("test_progress");
  }
  
  return (
    <div className="p-6 w-full overflow-auto pb-12">
      <div className="mb-8 flex flex-col gap-2">
        <Progress value={state?.downloaded} className="w-[60%] bg-white" />

        <button className="bg-white px-4 py-2 w-2xs" onClick={test_progress}>
          Click
        </button>
      </div>
    </div>
  );
};
