import { watch } from "vue";
import { storeManager } from "./managers/store";
import { instancesManager } from "./managers/instances";
import { launcherManager } from "./managers/launcher";

// watch(storeManager, () => console.log("Store Manager State: ", storeManager))
// watch(instancesManager, () => console.log("Instance Manager State: ", instancesManager))
// watch(launcherManager, () => console.log(`:`, launcherManager))