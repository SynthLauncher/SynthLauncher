import { invoke } from "@tauri-apps/api/core"
import { StoreCategoryType, StoreSearch, StoreType } from "../../types/store"

export const fetchStoreSearch = async (searchQuery: string, storeType: StoreType, storeCategory: StoreCategoryType, storePage: number) => {
    try {
        const result = await invoke<StoreSearch>("fetch_store_search", { 
            searchQuery: searchQuery,
            storeType: storeType,
            storeCategory: storeCategory,
            storePage: (storePage - 1)
         })
        return result as StoreSearch
    } catch (error) {
        console.log(`fetchStoreSearch error: ${error}`)
    }
}

export const downloadStoreContent = async (slug: string, version: string, instance_name: string) => {
    try {
        await invoke("download_store_content", { 
            slug: slug,
            version: version,
            instanceName: instance_name
         })
    } catch (error) {
        console.log(`fetchStoreSearch error: ${error}`)
    }
}

