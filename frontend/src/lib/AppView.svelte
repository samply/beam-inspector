<script lang="ts">
    import EventList from "./EventList.svelte";
    export let params: { app: string };
    let filter = t => t.task.from.startsWith(params.app);
    $: {
        params;
        filter = t => t.task.from.startsWith(params.app);
    }
</script>

<h1>{ params.app }</h1>
<div>
    <EventList default_filter={filter}/>
</div>
<!-- Render tasks related to that app -->
