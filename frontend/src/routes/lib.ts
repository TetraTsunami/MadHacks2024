import { goto } from "$app/navigation";
import { otherPlayers, state } from "./stores";

export const handleWebsocketMessage = (message: string) => {
  const command = message.split(' ')[0];
  console.log("webhook message", message);
  switch (command) {
    case 'start':
      state.set('round 1');
      goto('/game');
      break
    case 'new-player':
      otherPlayers.update((value) => [...value, message.split(' ')[1]]);
      break;
  }
}