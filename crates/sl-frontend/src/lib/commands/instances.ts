import { GameInfo, Instance, ModLoader } from '@/lib/types/instances';
import { invoke } from '@tauri-apps/api/core';
import { ToastError, ToastInfo, ToastSuccess } from '@/components/toasters';

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
	modLoader: string,
	icon: string
) => {
	try {
		await invoke('create_instance', {
			name: name,
			version: version,
			modLoader: modLoader,
			icon: icon
		});
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const launchInstance = async (name: string) => {
	try {
		ToastInfo("Instance has begun launching...")
		await invoke('launch_instance', { name: name });
		ToastSuccess("Instance has been closed successfully.")
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const killInstance = async (name: string) => {
	try {
		await invoke('kill_instance', { name: name });
	} catch (error) {
		ToastError(`${error}`)
	}
}
