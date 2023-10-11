<script lang="ts">
    import { default_mapping, Task, mappings } from "../task";
    import ResultView from "./ResultView.svelte";
    import Body from "./Body.svelte";
    import Expandable from "./Expandable.svelte";
    import type { MsgTaskRequest } from "../types";
    import JSONTree from 'svelte-json-tree';

    function format_failiure_strat(task: MsgTaskRequest): string {
        if ("discard" === task.failure_strategy) {
            return "Discard on fail."
        } else {
            return `Retry every ${task.failure_strategy.retry.backoff_millisecs}ms for ${task.failure_strategy.retry.max_tries} times.`
        }
    }

    export let task: Task;
    $: mapper = mappings[task.task.from.split(".")[0]] ?? default_mapping;
</script>

<div class="task">
    {#if task.is_incoming}
    <div>Task from remote app: {task.task.from}</div>
    {:else}
    <div>Outgoing task from local app: {task.task.from}</div>
    {/if}
    <table>
        {#each [...Object.entries(mapper.task(task.task))] as [key, value]}
        <tr>{value}</tr>
        {/each}
    </table>
    <!-- <Body json={task.task.body} /> -->
    <span>Results:</span>
    <ul>
        {#each [...task.results] as [to, result]}
            <li>
                {#if result}
                    <!-- <ResultView {result} /> -->
                    <table>
                        {#each [...Object.entries(mapper.result(result))] as [key, value]}
                        <tr>{value}</tr>
                        {/each}
                    </table>
                {:else}
                    <span>Pending result from: {to}</span>
                {/if}
            </li>
        {/each}
    </ul>
    <Expandable>
        <span slot="head">Show extra info</span>
        <div>
            Id: {task.task.id}
            <br />
            Ttl: {task.task.ttl}
            <br />
            Failiure stratagy: {format_failiure_strat(task.task)}
            <br />
            Metadata:
            <JSONTree value={task.task.metadata} />
        </div>
    </Expandable>
</div>

<style>
.task {
    background-color: var(--color-gray);
    border-radius: 2rem;
    margin: 1rem;
    text-align: start;
    padding: 1rem;
}
li {
    list-style: none;
}
ul {
    padding-left: 20px;
}
</style>
