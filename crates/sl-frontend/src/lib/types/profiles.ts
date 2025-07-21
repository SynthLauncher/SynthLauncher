export interface PlayerProfile {
	data: {
		name: string;
		id: string;
	};
	access_token: string;
	premium: boolean;
}

export interface PlayerProfiles {
	current_profile_index: number,
	profiles: PlayerProfile[]
}
