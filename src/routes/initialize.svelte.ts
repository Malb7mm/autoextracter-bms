import { loadConfig } from "@/libs/commands/config"
import { setVariables, variables } from "./settings/variables.svelte"

export const initialize = async () => {
  const config = await loadConfig();
  console.log(config);
  setVariables(config);
}