<script lang="ts">
    import { tasks } from "../store";
    import type { Task } from "../task";
    import TaskView from "./TaskView.svelte";
    import ModeToggle from "./ModeToggle.svelte";

    export let default_filter: (t: Task) => boolean = _ => true;

    let from_filter_value = "";
    $: from_filter = (task: Task) => {
        // If from_filter is not an empty string or undefined always return true otherwise check from field
        return !from_filter_value || task.task.from.includes(from_filter_value)
    };

    let done_filter = false;
    $: hide_done_filter = (task: Task) => {
        return !done_filter || !Array.from(task.results.values()).every(result => result?.status === "succeeded")
    };
    
    // This should proably be a toggle?
    let incoming_filter_status = false;
    $: incoming_outgoing_filter = (task: Task) => {
        return incoming_filter_status === task.is_incoming
    };

    $: filters = [from_filter, hide_done_filter, incoming_outgoing_filter, default_filter];
    // Update filtered tasks whenever a new task or a new filter gets added
    $: filtered_tasks = $tasks.filter(task => filters.every(filter => filter(task)))
</script>

<header>
    <h2>Tasks</h2>
    <ModeToggle bind:incoming={incoming_filter_status}/>
</header>
<div>
    <div class="settings">
        <button on:click={() => $tasks = []}>Clear Tasks</button>
        <button class:active={done_filter} on:click={() => done_filter = !done_filter}>Hide completed</button>
        <span>Filter from:</span>
        <input type="text" class="task-filter" bind:value={from_filter_value}>
    </div>
    <ul>
        {#each filtered_tasks as task}
            <li>
                <TaskView {task} />
            </li>
        {/each}
    </ul>
</div>

<style>
    header {
        margin: 1rem 0;
    }
    li {
        list-style: none;
    }
    button {
        background-color: var(--color-gray);
        outline: none;
    }
    .task-filter {
        border-radius: 1rem;
        font-size: inherit;
        padding: .2rem .4rem;
    }
    .settings {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0 3cqw;
        gap: 1rem;
    }
    .active {
        outline: solid var(--color-blue) 2px;
    }
</style>
