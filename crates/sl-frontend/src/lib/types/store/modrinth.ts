enum SupportRequirement {
	Required,
	Optional,
	Unsupported,
	Unknown,
}

enum MonetizationStatus {
	Monetized,
	Demonetized,
	ForceDemonetized,
}

interface Hit {
	slug: string;
	title: string;
	description: string;
	categories: string[];
	client_side: SupportRequirement;
	server_side: SupportRequirement;
	project_type: string;
	downloads: number;
	icon_url?: string;
	color?: number;
	thread_id?: string;
	monetization_status?: MonetizationStatus;
	project_id: string;
	author: string;
	display_categories: string[];
	versions: string[];
	follows: number;
	date_created: string;
	date_modified: string;
	latest_version: string;
	license: string;
	gallery: string[];
	featured_gallery?: string;
}

export interface SearchResult {
	hits: Hit[];
}
