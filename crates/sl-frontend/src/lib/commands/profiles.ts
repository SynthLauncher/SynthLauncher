import { invoke } from '@tauri-apps/api/core';
import { PlayerProfile } from '../types/profiles';
import { message } from '@tauri-apps/plugin-dialog';

export const getCurrentProfile = async (
	setProfile: (profile: PlayerProfile) => void
) => {
	try {
		const profile: PlayerProfile = await invoke('get_current_profile');

		setProfile(profile);
	} catch (err) {
		await message(`getCurrentProfile error: ${err}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
