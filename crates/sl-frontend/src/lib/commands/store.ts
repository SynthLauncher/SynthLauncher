import { message } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import { ModrinthSearchResult } from '@/lib/types/store/modrinth';
import { CurseforgeSearchResult } from '@/lib/types/store/curseforge';

export const getModrinthStoreSearch = async (
	query: string,
	projectType: string,
	page: number,
) => {
	try {
		const search = await invoke<ModrinthSearchResult>('search_modrinth_store', {
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

export const getCurseforgeStoreSearch = async (
	query: string,
	class_id: number,
	offset: number,
) => {
	try {
		const search = await invoke<CurseforgeSearchResult>('search_curseforge_store', {
			query: query,
			classId: class_id,
			offset: offset,
		});

		return search
	} catch (err) {
		await message(`getStoreSearch error: ${err}`, {
			title: 'SynthLauncher Error',
			kind: 'error',
		});
	}
};
