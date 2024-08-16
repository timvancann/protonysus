<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { open } from '@tauri-apps/api/dialog';
	import { writable, type Writable } from 'svelte/store';
	import { onMount } from 'svelte';
	import { Duration } from 'luxon';
	import IcBaselinePlayArrow from '~icons/ic/baseline-play-arrow';
	import IcBaselinePause from '~icons/ic/baseline-pause';
	import IcBaselineSkipPrevious from '~icons/ic/baseline-skip-previous';
	import IcBaselineSkipNext from '~icons/ic/baseline-skip-next';

	type Track = {
		path: string;
		duration: number;
		file_name: string;
		id3_tags?: {
			title?: string;
			artist?: string;
			album?: string;
			year?: string;
			album_art: string;
			track?: string;
		},
	};

	type Library = {
		tracks: Track[];
	};

	type PlayerState = {
		play_state: 'Playing' | 'Paused' | 'Stopped';
		current_track?: Track
		progress: number;
	};

	const PlayerStateStore: Writable<PlayerState> = writable({
		play_state: 'Stopped',
		progress: 0
	});
	const LibraryStore: Writable<Library> = writable({
		tracks: []
	});

	$: state = $PlayerStateStore;
	$: library = $LibraryStore;

	async function fetchData() {
		state = await invoke('player_state');
	}

	onMount(() => {
		const interval = setInterval(fetchData, 100);
		fetchData();

		return () => clearInterval(interval);
	});

	async function pause() {
		await invoke('pause');
	}

	async function stop() {
		await invoke('stop');
	}

	async function play() {
		await invoke('play');
	}

	async function previous() {
		await invoke('previous');
	}

	async function next() {
		await invoke('next');
	}

	async function select(track: Track) {
		await invoke('select_track', { track: track });
	}

	async function pickFile() {
		let files = [];
		let selected = await open({
			multiple: true,
			directory: true,
			filters: [
				{
					name: 'Audio',
					extensions: ['mp3', 'wav', 'ogg', 'flac']
				}
			]
		});
		if (Array.isArray(selected)) {
			files = selected;
		} else {
			files = [selected];
		}

		library = await invoke('upload_file', { files: files });
	}

</script>

<div class="flex flex-row gap-5">
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
		on:click={pickFile}>Pick a file
	</button
	>
	{#if state.play_state === "Paused"}
		<button
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
			on:click={play}
		>Play
		</button>
	{:else if state.play_state === "Playing"}
		<button
			class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
			on:click={pause}
		>Pause
		</button>
	{/if}
	<button
		class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
		on:click={stop}
	>Stop
	</button>
</div>

<div class="flex flex-row items-center justify-center">
	<div class="flex flex-row">
		<div class="h-20 w-20">
			<img class="object-cover rounded-xl shadow-sm" src="{state.current_track?.id3_tags?.album_art}" alt="current">
		</div>
		<div class="flex flex-col justify-center ml-2">
			<div>
				<p class="font-bold text-sm">{state.current_track?.id3_tags?.title}</p>
			</div>
			<div>
				<p class="text-xs">{state.current_track?.id3_tags?.artist}</p>
			</div>
		</div>
	</div>
	<div class="flex flex-col">
		<div id="player" class="flex flex-row space-x-2 justify-center items-center">
			<button
				class="w-10 h-10"
				on:click={previous}>
				<IcBaselineSkipPrevious
					class="w-10 h-10" />
			</button>
			<div>
				{#if state.play_state === "Paused"}
					<button
						class="rounded-full bg-amber-500 w-16 h-16 text-amber-950 shadow-md"
						on:click={play}
					>
						<IcBaselinePlayArrow
							class="w-16 h-16"
						/>
					</button>
				{:else if state.play_state === "Playing"}
					<button
						class="rounded-full bg-amber-500 w-16 h-16 text-amber-950 shadow-md"
						on:click={pause}
					>
						<IcBaselinePause
							class="w-16 h-16"
						/>
					</button>
				{/if}
			</div>
			<button
				class="w-10 h-10"
				on:click={next}>
				<IcBaselineSkipNext
					class="w-10 h-10" />
			</button>
		</div>
		{#if state.current_track}
			<div class="text-center flex flex-row space-x-2 items-center justify-center text-xs font-sans">
				<p class="w-10">{Duration.fromMillis(state.progress).toFormat('mm:ss')}</p>
				<div class="relative flex">
					<div class="w-60 bg-amber-200 h-2 rounded-full">
						<div class="bg-amber-800 h-2 leading-none rounded-full px-1"
								 style="width: {state.progress / state.current_track?.duration * 100}%"
						/>

					</div>
				</div>
				<p class="w-10">{Duration.fromMillis(state.current_track?.duration || 0).toFormat('mm:ss')}</p>

				<div>
					<p>{state.current_track?.path}</p>
				</div>
			</div>
		{/if}
	</div>
</div>

<div class="mt-4 max-w-3xl items-center justify-center mx-auto">
	<div class="shadow-sm my-7 bg-slate-50">
		<table class="border-collapse table-auto w-full text-sm">
			<thead class="border-b font-medium text-slate-400 text-left">
			<tr>
				<th class="p-4 pl-8 pt-0 pb-3"></th>
				<th class="p-4 pl-8 pt-0 pb-3">Song</th>
				<th class="p-4 pl-8 pt-0 pb-3">Artist</th>
				<th class="p-4 pl-8 pt-0 pb-3">Album</th>
				<th class="p-4 pl-8 pt-0 pb-3">Year</th>
			</tr>
			</thead>
			<tbody class="bg-white">
			{#each library.tracks as track (track.path)}
				<tr
					class="hover:text-amber-600 text-slate-500 cursor-pointer border-slate-100 border-b transition-colors duration-200 ease-in-out
					{track.path === state.current_track?.path ? 'bg-amber-50' : ''}"
					on:click={select(track)}>
					<td><img class="w-12 h-12 p-1 object-cover rounded-md" alt="album_art" src="{track.id3_tags?.album_art}" />
					</td>
					<td class="p-4 pl-8">{track.id3_tags?.title}</td>
					<td class="p-4 pl-8">{track.id3_tags?.artist}</td>
					<td class="p-4 pl-8">{track.id3_tags?.album}</td>
					<td class="p-4 pl-8">{track.id3_tags?.year}</td>
				</tr>
			{/each}
			</tbody>
		</table>
	</div>
</div>
