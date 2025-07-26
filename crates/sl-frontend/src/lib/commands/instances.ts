import { GameInfo, Instance, ModLoader } from '@/lib/types/instances';
import { invoke } from '@tauri-apps/api/core';
import { ToastError } from '@/components/toasters';

export const getInstances = async () => {
	try {
		return await invoke<Instance[]>('get_instances');
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const createInstance = async (
	name: string,
	version: string,
	modLoader: string
) => {
	try {
		await invoke('create_instance', {
			name: name,
			version: version,
			modLoader: modLoader,
		});
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const launchInstance = async (name: string) => {
	try {
		await invoke('launch_instance', { name: name });
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const getGameInfo = async (name: string, loader: ModLoader) => {
	try {
		return await invoke<GameInfo>('load_game_info', { name: name, loader: loader });
	} catch (error) {
		ToastError(`${error}`);
	}
};

export const killInstance = async (name: string) => {
	try {
		await invoke('kill_instance', { name: name });
	} catch (error) {
		ToastError(`${error}`)
	}
}
