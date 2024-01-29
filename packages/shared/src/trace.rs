use ethers_core::types::{GethTrace, H160, H256, Bytes};

#[derive(Debug, Clone)]
pub struct TraceExtractor {
    address: H160,
    topics: Vec<H256>,
    data: Bytes,
}

impl TraceExtractor {
    pub fn new(frame: GethTrace) -> TraceExtractor {
        TraceExtractor {}
    }

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
