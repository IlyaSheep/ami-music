<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
</script>

<div class="library">
	{#each Object.values(daemonState.library) as track}
		<button class="track" onclick={() => queue.enqueue(track.id)}>
			<span class="title">{track.metadata.title}</span>
			<span class="artist">{track.metadata.artist ?? 'Unknown'}</span>
			<span class="album">{track.metadata.album ?? 'Unknown'}</span>
		</button>
	{/each}
</div>
