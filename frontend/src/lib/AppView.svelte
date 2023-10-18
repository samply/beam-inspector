<script lang="ts">
    import type { Task } from "../task";
    import EventList from "./EventList.svelte";
    export let params: { app: string };
    $: filter = (t: Task) => t.task.from.startsWith(params.app) || t.task.to.some(rec => rec.startsWith(params.app));
</script>

<h1>{ params.app }</h1>
<div>
    <EventList default_filter={filter}/>
</div>
<!-- Render tasks related to that app -->
