use async_graphql::{BatchRequest, ObjectType, Schema, SubscriptionType};
use serde_json::Value;
use tauri::{AppHandle, Runtime, State};

use crate::Result;

#[tauri::command]
pub(crate) async fn graphql<R, Q, Mutation, Subscription>(
    _app: AppHandle<R>,
    _schema: State<'_, Schema<Q, Mutation, Subscription>>,
    payload: Value,
) -> Result<()>
where
    R: Runtime,
    Q: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    let _req: BatchRequest = serde_json::from_value(payload)?;

    Ok(())
}
