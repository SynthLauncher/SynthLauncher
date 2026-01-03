import { invoke } from "@tauri-apps/api/core";
import { reactive } from "vue";


export const launcherManager = reactive({
    minecraftVersions: [] as string[],

    init: async () => {
        launcherManager.minecraftVersions = await launcherManager.get_minecraft_versions();
    },

    get_minecraft_versions: async () => {
        try {
            const result = await invoke<string[]>("get_minecraft_versions");
            return result as string[];
        } catch (error) {
            console.log(`getAllInstances error: ${error}`)
            return []
        }
    },
})

launcherManager.init();