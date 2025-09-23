import { invoke } from "@tauri-apps/api/core"
import { InstanceMetadata } from "../../types/instances";

export const getAllInstances = async () => {
    try {
        const result = await invoke<InstanceMetadata[]>("get_all_instances");
        return result as InstanceMetadata[];
    } catch (error) {
        console.log(`getAllInstances error: ${error}`)
    }
}

export const getInstance = async (name: string) => {
    try {
        const result = await invoke<InstanceMetadata>("get_instance", { name: name })
        return result as InstanceMetadata
    } catch (error) {
        console.log(`getInstance error: ${error}`)
    }
}