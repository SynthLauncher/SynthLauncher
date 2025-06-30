import { invoke } from '@tauri-apps/api/core';
import { message } from '@tauri-apps/plugin-dialog';

export const openSynthLauncherFolder = async () => {
	try {
		await invoke('open_synthlauncher_folder');
	} catch (error) {
		await message(`openSynthLauncherFolder error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};

export const getSynthLauncherAddons = async (
	setAddons: (addons: string[]) => void
) => {
	try {
		let addons = await invoke<string[]>('get_synthlauncher_addons');
		setAddons(addons);
	} catch (error) {
		await message(`getSynthLauncherAddons error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
}