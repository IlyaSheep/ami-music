<script lang="ts">
	import * as playback from '$lib/commands/playback';
	import * as queue from '$lib/commands/queue';
	import * as library from '$lib/commands/library';
	import { connect } from '$lib/ws';

	let value = $state('');

	function onkeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			connect(value);
		}
	}
</script>

<button onclick={playback.play}>Play</button>
<button onclick={playback.pause}>Pause</button>

<input type="string" bind:value {onkeydown} />
