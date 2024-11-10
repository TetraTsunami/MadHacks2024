<script lang="ts">
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { state, roomCode, pid, isRoomCreator, otherPlayers } from './stores';
	
	let curState = '';
	state.subscribe((value) => {
		curState = value;
	});
	let name = '';
	let code = '';
	let error = '';

	async function handleJoin() {
		state.set('loading');
		try {
			const response = await fetch(`http://localhost:8000/api/games/${code}?name=${name}`, {
				method: 'PUT'
			});
			if (response.ok) {
				const data = await response.json();
				roomCode.set(data.code);
				pid.set(data.pid);
				isRoomCreator.set(false);
				state.set('waiting to start');
				goto('/game/waiting');
			} else {
				const errorData = await response.json();
				error = errorData.error || 'Invalid room code or name';
				state.set('login');
			}
		} catch (err) {
			error = 'Error joining the room';
			state.set('login');
		}
	}

	async function handleCreate() {
		state.set('loading');
		try {
			const response = await fetch(`http://localhost:8000/api/games?name=${name}`, {
				method: 'POST'
			});
			if (response.ok) {
				const data = await response.json();
				roomCode.set(data.code);
				pid.set(data.pid);
				otherPlayers.set([name]);
				isRoomCreator.set(true);
				state.set('waiting to start');
			
				goto('/game/waiting');
			} else {
				error = 'Error creating room';
				state.set('login');
			}
		} catch (err) {
			error = 'Error creating room';
			state.set('login');
		}
	}
</script>

<div class="flex w-full flex-col items-center">
	<h1 class="mb-8 text-xl font-bold">Join a Room</h1>
	<form on:submit|preventDefault={handleJoin} class="flex flex-col gap-4">
		<div>
			<label class="text-sm italic" for="room-code">Room Code:</label>
			<Input id="room-code" type="text" bind:value={code} />
		</div>
		{#if error}
			<div class="text-red-400">{error}</div>
		{/if}
		<div>
			<label class="text-sm italic" for="name">Name:</label>
			<Input id="name" type="text" bind:value={name} />
		</div>
		<Button type="submit">Join
			{#if (curState === 'loading')}
				<span class="spinner ml-2 animate-spin" />
			{/if}
		</Button>
	</form>

	<h1 class="mb-8 mt-8 text-xl font-bold">Create a Room</h1>
	<form on:submit|preventDefault={handleCreate} class="flex flex-col gap-4">
		<div>
			<label class="text-sm italic" for="name">Name:</label>
			<Input id="name" type="text" bind:value={name} />
		</div>
		<Button type="submit">Create
			{#if (curState === 'loading')}
				<span class="spinner ml-2 animate-spin" />
			{/if}
		</Button>
	</form>
</div>

<style>
	.spinner {
		border-top-color: transparent;
		border-left-color: transparent;
		border-bottom-color: transparent;
		border-right-color: currentColor;
		border-width: 2px;
		border-style: solid;
		border-radius: 9999px;
		width: 1em;
		height: 1em;
	}
</style>