import { message } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { Search } from '../types/store/modrinth';

export const getStoreSearch = async (
	query: string,
	category: string,
	setSearch: (search: Search) => void
) => {
	try {
		const search = await invoke<Search>('search_store', {
			query: query,
			category: category,
		});

		setSearch(search);
	} catch (err) {
		await message(`getStoreSearch error: ${err}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
