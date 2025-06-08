extern crate self as backend;

mod handlers;
pub mod schema;

pub struct MaybeMe {
    pub now: chrono::DateTime<chrono::Utc>,
    pub ud: Option<ft_sdk::UserData>,
    pub conn: ft_sdk::Connection,
}

impl ft_sdk::FromRequest for MaybeMe {
    fn from_request(req: &http::Request<serde_json::Value>) -> Result<Self, ft_sdk::Error> {
        let cookie: ft_sdk::Cookie<{ ft_sdk::auth::SESSION_KEY }> =
            ft_sdk::FromRequest::from_request(req)?;
        let mut conn = ft_sdk::FromRequest::from_request(req)?;
        let ud = ft_sdk::auth::ud(cookie, &mut conn)?;

        Ok(MaybeMe {
            now: ft_sdk::FromRequest::from_request(req)?,
            ud,
            conn,
        })
    }
}
