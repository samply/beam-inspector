<script lang="ts">
    import AppView from "./lib/AppView.svelte";
    import EventList from "./lib/EventList.svelte";
    import Nav from "./lib/Nav.svelte";
    import Router from 'svelte-spa-router'
    import { mappings } from "./task";
    const routes = {
        '/': EventList,
        '/app/:app': AppView,
    }
</script>

<main>
    <aside>
        <Nav apps={Object.keys(mappings)} />
    </aside>

    <div class="content">
        <Router {routes} />
    </div>
</main>

<style>
    main {
        display: grid;
        grid-template-columns: 1fr 4fr;
        height: 100%;
    }
    .content {
        padding: 1.5rem;
    }
    aside {
        background-color: whitesmoke;
        height: 100%;
    }
</style>
