import { getCurrentWindow } from '@tauri-apps/api/window';

export const handleWindowMaximize = async () => {
	const appWindow = getCurrentWindow();

	const isMaximized = await appWindow.isMaximized();

	if (isMaximized) {
		await appWindow.unmaximize();
	} else {
		await appWindow.maximize();
	}
};

export const handleWindowMinimize = async () => {
	const appWindow = getCurrentWindow();
	await appWindow.minimize();
};

export const handleWindowClose = async () => {
	const appWindow = getCurrentWindow();
	await appWindow.close();
};
