<script lang="ts">
	import { daemonState } from '$lib/stores/daemon_states.svelte';
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';

	let style = $props();
</script>

<div class="w-1/6 shrink-0 overflow-y-auto border-l-3">
	<table class="w-full">
		<thead>
			<tr class="bg-slate-200">
				<th>Queue</th>
			</tr>
		</thead>
		<tbody>
			{#if daemonState.queue}
				{#each Object.values(daemonState.queue.next_tracks) as track}
					<tr
						class="track cursor-pointer odd:bg-white even:bg-slate-200 hover:bg-black hover:text-white"
					>
						<td>{track.metadata.title} - {track.metadata.artist ?? 'Unknown'}</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	</table>
</div>
