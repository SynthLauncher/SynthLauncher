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


export interface Instance {
	name: string;
	mc_version: string;
	releaseTime: string;
	type: VersionType;	
	mod_loader: ModLoader;
	mod_loader_version: string;
}

export type InstanceCardProps = {
	title: string;
	version: string;
	modLoader?: string;
	image: string;
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
