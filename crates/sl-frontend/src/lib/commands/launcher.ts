import { ToastError } from '@/components/toasters';
import { invoke } from '@tauri-apps/api/core';

export const openFolder = async (folder_path: string) => {
	try {
		await invoke('open_folder', { folderPath: folder_path })		
	} catch (error) {
		ToastError(`${error}`)
	}
}

export const openSynthLauncherFolder = async () => {
	try {
		await invoke('open_synthlauncher_folder');
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const openInstanceFolder = async (name: string) => {
	try {
		await invoke('open_instance_folder', { name: name });
	} catch (error) {
		ToastError(`${error}`)
	}
};
