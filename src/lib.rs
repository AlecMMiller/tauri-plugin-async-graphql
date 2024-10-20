use async_graphql::{ObjectType, Schema, SubscriptionType};
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AsyncGraphql;
#[cfg(mobile)]
use mobile::AsyncGraphql;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the async-graphql APIs.
pub trait AsyncGraphqlExt<R: Runtime> {
    fn async_graphql(&self) -> &AsyncGraphql<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AsyncGraphqlExt<R> for T {
    fn async_graphql(&self) -> &AsyncGraphql<R> {
        self.state::<AsyncGraphql<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R, Query, Mutation, Subscription>(
    schema: Schema<Query, Mutation, Subscription>,
) -> TauriPlugin<R>
where
    R: Runtime,
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    Builder::new("async-graphql")
        .invoke_handler(tauri::generate_handler![commands::graphql])
        .setup(|app, api| {
            #[cfg(mobile)]
            let async_graphql = mobile::init(app, api)?;
            #[cfg(desktop)]
            let async_graphql = desktop::init(app, api)?;
            app.manage(async_graphql);
            app.manage(schema);
            Ok(())
        })
        .build()
}
