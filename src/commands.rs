use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AsyncGraphqlExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.async_graphql().ping(payload)
}
