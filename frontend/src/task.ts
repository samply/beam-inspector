import type { MsgTaskRequest, MsgTaskResult } from "./types";

export class Task {
    task: MsgTaskRequest;
    /** if true the request is coming from the broker if false the request is coming from an app in the proxies network */
    is_incoming: boolean;
    results: Map<string, MsgTaskResult | null>
    
    constructor(task: MsgTaskRequest, is_incoming: boolean) {
        this.task = task;
        this.is_incoming = is_incoming;
        this.results = new Map(task.to.map(t => [t, null]));
    }
    
    add_result(result: MsgTaskResult) {
        if (this.task.to.includes(result.from)) {
            this.results.set(result.from, result);
        } else {
            console.log("Result is not from this task");
        }
    }
    
}

function default_task_formatter(task: MsgTaskRequest) {
    return {"body": task.body}
}

function default_result_formatter(result: MsgTaskResult) {
    return {"body": result.body}
}

// TODO: Figure out what signature we want to have here
// Ideas
// Task.task.body => {"collum_name": html}
// Task.results[i] => {"collum_name": html}
const mappings_data: Record<string, {task?: string, result?: string}> = await (await fetch("/mappings")).json();
for (const key in mappings_data) {
    mappings_data[key].task = eval(`(${mappings_data[key].task ?? default_task_formatter})`);
    mappings_data[key].result = eval(`(${mappings_data[key].result ?? default_result_formatter})`);
}

type Mapping = {
    task: (task: MsgTaskRequest) => Record<string, string>,
    result: (msg: MsgTaskResult) => Record<string, string>
}
export const mappings: Record<string, Mapping> = mappings_data as any;

export const default_mapping: Mapping = {
    task: default_task_formatter,
    result: default_result_formatter
}
