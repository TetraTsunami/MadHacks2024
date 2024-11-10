<script lang="ts">
	import { onMount } from 'svelte';
	import { roomCode, isRoomCreator, pid, otherPlayers, webSocket } from '../../stores';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';

	let currentRoomCode = '';
	let currentPid = '';
	let currentIsRoomCreator = false;
  let currentOtherPlayers: string[] = [];
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

	onMount(() => {
		if (!currentRoomCode || !currentPid) {
			goto('/');
			return;
		}

		ws = new WebSocket(`ws://localhost:8000/api/games/${currentRoomCode}/${currentPid}/ws`);

		ws.onmessage = (event) => {
			if (event.data === 'start') {
				goto('/game');
			}
		};
		webSocket.set(ws);
	});

	async function startGame() {
		try {
			const response = await fetch(`http://localhost:8000/api/games/${currentRoomCode}/start`, {
				method: 'POST'
			});
			if (!response.ok) {
				const errorData = await response.json();
				alert(errorData.error || 'Failed to start the game.');
			}
		} catch (err) {
			alert('Error starting the game.');
		}
	}
</script>

<div class="flex w-full flex-col items-center">
	<h1 class="mb-8 text-xl font-bold">Waiting Room</h1>
  <p>Room Code: {currentRoomCode}</p>
	{#if currentIsRoomCreator}
		<Button on:click={startGame}>Start Game</Button>
	{/if}
	<p class="mt-4">Waiting for other players to join...</p>
  <div class="text-left">
    <h2 class="text-lg">Current players</h2>
    <ul>
      {#each currentOtherPlayers as player}
        <li>{player}</li>
      {/each}
    </ul>
  </div>
</div>
