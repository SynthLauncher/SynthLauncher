import { ToastError } from '@/components/toasters';
import { invoke } from '@tauri-apps/api/core';

export const getMinecraftVersions = async () => {
	try {
		return await invoke<string[]>('get_minecraft_versions');
	} catch (error) {
		ToastError(`${error}`);
	}
};
