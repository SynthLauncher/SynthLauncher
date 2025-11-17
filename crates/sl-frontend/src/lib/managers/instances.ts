import { reactive } from 'vue';
import { InstanceMetadata, ModLoader } from '../../types/instances';
import { invoke } from '@tauri-apps/api/core';

export const instancesManager = reactive({
    loading: false,
    instances: [] as InstanceMetadata[],

    add: (instance: InstanceMetadata) => {
        instancesManager.instances.push(instance)
    },

    get: (name: string) => {
        return instancesManager.instances.find(instance => instance.name === name);
    },

    refresh: async () => {
        instancesManager.loading = true;
        instancesManager.instances = await instancesManager.fetch_instances();
        instancesManager.loading = false;
    },

    init: async () => {
        if (instancesManager.instances.length === 0) {
            await instancesManager.refresh();
        }
    },

    fetch_instances: async () => {
        try {
            const result = await invoke<InstanceMetadata[]>("get_all_instances");
            return result as InstanceMetadata[];
        } catch (error) {
            console.log(`getAllInstances error: ${error}`)
            return []
        }
    },

    create_instance: async (
        name: string,
        game_version: string,
        mod_loader: ModLoader,
        mod_loader_version?: string
    ) => {
        try {
            const new_instance = await invoke<InstanceMetadata>("create_instance", {
                instanceName: name,
                gameVersion: game_version,
                loader: mod_loader,
                loaderVersion: mod_loader_version
            })

            instancesManager.add(new_instance)
        } catch (error) {
        }
    },

    launch_instance: async (name: string) => {
        try {
            await invoke("launch_instance", { name: name })
        } catch (error) {
        }
    },

    delete_instance: async () => {

    },

    edit_instance: async () => {

    }
});

await instancesManager.init()