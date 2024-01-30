use ethers_core::types::{Bytes, CallFrame, GethTrace, GethTraceFrame, H160, H256};

#[derive(Debug, Clone)]
pub struct TraceLogData {
    address: H160,
    topics: Vec<H256>,
    data: Bytes,
}

impl TraceLogData {
    pub fn topics(&self) -> Vec<H256> {
        self.topics.clone()
    }

    pub fn data(&self) -> Bytes {
        self.data.clone()
    }
    
    pub fn address(&self) -> H160 {
        self.address
    }
}

fn get_logs_on_frame(call_frame: CallFrame, mylogs: &mut Vec<TraceLogData>) {
    if let Some(call_frame_logs) = call_frame.logs {
        for call_log_frame in call_frame_logs {
            if let Some(topics) = call_log_frame.topics {
                mylogs.push(TraceLogData {
                    address: call_log_frame.address.unwrap(),
                    topics,
                    data: call_log_frame.data.unwrap(),
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