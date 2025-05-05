import { v4 as uuidv4 } from 'uuid';
import { listen } from '@tauri-apps/api/event';

type MessageType = "Info" | "Alert" | "Error";

type Message = {
  id: string,
  message: string,
  createdAt: Date,
  jumpTo?: string,
  type: MessageType,
}

export const logs: Message[] = $state([]);

function addLog(log: Message) {
  logs.unshift(log);
  if (logs.length > 25) {
    logs.pop();
  }
}

export function log(type: MessageType, message: string, jumpTo: string | undefined, createdAt: string) {
  const date = new Date(createdAt);
  addLog({ id: uuidv4(), message, jumpTo, type, createdAt: date });
}

export type LogMessagePayload = {
  message: string,
  jump_to?: string,
  created_at: string,
}