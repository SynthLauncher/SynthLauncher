import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export function useRunningInstances() {
  const [runningInstances, setRunningInstances] = useState<Set<string>>(new Set());

  useEffect(() => {
    let mounted = true;

    async function fetchInstances() {
      const list = await invoke<string[]>("get_running_instances");
      if (!mounted) return;
      setRunningInstances(new Set(list));
    }

    fetchInstances();

    const unlistenPromise = listen("running_instances_updates", async () => {
      await fetchInstances();
    });

    return () => {
      mounted = false;
      unlistenPromise.then(unlisten => unlisten());
    };
  }, []);

  return runningInstances;
}