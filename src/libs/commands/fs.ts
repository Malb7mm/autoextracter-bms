import { invoke } from "@tauri-apps/api/core";

export const isValidDir = async (path: string) => {
  const result = await invoke("is_valid_dir", { pathStr: path });
  return result as boolean;
}

export const openDir = async (path: string) => {
  await invoke("open_dir", { pathStr: path });
}