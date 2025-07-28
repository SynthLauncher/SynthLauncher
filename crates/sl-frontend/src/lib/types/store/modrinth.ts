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
	total_hits: number
}


export interface ModrinthProjectFileHashes {
    sha1: string,
    sha512: string,
}

export interface ModrinthProjectFile {
    hashes: ModrinthProjectFileHashes,
    url: string,
    filename: string,
    primary: boolean,
    size: number,
    file_type?: string,
}

export interface ModrinthProjectVersion {
    id: string,
    project_id: string,
    name: string,
    game_versions: string[],
    loaders: string[],
    version_number: string,
    downloads: number,
    version_type: string,
    files: ModrinthProjectFile[],
}