import { message } from '@tauri-apps/plugin-dialog';
import { Instance } from '../types/instances';
import { invoke } from '@tauri-apps/api/core';

export const getInstances = async (
	setInstances: (instances: Instance[]) => void
) => {
	try {
		const instances = await invoke<Instance[]>('get_instances');
		setInstances(instances);
	} catch (error) {
		await message(`Failed to get instances!\n getInstances error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};

export const createInstance = async (name: string, version: string) => {
	try {
		await invoke('create_instance', { name: name, version: version });
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
