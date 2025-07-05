import { invoke } from '@tauri-apps/api/core';
import { PlayerProfile } from '../types/profiles';
import { message } from '@tauri-apps/plugin-dialog';

export const getCurrentProfile = async () => {
	try {
		const profile: PlayerProfile = await invoke('get_current_profile');
		return profile;
	} catch (err) {
		await message(`getCurrentProfile error: ${err}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
