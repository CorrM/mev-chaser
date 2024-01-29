use super::{DebugTraceCallNodeProvider, NormalNodeProvider};

#[derive(Clone)]
pub enum NodeProviderKind {
    Normal(NormalNodeProvider),
    DebugTraceCall(DebugTraceCallNodeProvider),
}