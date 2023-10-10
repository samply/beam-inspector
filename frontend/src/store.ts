import { get, writable } from "svelte/store";
import { Task } from "./task";
import type { MonitoringUpdate, MsgTaskRequest, MsgTaskResult } from "./types";

export const tasks = writable<Task[]>([])
        
function append_result(result: MsgTaskResult) {
    let task: Task = get(tasks).find((t) => result.task === t.task.id);
    if (task) {
        task.add_result(result);
    } else {
        console.log("Could not find task for result", result);
    }
}

const sse_stream = new EventSource("/events");

sse_stream.addEventListener("message", (e) => {
    // We cant push as we need svelte to understand that we updated this and need to rerender
    let update = JSON.parse(e.data) as MonitoringUpdate;
    if ("request" in update) {
        let msg = update.request.json;
        let request = update.request;
        // We have a MsgTaskRequest from some App in the proxies network to the beam network
        if ("id" in msg) {
            tasks.update((ts) => [...ts, new Task(msg as MsgTaskRequest, false)]);
        // We have a MsgTaskResult from some App in the proxies network
        } else if ("status" in msg) {
            append_result(msg);
        } else {
            console.log("Ignoring:", request);
        }
    } else if ("response" in update) {
        let msg = update.response.json;
        let response = update.response;

        if (Array.isArray(msg)) {
            msg.forEach(append_result);
        } else if ("id" in msg) {
            tasks.update((ts) => [...ts, new Task(msg as MsgTaskRequest, true)]);
        } else if ("status" in msg) {
            append_result(msg);
        } else {
            console.log("Ignoring:", response);
        }
    } else {
        console.log("Unknown update", update);
    }
})

sse_stream.addEventListener("error", (e) => {
    console.log("Error during SSE", e);
    if (sse_stream.CLOSED) {
        alert("SSE connection closed");
    }
});
