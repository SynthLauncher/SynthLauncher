import { message } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { SearchResult } from '../types/store/modrinth';

export const getModrinthStoreSearchResult = async (
	query: string,
	projectType: string,
	page: number,
) => {
	try {
		const search = await invoke<SearchResult>('search_modrinth_store', {
			query: query,
			projectType: projectType,
			page: page
		});

		return search
	} catch (err) {
		await message(`getStoreSearch error: ${err}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
