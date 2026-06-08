<script lang="ts">
    import { extractPalette, type Palette, type Swatch } from "@bojurgess/jiang";
    import { onDestroy, onMount } from "svelte";

    const imageURLs = [
        `https://i.scdn.co/image/ab67616d00001e02530f41ffd5a9bf22943d1be8`,
        `https://i.scdn.co/image/ab67616d00001e02dcdd9491f951ab39df5d203f`,
        `https://i.scdn.co/image/ab67616d00001e02912ac6ffde4f05d2ecd076d2`,
		'https://i.scdn.co/image/ab67616d0000b273d9194aa18fa4c9362b47464f'
    ];

    let currentImageIndex = $state(0);
    let imageURL = $derived(imageURLs[currentImageIndex]);

    let palette: Palette | null = $state(null);

    $effect(() => {
        fetch(imageURL)
            .then((res) => res.arrayBuffer())
            .then((buf) => extractPalette(new Uint8Array(buf), 64))
            .then((p) => (palette = p));
    });

    function paletteAsIterable(palette: Palette) {
        return Object.entries(palette) as [keyof Palette, Swatch][];
    }

    // Timer
    const CYCLE_DURATION = 3000;
    let animId: number | null = null;
    let startTime: number | null = null;

    function tick(timestamp: DOMHighResTimeStamp) {
        if (startTime === null) startTime = timestamp;
        if (timestamp - startTime >= CYCLE_DURATION) {
            currentImageIndex = (currentImageIndex + 1) % imageURLs.length;
            startTime = timestamp;
        }
        animId = requestAnimationFrame(tick);
    }

    function onVisibility() {
        if (document.visibilityState === 'visible') {
            startTime = null;
        }
    }

    onMount(() => {
        animId = requestAnimationFrame(tick);
        document.addEventListener('visibilitychange', onVisibility);
    });

    onDestroy(() => {
        if (animId) cancelAnimationFrame(animId);
        document.removeEventListener('visibilitychange', onVisibility);
    });
</script>

<div class="flex flex-col justify-center items-center gap-4 w-full h-screen">
    <img src={imageURL} alt="Album Art" class="w-64 h-64 rounded-xl" />
    {#if palette}
        <div class="flex w-full justify-center gap-4">
            {#each paletteAsIterable(palette) as [name, swatch]}
                {#if swatch}
                    <div class="flex flex-col justify-center items-center gap-2">
                        <div
                            class="w-16 h-16 rounded-xl"
                            style:background-color={swatch.hex}
                        ></div>
                        <span class="text-xs" style:color={swatch.hex}>{name}</span>
                        <span class="text-xs opacity-50">{swatch.hex}</span>
                    </div>
                {/if}
            {/each}
        </div>
    {/if}
</div>