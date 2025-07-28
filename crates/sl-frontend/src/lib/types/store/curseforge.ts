interface CurseforgeProjectAsset {
	url?: string;
}

interface CurseforgeProjectAuthor {
	name: string;
}

export interface CurseforgeFile {
	fileName?: string;
	downloadUrl?: string;
	gameVersions: string[];
}

export interface CurseforgeProject {
	id: number;
	gameId: number;
	name: string;
	slug: string;
	summary: string;
	downloadCount: number;
	logo: CurseforgeProjectAsset;
	screenshots: CurseforgeProjectAsset[];
	authors: CurseforgeProjectAuthor[];
	latestFiles: CurseforgeFile[];
}

export interface CurseforgePagination {
	totalCount: number
}

export interface CurseforgeSearchResult {
	data: CurseforgeProject[];
	pagination: CurseforgePagination
}
