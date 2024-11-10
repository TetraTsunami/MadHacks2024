<script lang="ts">
	import { onMount } from 'svelte';
	import { roomCode, isRoomCreator, pid, otherPlayers, webSocket } from '../../stores';
	import { goto } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { handleWebsocketMessage } from '../../lib';

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

		ws = new WebSocket(`ws://residual.at/api/games/${currentRoomCode}/${currentPid}/ws`);

		ws.onmessage = (event) => {
			handleWebsocketMessage(event.data);
		};
		webSocket.set(ws);
	});

	async function startGame() {
		try {
			const response = await fetch(`/api/games/${currentRoomCode}/start`, {
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

<div class="flex w-full flex-col items-center gap-2">
	<h1 class="mb-8 text-xl font-bold">Waiting Room</h1>
  <p>Room Code: {currentRoomCode}</p>
	<Button on:click={() => navigator.clipboard.writeText(currentRoomCode)}>Copy Room Code</Button>
	{#if currentIsRoomCreator}
		<Button on:click={startGame}>Start Game</Button>
	{/if}
	<p class="mt-4">Waiting for other players to join...</p>
	{#if currentIsRoomCreator}
		<div class="rounded-xl border border-foreground p-4 text-left">
			<h2 class="text-lg">Current players</h2>
			<ul class="list-disc pl-4">
				{#each currentOtherPlayers as player}
					<li>{player}</li>
				{/each}
			</ul>
		</div>
	{/if}
</div>
