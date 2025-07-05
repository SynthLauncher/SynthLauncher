import { message } from '@tauri-apps/plugin-dialog';
import { GameInfo, Instance } from '../types/instances';
import { invoke } from '@tauri-apps/api/core';

export const getInstances = async () => {
	try {
		const instances = await invoke<Instance[]>('get_instances');
		return instances ?? []
	} catch (error) {
		await message(`Failed to get instances!\n getInstances error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});

		return [];
	}
};

export const createInstance = async (name: string, version: string, modLoader: string) => {
	try {
		await invoke('create_instance', { name: name, version: version, modLoader: modLoader });
	} catch (error) {
		await message(
			`Creating the instance failed!\n createInstance error: ${error}`,
			{
				title: 'SynthLauncher Error',
				kind: 'error',
			}
		);
	}
};

export const launchInstance = async (name: string) => {
	try {
		await invoke('launch_instance', { name: name });
	} catch (error) {
		await message(
			`Launching the instance failed!\n launchInstance error: ${error}`,
			{
				title: 'SynthLauncher Error',
				kind: 'error',
			}
		);
	}
};

export const GetGameInfo = async (name: string) => {
	try {
		let gameInfo = await invoke<GameInfo>('load_game_info', { name: name });
		return gameInfo
	} catch (error) {
		await message(
			`Launching the instance failed!\n launchInstance error: ${error}`,
			{
				title: 'SynthLauncher Error',
				kind: 'error',
			}
		);
	}
}
