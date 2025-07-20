import { invoke } from '@tauri-apps/api/core';
import { message } from '@tauri-apps/plugin-dialog';

export const getMinecraftVersions = async () => {
	try {
		let minecraftVersions = await invoke<string[]>('get_minecraft_versions');
		return minecraftVersions ?? [];
	} catch (error) {
		await message(`getMinecraftVersions error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});

		return [];
	}
};

export async function minecraftLogin(
	username: string,
	password: string,
	twofa?: string
): Promise<any> {
	try {
		const result = await invoke('minecraft_login', {
			username,
			password,
			twofa,
		});
		return result;
	} catch (err: any) {
		return { error: 'network', errorMessage: err?.toString() };
	}
}
