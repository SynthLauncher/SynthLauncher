import { listen } from "@tauri-apps/api/event";
import base32 from "hi-base32"

const logCache = new Map<string, string[]>();
const logListeners = new Map<string, boolean>();

export function getLogs(instanceId: string) {
  return logCache.get(instanceId) ?? [];
}

export function addLog(instanceId: string, log: string) {
  const current = logCache.get(instanceId) ?? [];
  const updated = [...current, log];
  logCache.set(instanceId, updated);
}

export function hasListener(instanceId: string) {
  return logListeners.get(instanceId) === true;
}

export function markListener(instanceId: string) {
  logListeners.set(instanceId, true);
}

export function setupLogListener(instanceId: string) {
  if (hasListener(instanceId)) return; 

  let encoded = base32.encode(instanceId).replace(/=+$/, ""); 

  listen<string>(`${encoded}-console`, (event) => {
    addLog(instanceId, event.payload);
  });

  markListener(instanceId);
}