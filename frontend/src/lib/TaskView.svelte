<script lang="ts">
    import { default_mapping, Task, mappings } from "../task";
    import ResultView from "./ResultView.svelte";
    import Body from "./Body.svelte";
    import Expandable from "./Expandable.svelte";
    import type { MsgTaskRequest } from "../types";
    import JSONTree from "svelte-json-tree";

    function format_failiure_strat(task: MsgTaskRequest): string {
        if ("discard" === task.failure_strategy) {
            return "Discard on fail.";
        } else {
            return `Retry every ${task.failure_strategy.retry.backoff_millisecs}ms for ${task.failure_strategy.retry.max_tries} times.`;
        }
    }

    export let task: Task;
    $: mapper = mappings[task.task.from.split(".")[0]] ?? default_mapping;
    $: formated_task = mapper.task(task.task);
    $: result_keys = Object.keys(Array.from(task.results.values()).filter((v) => !!v).slice(1).map(mapper.result)[0] ?? {});
    $: all_result_keys = ["Results", ...result_keys]
</script>

<div class="task">
    {#if task.is_incoming}
        <div>Task from remote app: {task.task.from}</div>
    {:else}
        <div>Outgoing task from local app: {task.task.from}</div>
    {/if}
    <table>
        <tr>
            {#each Object.keys(formated_task) as th}
                <th>{th}</th>
            {/each}
        </tr>
        <tr>
            {#each Object.values(formated_task) as value}
                <td>{value}</td>
            {/each}
        </tr>
    </table>
    <table>
        <tr>
        {#each all_result_keys as th}
            <th>{th}</th>
        {/each}
        </tr>
        {#each [...task.results] as [to, result]}
            <tr>
                <td>{to}</td>
            {#if result}
                {#each Object.values(mapper.result(result)) as value}
                    <td>{value}</td>
                {/each}
            {:else}
                <td>Pending result from: {to}</td>
            {/if}
            </tr>
        {/each}
    </table>
    <Expandable>
        <span slot="head">Additional information</span>
        <div>
            ID: {task.task.id}
            <br />
            TTL: {task.task.ttl}
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
    table {
        font-family: Arial, sans-serif;
        border-collapse: collapse;
        width: 100%;
    }

    th, td {
        border: 1px solid #dddddd;
        text-align: left;
        padding: 8px;
    }

    th {
        background-color: #f2f2f2;
    }

    tr:nth-child(even) {
        background-color: #e7e7e7;
    }

    tr:hover {
        background-color: #ddd;
    }
</style>
