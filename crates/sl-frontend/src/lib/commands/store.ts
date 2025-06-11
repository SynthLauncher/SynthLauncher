import { message } from "@tauri-apps/plugin-dialog";
import { Search } from "../types/store";
import { invoke } from "@tauri-apps/api/core";

export const getStoreSearch = async () => {
  try {
    const search: Search = await invoke("search_store");
    return search;
  } catch (err) {
    await message(`getStoreSearch error: ${err}`, {
      title: "SynthLauncher Error",
      kind: "error",
    });
  }
};
