use axum::http::{Method, Uri};

#[derive(Clone, Debug)]
pub struct Ctx {
	pub user_id: u64,
}
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }

    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}

#[derive(Clone, Debug, Default)]
pub struct RequestData {
    pub(crate) method: Method,
    pub(crate) uri: Uri,
    pub(crate) ctx: Option<Ctx>,
}