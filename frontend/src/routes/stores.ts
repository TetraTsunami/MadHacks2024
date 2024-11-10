import { writable } from "svelte/store";

export const roomCode = writable('');
export const isRoomCreator = writable(false);
export const state = writable('login' as 'login' | 'loading' | 'waiting to start' | 'round 1');
export const pid = writable('');
export const otherPlayers = writable([] as string[]);
export const webSocket = writable<WebSocket | null>(null);