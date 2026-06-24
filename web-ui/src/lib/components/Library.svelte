<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';

	let style = $props();
</script>

<table class="flex-1 overflow-auto">
	<thead>
		<tr class="bg-slate-200">
			<th>Track</th>
			<th>Artist</th>
		</tr>
	</thead>
	<tbody>
		{#each Object.values(daemonState.library) as track}
			<tr
				class="track cursor-pointer odd:bg-white even:bg-slate-200 hover:bg-black hover:text-white"
				onclick={() => queue.enqueue(track.id)}
			>
				<td class="title">{track.metadata.title}</td>
				<td class="artist">{track.metadata.artist ?? 'Unknown'}</td>
			</tr>
		{/each}
	</tbody>
</table>
