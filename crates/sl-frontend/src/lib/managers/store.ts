import { reactive } from "vue";
import { StoreCategoryType, StoreContentVersions, StoreSearch, StoreType } from "../../types/store";
import { invoke } from "@tauri-apps/api/core";
import { instancesManager } from "./instances";
import { InstanceMetadata } from "../../types/instances";

export interface SelectedContentState
{
    slug: string,
    versions: StoreContentVersions,
    loading: boolean
}

export const storeManager = reactive({
    storeType: "modrinth" as StoreType,
    storeCategory: "modpacks" as StoreCategoryType,
    searchQuery: "",
    page: 1,

    items: undefined as StoreSearch | undefined,
    loading: false,

    selectedContent: {
        slug: "",
        versions: undefined as StoreContentVersions | undefined,
        loading: false
    } as SelectedContentState,

    selectedContents: new Set<string>,
    selectedInstance: undefined as InstanceMetadata | undefined,

    selectContent: async (slug: string) => {
        if (storeManager.selectedContents.has(slug) && storeManager.selectedContent.slug == slug)
        {
            storeManager.selectedContents.delete(slug)     
            storeManager.selectedContent.slug = ""
        } else {
            storeManager.selectedContent.slug = slug
            storeManager.selectedContents.add(slug)
        }
    },

    fetchSearch: async (
        searchQuery: string,
        storeType: StoreType,
        storeCategory: StoreCategoryType,
        storePage: number
    ) => {
        const result = await invoke<StoreSearch>("fetch_store_search", {
                searchQuery: searchQuery,
                storeType: storeType,
                storeCategory: storeCategory,
                storePage: (storePage - 1)
            });

        return result
    },

    fetchContentVersions: async () => {
        const result = await invoke<StoreContentVersions>("fetch_content_versions",  {
            storeType: storeManager.storeType,
            slug: storeManager.selectedContent.slug,
            gameVersion: storeManager.selectedInstance?.mc_version,
            loader: storeManager.selectedInstance?.mod_loader
        })

        return result
    },

    loadContentVersions: async () => {
        storeManager.selectedContent.loading = true;
        try {
            storeManager.selectedContent.versions = await storeManager.fetchContentVersions()
        } catch (error) {
            
        } finally {
            storeManager.selectedContent.loading = false;
        }
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
        } catch (error) {

        } finally {
            storeManager.loading = false
        }
    }

})