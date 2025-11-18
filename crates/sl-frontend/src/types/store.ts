export type StoreCategoryType = "modpacks" | "mods" | "resourcepacks" | "shaderpacks"
export type StoreType = "modrinth" | "curseforge"
export type StoreSearch = { hits: ModrinthProjectSearch[] } | { data: CurseforgeContentMetadata[] };
export type StoreContentVersion = ModrinthProject
export type ContentFile = ModrinthFile

export type StoreSearchResult = {
    source: StoreType,
    data: { hits: ModrinthProjectSearch[] } | { data: CurseforgeContentMetadata[] }
}

interface CurseforgeContentMetadata {
    id: number,
    name: string,
    slug: string,
    summary: string,
    downloadCount: number,
    logo: { url?: string },
    authors: { name: string }[]
}

interface ModrinthProjectSearch {
    id: string,
    slug: string,
    title: string,
    description: string,
    downloads: number,
    icon_url?: string,
    author: string,
}

interface ModrinthFile {
    filename: string,
    url: string
}

interface ModrinthProject {
    id: string,
    name: string,
    game_versions: string[],
    files: ModrinthFile[]
}

