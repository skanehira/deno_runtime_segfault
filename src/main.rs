use std::rc::Rc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use deno_core::{Extension, FsModuleLoader, JsRuntime, RuntimeOptions};
use deno_runtime::permissions::Permissions;
use deno_runtime::js;
use futures::executor::block_on;

pub async fn handler() -> Result<impl IntoResponse, String> {
    block_on(async {
        let extensions: Vec<Extension> = vec![
            deno_web::init::<Permissions>(deno_web::BlobStore::default(), None),
            deno_crypto::init(None),
        ];

        let _ = JsRuntime::new(RuntimeOptions {
            module_loader: Some(Rc::new(FsModuleLoader)),
            startup_snapshot: Some(js::deno_isolate_init()),
            source_map_getter: None,
            get_error_class_fn: None,
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            extensions,
            ..Default::default()
        });
    });
    Ok(StatusCode::OK)
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/test", get(handler));
    let addr = format!("0.0.0.0:{}", 7777);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
