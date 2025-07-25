import { invoke } from '@tauri-apps/api/core';
import { ToastError } from '@/components/toasters';
import { PlayerAccounts } from '../types/account';

export const getAccounts = async () => {
	try {
		return await invoke<PlayerAccounts>('get_accounts');
	} catch (error) {
		ToastError(`${error}`);
	}
}

export const setCurrentAccount = async (name: string) => {
	try {
		await invoke('set_current_account', { name: name });
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