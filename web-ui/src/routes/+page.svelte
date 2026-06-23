<script>
	import Library from '$lib/components/Library.svelte';
	import Control from '$lib/components/Control.svelte';
	import Host from '$lib/components/Host.svelte';
	import { connected } from '$lib/ws';
</script>

{#if $connected}
	<div class="flex h-screen flex-col dark:bg-black dark:text-white">
		<!-- <h1 class="text-2xl font-bold dark:text-white">Ametrine</h1> -->
		<Library></Library>
		<Control></Control>
	</div>
{:else}
	<div
		class="flex h-screen flex-col items-center justify-center space-y-10 align-middle dark:bg-black dark:text-white"
	>
		<h1 class="text-7xl">Connect</h1>
		<Host></Host>
	</div>
{/if}
