enum VersionType {
	OldBeta,
	OldAlpha,
	Release,
	Snapshot,
}

enum ModLoader {
	Vanilla,
	Fabric,
	Quilt,
	Forge,
	Neoforge,
}

interface InstanceGameInfo {
	id: string;
	releaseTime: string;
	type: VersionType;
}

export interface Instance {
	name: string;
	game_metadata: InstanceGameInfo;
	mod_loader: ModLoader;
	mod_loader_version: string;
}

export type InstanceCardProps = {
	title: string;
	version: string;
	modLoader?: string;
	modCount?: number;
	lastPlayed: string;
	image: string;
	favorite?: boolean;
};

export interface MinecraftWorldMetadata {
	name: string;
	icon: string;
}

export interface ScreenshotMetadata {
	name: string;
	screenshot: string;
}

export interface GameInfo {
	worlds: MinecraftWorldMetadata[];
	screenshots: ScreenshotMetadata[];
}
