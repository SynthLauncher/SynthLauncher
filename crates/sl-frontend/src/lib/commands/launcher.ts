import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";

export const openSynthLauncherFolder = async () => {
  try {
    await invoke("open_synthlauncher_folder");
  } catch (error) {
    await message(`openSynthLauncherFolder error: ${error}`, {
      title: "SynthLauncher Error",
      kind: "error",
    });
  }
};
