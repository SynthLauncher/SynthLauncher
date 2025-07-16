export interface ModrinthSearchHit {
	slug: string;
	title: string;
	description: string;
	project_type: string;
	downloads: number;
	icon_url?: string;
	project_id: string;
	author: string;
	versions: string[];
	follows: number;
	latest_version: string;
	gallery: string[];
}

export interface ModrinthSearchResult {
	hits: ModrinthSearchHit[];
}
