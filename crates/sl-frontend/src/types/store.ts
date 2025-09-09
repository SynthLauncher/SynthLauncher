export type StoreCategoryType = "modpacks" | "mods" | "resourcepacks" | "shaderpacks"
export type StoreType = "modrinth" | "curseforge"
export type StoreSearch = { hits: ModrinthContentMetadata[] } | { data: CurseforgeContentMetadata[] };

export interface StoreContentMetadata {
    source: StoreType,
    metadata: CurseforgeContentMetadata | ModrinthContentMetadata
}

interface CurseforgeContentMetadata {
    id: number,
    name: string,
    slug: string,
    summary: string,
    download_count: number,
    logo: { url?: string },
    authors: { name: string }[]
}

interface ModrinthContentMetadata {
    id: string,
    slug: string,
    title: string,
    description: string,
    downloads: number,
    icon_url?: string,
    author: string,
    followers: number
}
