import { writeConfig, type Config, type ExtractedHandlingMode } from "@/libs/commands/config";

export type PartialConfig = {
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

export const variables: PartialConfig = $state({
  directory: {
    monitor: "",
    output: "",
  },
  behavior: {
    create_wrapper_folder: false,
    extracted_handling: {
      handling_mode: "NoAction",
      delete_permanently: false,
      move_destination: "",
    }
  },
});

export const setVariables = (value: Config | PartialConfig) => {
  Object.assign(variables, value);
  writeConfig(variables);
}