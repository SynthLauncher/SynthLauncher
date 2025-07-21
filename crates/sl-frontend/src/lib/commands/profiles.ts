import { invoke } from '@tauri-apps/api/core';
import { PlayerProfiles } from '@/lib/types/profiles';
import { ToastError } from '@/components/toasters';

export const getAllProfiles = async () => {
	try {
		return await invoke<PlayerProfiles>('get_profiles');
	} catch (error) {
		ToastError(`${error}`);
	}
}

export const setCurrentProfile = async (index: number) => {
	try {
		await invoke('set_current_profile', { index: index });
	} catch (error) {
		ToastError(`${error}`);
	}
}

export const createOfflineAccount = async (name: string) => {
	try {
		await invoke('create_offline_profile', { name: name })
	} catch (error) {
		ToastError(`${error}`);
	}
}