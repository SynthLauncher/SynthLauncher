import { invoke } from "@tauri-apps/api/core"

export const openSynthLauncherRootFolder = async () => {
    try {
        await invoke("open_synthlauncher_root_folder")
    } catch (error) {
        console.log(`openSynthLaucnherRootFolder error: ${error}`)
    }
}