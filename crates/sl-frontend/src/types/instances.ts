export type ModLoader = "Vanilla" | "Fabric" | "Quilt" | "Forge" | "NeoForge";

export interface InstanceMetadata {
    scheme_version: number,
    name: string,
    mc_version: string,
    mod_loader_version: string
    mod_loader: ModLoader,
    icon?: Uint8Array
}
