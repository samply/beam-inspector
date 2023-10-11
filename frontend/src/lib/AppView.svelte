<script lang="ts">
    import { derived } from "svelte/store";
    import { tasks } from "../store";
    import TaskView from "./TaskView.svelte";
    export let params: { app: string };
    let filtered_tasks = [];
    $: {
        params;
        filtered_tasks = $tasks.filter(t => t.task.from.startsWith(params.app));
    }
</script>

<h1>{ params.app }</h1>
<div>
    {#each filtered_tasks as task}
        <TaskView {task} />
    {/each}
</div>
<!-- Render tasks related to that app -->
