import { reactive } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { StoreContentCategory, StoreContentHit, StoreContentHits, StoreContentVersion, StoreSource } from "@/types/store";
import { InstanceMetadata } from "@/types/instances";
import { setIfNotPresent } from "../utils";

export interface SelectedContentState {
    slug: string,
    versions: StoreContentVersion[],
    loading: boolean
}

export const storeManager = reactive({
    storeType: 'Modrinth' as StoreSource,
    storeCategory: 'modpacks' as StoreContentCategory,
    searchQuery: "",
    page: 1,

    items: undefined as StoreContentHits | undefined,
    loading: false,

    selectedContent: reactive<SelectedContentState>({
        slug: "",
        versions: [],
        loading: false
    }),
    selectedContents: new Map<string, StoreContentVersion | undefined>,
    selectedInstance: undefined as InstanceMetadata | undefined,

    selectContent: async (slug: string) => {
        if (storeManager.storeCategory === 'modpacks') {
            storeManager.selectedContents.clear()
            storeManager.selectedContent.slug = slug
            setIfNotPresent(storeManager.selectedContents, slug, undefined)
        } else {
            if (storeManager.selectedContents.has(slug) && storeManager.selectedContent.slug == slug) {
                storeManager.selectedContents.delete(slug)
                storeManager.selectedContent.slug = ""
            } else {
                storeManager.selectedContent.slug = slug
                setIfNotPresent(storeManager.selectedContents, slug, undefined)
            }
        }
    },

    fetchSearch: async (
        query: string,
        storeSrc: StoreSource,
        storeCategory: StoreContentCategory,
        storePage: number
    ) => {
        const result = await invoke<StoreContentHits>("cmd_search_store", {
            query: query,
            storeSrc: storeSrc,
            storeCategory: storeCategory,
            storePage: (storePage - 1)
        });

        return result
    },

    loadSearch: async () => {
        storeManager.loading = true
        try {
            storeManager.items = await storeManager.fetchSearch(
                storeManager.searchQuery,
                storeManager.storeType,
                storeManager.storeCategory,
                storeManager.page
            )
        } catch (e) {
            console.error("storeManager.loadSearch error: ", e);
        } finally {
            storeManager.loading = false
        }
    },

    // fetchContentVersions: async () => {
    //     if (storeManager.storeCategory === 'modpacks') {
    //         const result = await invoke<StoreContentVersion[]>("fetch_content_versions", {
    //             storeType: storeManager.storeType,
    //             slug: storeManager.selectedContent.slug,
    //             gameVersion: undefined,
    //             loader: undefined
    //         })
    //         return result
    //     }

    //     const result = await invoke<StoreContentVersion[]>("fetch_content_versions", {
    //         storeType: storeManager.storeType,
    //         slug: storeManager.selectedContent.slug,
    //         gameVersion: storeManager.selectedInstance?.mc_version,
    //         loader: storeManager.selectedInstance?.mod_loader
    //     })

    //     console.log(result)

    //     return result
    // },

    // loadContentVersions: async () => {
    //     storeManager.selectedContent.loading = true;
    //     try {
    //         storeManager.selectedContent.versions = await storeManager.fetchContentVersions()
    //         setIfNotPresent(storeManager.selectedContents, storeManager.selectedContent.slug, storeManager.selectedContent.versions[0])
    //     } catch (error) {
    //     } finally {
    //         storeManager.selectedContent.loading = false;
    //     }
    // },

    // installContent: async () => {
    //     try {
    //         await invoke("install_modpack", {
    //             slug: storeManager.selectedContent.slug,
    //             version: storeManager.selectedContents.get(storeManager.selectedContent.slug)?.id
    //         })
    //         // await invoke("install_content", {
    //         //     instanceName: storeManager.selectedInstance?.name,
    //         //     files: Array.from(storeManager.selectedContents.values()).map(val => val?.files[0])
    //         // })
    //         console.log("StoreManager.installContent log: Successfully installed")
    //     } catch (error) {
    //         console.error("StoreManager.installContent error: ", error)
    //     }
    // },
})