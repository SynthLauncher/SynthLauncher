export type StoreContentVersion = ModrinthProject
export type ContentFile = ModrinthFile

export type StoreContentCategory = "modpacks" | "mods" | "resourcepacks" | "shaderpacks"
export type StoreSource = "Modrinth" | "Curseforge"
export type ContentIdentifier = {
    modrinth: string,
    curseforge: number,
}

export type StoreContentHit = {
    source: StoreSource,
    type: StoreContentCategory,
    id: ContentIdentifier,
    name: string,
    slug: string,
    description: string,
    icon_url?: string,
    author: string,
    downloads: number    
}

export type StoreContentHits = {
    hits: StoreContentHit[],
    total_hits: number
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

