<script lang="ts">
    import { untrack } from "svelte";
    import Artwork from "./Artwork.svelte";
    import {
        peekQueueCover,
        requestQueueCover,
        type CoverPriority,
        type CoverStatus,
        type CoverTicket,
        type Track,
    } from "../services/music";
    import { library } from "../stores/library.svelte";
    import { watchRow, type RowState } from "../utils/row-visibility";
    let { track, current = false }: { track: Track; current?: boolean } = $props();
    let host = $state<HTMLSpanElement>();
    // Depends on the path string, not the track object: snapshots rebuild the queue array.
    const path = $derived(track.path);
    let shown = $state<{ path: string; src: string | null; status: CoverStatus | "loading"; code?: string } | null>(null);
    let ticket: CoverTicket | null = null;
    let row: RowState = { near: false, seen: false };
    const priority = (): CoverPriority => (current ? 0 : row.seen ? 1 : 2);
    $effect(() => {
        const requested = path;
        void library.coverEpoch;
        const node = host;
        if (!node) return;
        // A different track in this row drops the old picture; a rescan keeps it until the new result.
        if (untrack(() => shown?.path !== requested)) shown = { path: requested, src: null, status: "loading" };
        const hit = peekQueueCover(requested);
        if (hit && hit.status !== "failed") {
            shown = { path: requested, src: hit.src, status: hit.status };
            return;
        }
        let finished = false;
        const done = (o: { status: CoverStatus; src: string | null; code?: string }) => {
            ticket = null;
            // Dropped preloads ask again once the row is visible.
            if (o.code === "dropped") return;
            finished = true;
            shown = { path: requested, src: o.src, status: o.status, code: o.code };
        };
        const ask = () => {
            if (finished) return;
            if (!row.near && !row.seen && !current) {
                ticket?.cancel();
                ticket = null;
            } else if (ticket) ticket.setPriority(priority());
            else {
                const next = requestQueueCover(requested, priority(), done);
                if (!finished) ticket = next;
            }
        };
        const list = node.closest(".queue-list");
        // The row box, not the span: row contents are skipped by content-visibility while off-screen.
        const target = node.closest(".queue-item") ?? node;
        row = { near: false, seen: false };
        const stop = list
            ? watchRow(target, list, "360px 0px", (state) => {
                  row = state;
                  ask();
              })
            : ((row = { near: true, seen: true }), untrack(ask), () => {});
        return () => {
            stop();
            ticket?.cancel();
            ticket = null;
        };
    });
    $effect(() => {
        void current;
        ticket?.setPriority(priority());
    });
</script>

<span
    bind:this={host}
    class="cover-fill"
    data-cover={shown?.status ?? "loading"}
    data-cover-code={shown?.code}
    ><Artwork src={shown?.src ?? null} alt={track.album || track.title} /></span
>
