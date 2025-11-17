import { open } from '@tauri-apps/plugin-shell';

export function bytesToImageUrl(bytes: Uint8Array) {
  const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
  return URL.createObjectURL(blob);
}

export async function openDiscordLink() {
  try {
    await open('https://discord.gg/ajZux2Uy9E')
  } catch (e) {
    console.error('Failed to open link:', e)
  }
}