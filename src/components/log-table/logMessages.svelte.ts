import { v4 as uuidv4 } from 'uuid';
import { listen } from '@tauri-apps/api/event';

type Message = {
  id: string,
  message: string,
  jumpTo?: string,
  type: "Info" | "Alert" | "Error",
}

export const logs: Message[] = $state([]);

function addLog(log: Message) {
  logs.unshift(log);
  if (logs.length > 25) {
    logs.pop();
  }
}

export function logInfo(message: string, jumpTo: string | undefined = undefined) {
  addLog({ id: uuidv4(), message, jumpTo, type: "Info" });
}

export function logAlert(message: string, jumpTo: string | undefined = undefined) {
  addLog({ id: uuidv4(), message, jumpTo, type: "Alert" });
}

export function logError(message: string, jumpTo: string | undefined = undefined) {
  addLog({ id: uuidv4(), message, jumpTo, type: "Error" });
}

type Payload = {
  message: string,
  jumpTo?: string,
}

listen("log-info", e => {
  const payload = e.payload as Payload
  logInfo(payload.message, payload.jumpTo);
})

listen("log-alert", e => {
  const payload = e.payload as Payload
  logAlert(payload.message, payload.jumpTo);
})

listen("log-error", e => {
  const payload = e.payload as Payload
  logError(payload.message, payload.jumpTo);
})