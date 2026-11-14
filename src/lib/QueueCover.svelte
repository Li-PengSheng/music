<script lang="ts">
    import { onMount } from "svelte";
    import Artwork from "./Artwork.svelte";
    import { queueCover, type Track } from "./services/music";
    let { track }: { track: Track } = $props();
    let host: HTMLSpanElement;
    let src = $state<string | null>(null);
    onMount(() => {
        let disposed = false;
        const observer = new IntersectionObserver(
            (entries) => {
                if (entries.some((e) => e.isIntersecting)) {
                    observer.disconnect();
                    void queueCover(track)
                        .then((value) => {
                            if (!disposed) src = value;
                        })
                        .catch(() => {});
                }
            },
            { rootMargin: "100px" },
        );
        observer.observe(host);
        return () => {
            disposed = true;
            observer.disconnect();
        };
    });
</script>

<span bind:this={host} class="cover-fill"
    ><Artwork {src} alt={track.album || track.title} /></span
>
