import { getCurrentWindow } from "@tauri-apps/api/window";

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
