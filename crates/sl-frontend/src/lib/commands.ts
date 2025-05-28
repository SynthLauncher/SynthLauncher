import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { message } from "@tauri-apps/plugin-dialog";
import { Instance } from "./types";

// TODO: Add unmaximizing!!!
export const handleWinndowMaximize = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.maximize();
};

export const handleWindowMinimize = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.minimize();
};

export const handleWindowClose = async () => {
  const appWindow = getCurrentWindow();
  await appWindow.close();
};

export const getPlayerUsername = async (
  setUsername: (username: string) => void
) => {
  try {
    const username: string = await invoke("get_username");
    setUsername(username);
  } catch (error) {
    await message(`getPlayerUsername error: ${error}`, {
      title: "SynthLauncher",
      kind: "error",
    });
  }
};

export const launchInstance = async (name: string) => {
  try {
    await invoke("launch", { name: name });
  } catch (error) {
    await message(`Launching error: ${error}`, {
      title: "SynthLauncher",
      kind: "error",
    });
  }
};

export const getInstances = async (setInstances: (instances: Instance[]) => void) => {
  try {
    const instances: Instance[] = await invoke("get_installations");
    setInstances(instances);
  } catch (error) {
    await message(`Failed to get instances: ${error}`, {
      title: "SynthLauncher",
      kind: "error",
    });
  }
}

export const createInstance = async (name: string, version: string) => {
  await invoke("create_installation", { name: name, version: version });
}

export const removeInstance = async (name: string) => {
  await invoke("remove_installation", { name: name });
}

export const loadInstances = async () => {
  await invoke("load_all_installations");
}