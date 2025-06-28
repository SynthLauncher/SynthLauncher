import { invoke } from '@tauri-apps/api/core';
import { message } from '@tauri-apps/plugin-dialog';
import { Theme } from '../types/themes/theme';

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

export const getThemes = async (setThemes: (themes: Theme[]) => void) => {
	try {
		let themes: Theme[] = await invoke('get_synthlauncher_themes');
		setThemes(themes);
	} catch (error) {
		await message(`getThemes error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
