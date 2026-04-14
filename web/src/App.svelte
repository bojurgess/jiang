<script lang="ts">
	import { extractPalette, type Colour, type Palette } from "@bojurgess/jiang";
    import { onDestroy, onMount } from "svelte";

	let imageURLs = [
		`https://i.scdn.co/image/ab67616d00001e02530f41ffd5a9bf22943d1be8`,
		`https://i.scdn.co/image/ab67616d00001e02dcdd9491f951ab39df5d203f`,
		`https://i.scdn.co/image/ab67616d00001e02912ac6ffde4f05d2ecd076d2`
	];
	let currentImageIndex = $state(0);
	let imageURL = $derived(imageURLs[currentImageIndex]);
	let palette = $derived(
		await fetch(imageURL)
			.then((res) => res.arrayBuffer())
			.then((buf) => extractPalette(new Uint8Array(buf), {
				algorithm: "medianCut",
				k: 64,
				scoringOptions: undefined
			}))
	);

	$inspect(palette);

	let lastTime: number | null = null;
	let animId: number | null = null;
	let accumulated: number = 0;

	const SECONDS_BEFORE_CHANGE = 3;

	function tick(timestamp: DOMHighResTimeStamp) {
	    if (lastTime === null) lastTime = timestamp;
	    accumulated += (timestamp - lastTime) / 1000;
	    lastTime = timestamp;
	
	    if (accumulated >= SECONDS_BEFORE_CHANGE) {
	        currentImageIndex = (currentImageIndex + 1) % imageURLs.length;
	        accumulated -= SECONDS_BEFORE_CHANGE;
	    }
	
	    animId = requestAnimationFrame(tick);
	}

	function paletteAsIterable(palette: Palette) {
		return Object.entries(palette) as [keyof Palette, Colour][];
	}

	onMount(() => {
	    animId = requestAnimationFrame(tick);
	    document.addEventListener('visibilitychange', onVisibility);
	});

	onDestroy(() => {
	    if (animId) cancelAnimationFrame(animId);
	    document.removeEventListener('visibilitychange', onVisibility);
	});

	function onVisibility() {
	    if (document.visibilityState === 'visible') lastTime = null;
	}
</script>

<div class="flex flex-col justify-center items-center gap-4 w-full h-screen">
	<img src={imageURL} alt="Album Art" class="w-64 h-64 rounded-xl" />

{#if palette}
	<div class="flex w-full justify-center gap-4">
		{#each paletteAsIterable(palette) as [name, swatch] }
			{#if swatch}
			{@const { r, g, b } = swatch}
			<div class="flex flex-col justify-center items-center">
				<div class="w-16 h-16 rounded-xl" style:background-color={`rgb(${r}, ${g}, ${b})`}></div>
				<div class="text-sm">{name}</div>
			</div>
			{/if}
		{/each}
	</div>
{/if}
</div>