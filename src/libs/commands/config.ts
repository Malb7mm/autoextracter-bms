import { invoke } from "@tauri-apps/api/core";

export type ExtractedHandlingMode = "NoAction" | "Delete" | "Move";

export type Config = {
  directory: {
    monitor: string,
    output: string,
  },
  behavior: {
    create_wrapper_folder: boolean,
    extracted_handling: {
      handling_mode: ExtractedHandlingMode,
      delete_permanently: boolean,
      move_destination: string,
    }
  },
}

export const loadConfig = async () => {
  const result = await invoke("load_config");
  return result as Config;
}

export const writeConfig = async (config: Partial<Config>) => {
  await invoke("write_config", {config});
}