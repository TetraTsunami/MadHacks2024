<script lang="ts">
	import '../app.css';
</script>

<div class="flex min-h-screen w-full justify-center bg-background px-2">
	<div class="container max-h-screen gap-4 overflow-auto p-8 md:flex">
		<slot />
	</div>
</div>
