<script lang="ts">
    import { onMount, tick, setContext } from "svelte";
    import { fly } from "svelte/transition";
    import { X } from "lucide-svelte";
    import { shortcut } from "./lib/utils/shortcuts";
    import { diagnostics } from "./lib/utils/cover-diagnostics";
    import {
        desktopAvailable,
        verifyConnection,
        getPlayerState,
        getArtwork,
        desktopPreferences,
        openFile,
        play,
        pause,
        queueAction,
        cancelScan,
    } from "./lib/services/music";
    import { player, accept, control } from "./lib/stores/player.svelte";
    import {
        library,
        refreshSummary,
        watchScans,
    } from "./lib/stores/library.svelte";
    import { nav, routeKey, memory } from "./lib/stores/nav.svelte";
    import { ui, dismissToast, notifyError } from "./lib/stores/ui.svelte";
    import {
        loadPreferences,
        watchSystemTheme,
        motionMs,
    } from "./lib/stores/preferences";
    import { installNativeMenuGuard } from "./lib/utils/native-menu";
    import { escapeLayer } from "./lib/utils/layers";
    import Sidebar from "./lib/layout/Sidebar.svelte";
    import TopBar from "./lib/layout/TopBar.svelte";
    import PlayerBar from "./lib/layout/PlayerBar.svelte";
    import Queue from "./lib/layout/Queue.svelte";
    import NowPlaying from "./lib/views/NowPlaying.svelte";
    import TrackDetails from "./lib/components/TrackDetails.svelte";
    import ContextMenu from "./lib/components/ContextMenu.svelte";
    import Dialog from "./lib/components/Dialog.svelte";
    import HomeView from "./lib/views/HomeView.svelte";
    import SongsView from "./lib/views/SongsView.svelte";
    import GridView from "./lib/views/GridView.svelte";
    import SettingsView from "./lib/views/SettingsView.svelte";

    let snapshot = $derived(player.snapshot);
    let searchInput = $state<HTMLInputElement>();
    let queueTrigger = $state<HTMLButtonElement>();
    let main = $state<HTMLElement>();
    setContext("scroller", () => main);

    // Cover for the current song: one request per track change, stale results ignored.
    let artwork = $state<string | null>(null);
    let artworkRequest = 0;
    let artworkTrackId = $derived(snapshot?.track_id);
    $effect(() => {
        const path = artworkTrackId;
        if (diagnostics) {
            diagnostics.trackChanges++;
            if (path && desktopAvailable) diagnostics.coverRequests++;
        }
        const ticket = ++artworkRequest;
        artwork = null;
        if (path && desktopAvailable)
            void getArtwork()
                .then((a) => {
                    if (ticket === artworkRequest) artwork = a;
                })
                .catch(() => {});
    });
    async function connect() {
        try {
            const r = await verifyConnection();
            accept(r.snapshot);
            player.connection = "Rust 播放引擎已连接";
            player.connected = true;
        } catch (e) {
            player.connection = desktopAvailable
                ? "连接失败，请重试"
                : "浏览器预览";
            notifyError(e);
        }
    }

    // Each route keeps its own scroll position for the session.
    let lastKey = routeKey(nav.route);
    $effect.pre(() => {
        const key = routeKey(nav.route);
        if (key === lastKey) return;
        if (main)
            memory.set(lastKey, {
                ...memory.get(lastKey),
                scroll: main.scrollTop,
            });
        lastKey = key;
        void tick().then(() => {
            if (main) main.scrollTop = memory.get(key)?.scroll ?? 0;
        });
    });

    // Mounted once (while idle after startup, or on first open) and kept: closing only animates it away,
    // so scroll position survives and the first open is not the frame that builds every row.
    let queueMounted = $state(false);
    $effect.pre(() => {
        if (ui.queueOpen) queueMounted = true;
    });

    async function toggleQueue(value = !ui.queueOpen) {
        ui.queueOpen = value;
        if (desktopAvailable)
            desktopPreferences(null, value).catch(notifyError);
        if (!value) queueTrigger?.focus();
    }

    function keydown(e: KeyboardEvent) {
        const target = e.composedPath()[0];
        const el = target instanceof HTMLElement ? target : null;
        const modal = !!document.querySelector("dialog[open]") || !!ui.menu;
        const action = shortcut(
            e,
            !!el?.closest(
                'input,textarea,select,[contenteditable]:not([contenteditable="false"]),[role="textbox"]',
            ),
            !!el?.closest('button,a,[role="button"],summary'),
            modal,
        );
        if (!action) return;
        if (action === "escape") {
            if (modal) return;
            const layer = escapeLayer({
                queue: ui.queueOpen,
                immersive: ui.immersive,
                toast: !!ui.toast,
            });
            if (layer === "queue") void toggleQueue(false);
            else if (layer === "immersive") ui.immersive = false;
            else if (layer === "toast") dismissToast();
            else return;
        } else if (action === "search") {
            ui.immersive = false;
            void tick().then(() => {
                searchInput?.focus();
                searchInput?.select();
            });
        } else if (action === "lyrics") {
            if (snapshot?.current_track || ui.immersive)
                ui.immersive = !ui.immersive;
        } else if (action === "open") {
            if (desktopAvailable) void control(openFile);
        } else if (action === "toggle") {
            if (snapshot?.current_track)
                void control(
                    snapshot.playback_state === "playing" ? pause : play,
                );
        } else if (action === "previous" || action === "next") {
            if (snapshot?.queue.length) void control(() => queueAction(action));
        }
        e.preventDefault();
    }

    onMount(() => {
        loadPreferences();
        const stopTheme = watchSystemTheme();
        const removeGuard = installNativeMenuGuard(import.meta.env.DEV);
        if (!desktopAvailable) {
            player.connection = "浏览器预览";
            return () => {
                stopTheme();
                removeGuard();
            };
        }
        void desktopPreferences()
            .then((p) => {
                ui.queueOpen = p.queue_open;
            })
            .catch(notifyError);
        void connect();
        void refreshSummary();
        const stopScans = watchScans();
        let disposed = false;
        let timer: ReturnType<typeof setTimeout>;
        async function refresh() {
            try {
                const next = await getPlayerState();
                if (!disposed) {
                    accept(next);
                    player.connected = true;
                }
            } catch (e) {
                if (!disposed) {
                    player.connected = false;
                    player.connection = "连接失败";
                    notifyError(e);
                }
            }
            if (!disposed) timer = setTimeout(refresh, 200);
        }
        void refresh();
        const idle = requestIdleCallback(() => (queueMounted = true), {
            timeout: 4000,
        });
        return () => {
            disposed = true;
            cancelIdleCallback(idle);
            clearTimeout(timer);
            stopScans();
            stopTheme();
            removeGuard();
        };
    });

    let route = $derived(nav.route);
</script>

<div
    class="app"
    class:reduce-motion={ui.reduceMotion}
    class:queue-open={ui.queueOpen}
    class:immersive-open={ui.immersive}
>
    <Sidebar />
    <div class="workspace">
        <TopBar bind:searchInput />
        <div class="content">
            <main class="main" bind:this={main} id="main-content">
                {#if library.progress?.running}
                    <div class="scan-banner" role="status">
                        <span class="spinner" aria-hidden="true"
                        ></span>正在扫描音乐文件夹 · 已检查 {library.progress
                            .visited} · 已导入 {library.progress
                            .imported}<button
                            type="button"
                            class="btn btn-ghost btn-sm"
                            onclick={() => cancelScan().catch(notifyError)}
                            >取消</button
                        >
                    </div>
                {/if}
                {#if route.view === "home"}<HomeView {artwork} />
                {:else if route.view === "albums" || route.view === "artists"}{#key route.view}<GridView
                            kind={route.view}
                        />{/key}
                {:else if route.view === "settings"}<SettingsView />
                {:else}<SongsView />{/if}
            </main>
        </div>
    </div>
    <PlayerBar
        {artwork}
        ontogglequeue={() => toggleQueue()}
        bind:queueTrigger
    />
    {#if ui.immersive}
        <div
            class="immersive-layer"
            transition:fly={{ y: 24, duration: motionMs(220) }}
        >
            <NowPlaying {artwork} onclose={() => (ui.immersive = false)} />
        </div>
    {/if}
    <!-- App-level so it opens over every page, including Now Playing. -->
    {#if queueMounted}
        <button
            type="button"
            class="queue-scrim"
            aria-label="关闭播放队列"
            tabindex="-1"
            inert={!ui.queueOpen}
            onclick={() => toggleQueue(false)}
        ></button>
        <Queue open={ui.queueOpen} onclose={() => toggleQueue(false)} />
    {/if}

    <ContextMenu />
    <Dialog />
    {#if ui.details}{#key ui.details.id}<TrackDetails
                track={ui.details}
                onclose={() => (ui.details = null)}
            />{/key}{/if}
    {#if ui.toast}
        {#key ui.toast.id}
            <div
                class="toast"
                class:error={ui.toast.tone === "error"}
                role={ui.toast.tone === "error" ? "alert" : "status"}
                transition:fly={{ y: 12, duration: motionMs(160) }}
            >
                <span>{ui.toast.message}</span><button
                    type="button"
                    class="icon-btn icon-btn-sm"
                    aria-label="关闭提示"
                    onclick={dismissToast}><X size={15} /></button
                >
            </div>
        {/key}
    {/if}
</div>
<svelte:window onkeydown={keydown} />
