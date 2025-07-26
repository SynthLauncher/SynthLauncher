import { listen } from "@tauri-apps/api/event";

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

  listen<string>(`${instanceId}-console`, (event) => {
    addLog(instanceId, event.payload);
  });

  markListener(instanceId);
}