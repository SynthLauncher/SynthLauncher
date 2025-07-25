import { invoke } from '@tauri-apps/api/core';
import { ModrinthProjectVersion, ModrinthSearchResult } from '@/lib/types/store/modrinth';
import { CurseforgeSearchResult } from '@/lib/types/store/curseforge';
import { ToastError } from '@/components/toasters';

export const getModrinthStoreSearch = async (
	query: string,
	projectType: string,
	page: number
) => {
	try {
		const search = await invoke<ModrinthSearchResult>('search_modrinth_store', {
			query: query,
			projectType: projectType,
			page: page,
		});

		return search;
	} catch (error) {
		ToastError(`${error}`)
	}
};

export const getCurseforgeStoreSearch = async (
	query: string,
	class_id: number,
	offset: number
) => {
	try {
		const search = await invoke<CurseforgeSearchResult>(
			'search_curseforge_store',
			{
				query: query,
				classId: class_id,
				offset: offset,
			}
		);

		return search;
	} catch (err) {
		ToastError(`${err}`)
	}
};

export const getModrinthProjectVersions = async (
	slug: string,
	game_version: string,
	loader: string
) => {
	try {
		const search = await invoke<ModrinthProjectVersion[]>(
			'get_modrinth_project_versions',
			{
				slug: slug,
				gameVersion: game_version,
				loader: loader
			}
		);

		return search;
	} catch (err) {
		ToastError(`${err}`)
	}
};

export const installModrinthProject = async (
	slug: string,
	version: string,
	instance_name: string,
	project_type: string
) => {
	try {
		await invoke('install_modrinth_project', {
			slug: slug,
			version: version,
			instanceName: instance_name,
			projectType: project_type
		})
	} catch (error) {
		ToastError(`${error}`)
	}
}