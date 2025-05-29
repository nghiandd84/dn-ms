#[derive(Default)]
pub struct HttpGatewayCtx {
    pub request_id: Option<String>,
    pub user_id: Option<String>,
}

impl HttpGatewayCtx {
    pub fn new() -> Self {
        Self {
            request_id: None,
            user_id: None,
        }
    }
}
