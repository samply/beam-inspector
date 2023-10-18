
export interface MsgEmpty {
    from: string
}

export type WorkStatus = "claimed" | "tempfailed" | "permfailed" | "succeeded"

export interface MsgTaskResult extends MsgEmpty {
    to: Array<string>,
    task: string,
    status: WorkStatus,
    body: any,
    metadata: any
}


export interface MsgTaskRequest extends MsgEmpty {
    id: string,
    to: Array<string>,
    body: any,
    metadata: any,
    ttl: string,
    failure_strategy: {
        retry: {
            backoff_millisecs: number,
            max_tries: number,
        }
    } | "discard"
}

export type Msg = MsgEmpty | MsgTaskRequest | MsgTaskResult

export type MonitoringUpdate = {
    request: {
        uri: string,
        method: string,
        headers: Map<string, string>,
        json: Msg
    }
} | {
    response: {
        status: number,
        headers: Map<string, string>,
        json: Array<MsgTaskResult> | Array<MsgTaskRequest>
    }
}
