import { message } from "@tauri-apps/plugin-dialog";
import { Instance } from "../types/instances";
import { invoke } from "@tauri-apps/api/core";

export const getInstances = async (
  setInstances: (instances: Instance[]) => void
) => {
  try {
    const instances: Instance[] = await invoke("get_instances");
    setInstances(instances);
  } catch (error) {
    await message(`getInstances error: ${error}`, {
      title: "SynthLauncher Error",
      kind: "error",
    });
  }
};
