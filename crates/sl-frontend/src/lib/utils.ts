export function bytesToImageUrl(bytes: Uint8Array) {
  const blob = new Blob([new Uint8Array(bytes)], { type: "image/png" });
  return URL.createObjectURL(blob);
}
