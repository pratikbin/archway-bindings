use cosmwasm_schema::cw_serde;

#[cw_serde]
#[derive(Default)]
pub struct PageRequest {
    key: Option<String>,
    offset: Option<u64>,
    limit: Option<u64>,
    count_total: Option<bool>,
    reverse: Option<bool>,
}

impl PageRequest {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn count_total(mut self) -> Self {
        self.count_total = Some(true);
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = Some(true);
        self
    }
}

#[cw_serde]
pub struct PageResponse {
    pub next_key: Option<String>,
    pub total: Option<u64>,
}
