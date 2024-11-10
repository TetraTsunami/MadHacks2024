<script lang="ts">
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';
	import Input from '$lib/components/ui/input/input.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	export const roomCode = writable('');
	export const state = writable('login' as 'login' | 'loading' | 'waiting to start');
	let curState = '';
	state.subscribe((value) => {
		curState = value;
	});
	let name = '';
	let code = '';
	let error = '';

	async function verifyRoomCode(code: string): Promise<boolean> {
		try {
			const response = await fetch('http://localhost:8080/api/v1/room', {
				method: 'GET',
				headers: {
					'Room-Code': code
				}
			});
			if (response.ok) {
				error = '';
				return true;
			} else {
				error = 'Invalid room code';
				return false;
			}
		} catch (err) {
			error = 'Error verifying room code';
			return false;
		}
	}

	async function handleJoin() {
		state.set('loading');
		if (await verifyRoomCode(code)) {
			roomCode.set(code);
			state.set('waiting to start');
			goto('/waiting');
		} else {
			state.set('login');
		}
	}

	$: if (code.length === 4) {
		verifyRoomCode(code);
	}
</script>

<div class="flex w-full flex-col items-center">
	<h1 class="mb-8 text-xl font-bold">Join a Room</h1>
	<form on:submit|preventDefault={handleJoin} class="flex flex-col gap-4">
		<div>
			<label class="text-sm italic" for="room-code">Room Code:</label>
			<Input id="room-code" type="text" bind:value={code} maxlength={4} />
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