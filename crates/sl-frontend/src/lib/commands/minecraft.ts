import { invoke } from '@tauri-apps/api/core';
import { message } from '@tauri-apps/plugin-dialog';

export const getMinecraftVersions = async (
	setMinecraftVersions: (versions: string[]) => void
) => {
	try {
		let minecraftVersions: string[] = await invoke('get_minecraft_versions');
		setMinecraftVersions(minecraftVersions);
	} catch (error) {
		await message(`getMinecraftVersions error: ${error}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
