import { loadConfig, writeConfig } from "@/libs/commands/config"
import { setVariables, variables } from "./settings/variables.svelte"
import { listen } from "@tauri-apps/api/event";
import { log, type LogMessagePayload } from "@/components/log-table/logMessages.svelte";

export const initialize = async () => {
  listen("log-info", e => {
    const payload = e.payload as LogMessagePayload
    log("Info", payload.message, payload.jump_to, payload.created_at);
  });

  listen("log-alert", e => {
    const payload = e.payload as LogMessagePayload
    log("Alert", payload.message, payload.jump_to, payload.created_at);
  });

  listen("log-error", e => {
    const payload = e.payload as LogMessagePayload
    log("Error", payload.message, payload.jump_to, payload.created_at);
  });

  const config = await loadConfig();
  setVariables(config);
  writeConfig({});
}