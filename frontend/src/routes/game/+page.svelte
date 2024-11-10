<script lang="ts">
	import { onMount } from "svelte";
  import { state, roomCode, isRoomCreator, pid, otherPlayers, webSocket } from '../stores';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';

  let currentState = '';
	let currentRoomCode = '';
	let currentPid = '';
	let currentIsRoomCreator = false;
  let currentOtherPlayers = [];
	let ws: WebSocket;

	roomCode.subscribe((value) => {
		currentRoomCode = value;
	});

	pid.subscribe((value) => {
		currentPid = value;
	});

	isRoomCreator.subscribe((value) => {
		currentIsRoomCreator = value;
	});

  otherPlayers.subscribe((value) => {
    currentOtherPlayers = value;
  });

  state.subscribe((value) => {
    currentState = value;
  });

  webSocket.subscribe((value) => {
    if (!value) {
      goto('/');
    } else {
      ws = value;
    }
  });
  onMount(() => {
    state.set('round 1');
  })
</script>