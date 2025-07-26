import { invoke } from '@tauri-apps/api/core';
import { ToastError } from '@/components/toasters';
import { PlayerAccounts } from '../types/account';

export const getAccounts = async () => {
	try {
		return await invoke<PlayerAccounts>('accounts_get');
	} catch (e) {
		ToastError(`${e}`);
	}
}

export const setCurrentAccount = async (name: string) => {
	try {
		await invoke('accounts_set_current', { name: name });
	} catch (e) {
		ToastError(`${e}`);
	}
}

export const createOfflineAccount = async (name: string) => {
	try {
		await invoke('accounts_create_offline', { name: name })
	} catch (e) {
		ToastError(`${e}`);
	}
}

export const removeAccount = async (name: string) => {
	try {
		await invoke('accounts_remove', { name: name })
	} catch (e) {
		ToastError(`${e}`)
	}
}