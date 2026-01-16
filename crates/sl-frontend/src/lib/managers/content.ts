import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";

export enum ContentSource {
    Modrinth,
    Curseforge,
    External
}

export type ContentType =
    | { Mod: boolean }
    | "Resourcepack"
    | "Shaderpack";

export interface Content {
    name: string,
    hash?: string,
    source: ContentSource,
    type: ContentType
}

export interface ContentList {
    scheme_version: number,
    list: Map<string, Content>
}

export const contentManager = reactive({
    loading: false,

    get_content_list: async (instance_name: string) => {
        const list = await invoke<ContentList>("get_content_list", { instanceName: instance_name });
        return list as ContentList
    },

    add_content: async () => {

    },

    remove_content: async () => {

    }
})
