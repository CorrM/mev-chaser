use ethers_core::{
    abi::RawLog,
    types::{Address, CallFrame, GethTrace, GethTraceFrame},
};

#[derive(Debug, Clone)]
pub struct TraceLogData {
    address: Address,
    raw_log: RawLog,
}

impl TraceLogData {
    pub fn address(&self) -> Address {
        self.address
    }

    pub fn raw_log(&self) -> &RawLog {
        &self.raw_log
    }
}

fn get_logs_on_frame(call_frame: CallFrame, mylogs: &mut Vec<TraceLogData>) {
    if let Some(call_frame_logs) = call_frame.logs {
        for call_log_frame in call_frame_logs {
            if let Some(topics) = call_log_frame.topics {
                mylogs.push(TraceLogData {
                    address: call_log_frame.address.unwrap(),
                    raw_log: RawLog {
                        topics,
                        data: call_log_frame.data.unwrap().to_vec(),
                    },
                })
            }
        }
    };

    if let Some(calls) = call_frame.calls {
        for _call_frame in calls {
            get_logs_on_frame(_call_frame, mylogs)
        }
    }
}

pub fn get_trace_all_logs(frame: GethTrace) -> Vec<TraceLogData> {
    let mut mylogs: Vec<TraceLogData> = Vec::new();

    let trace_frame: GethTraceFrame = match frame {
        GethTrace::Known(a) => a,
        GethTrace::Unknown(_) => return mylogs,
    };

    let call_frame: CallFrame = match trace_frame {
        GethTraceFrame::CallTracer(a) => a,
        _ => return mylogs,
    };

    get_logs_on_frame(call_frame, &mut mylogs);

    mylogs
}
