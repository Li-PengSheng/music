<script lang="ts">
    import Artwork from "./Artwork.svelte";
    import { queueCover, type Track } from "./services/music";
    import { library } from "./stores/library.svelte";
    let { track }: { track: Track } = $props();
    let host = $state<HTMLSpanElement>();
    let src = $state<string | null>(null);
    $effect(() => {
        const requested = track.path;
        const epoch = library.coverEpoch;
        const node = host;
        if (!node) return;
        src = null;
        let disposed = false;
        const observer = new IntersectionObserver(
            (entries) => {
                if (!entries.some((e) => e.isIntersecting)) return;
                observer.disconnect();
                void queueCover(track)
                    .then((value) => {
                        if (!disposed && requested === track.path && epoch === library.coverEpoch) src = value;
                    })
                    .catch(() => {});
            },
            { rootMargin: "100px" },
        );
        observer.observe(node);
        return () => {
            disposed = true;
            observer.disconnect();
        };
    });
</script>

<span bind:this={host} class="cover-fill"
    ><Artwork {src} alt={track.album || track.title} /></span
>
