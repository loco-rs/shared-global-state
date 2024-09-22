```toml
libvips = "1.7.0"
lazy_static = "1.5.0"
```








https://crates.io/crates/libvips
[ld issue](https://github.com/olxgroup-oss/libvips-rust-bindings/issues/107)


add vips 



lazy_static! {
    pub static ref VIPS: Arc<VipsApp> = {
        let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
        //set number of threads in libvips's threadpool
        app.concurrency_set(2);
        Arc::new(app)
    };
}




```rust
impl worker::AppWorker<DownloadWorkerArgs> for DownloadWorker {
    fn build(ctx: &AppContext) -> Self {
        Self {
            ctx: ctx.clone(),
            vips: app::VIPS.clone(),
        }
    }
}
```



remove this 






2024-09-20T07:03:11.891281Z DEBUG http-request: tower_http::trace::on_response: finished processing request latency=1 ms status=200 http.method=GET http.uri=/api/notes http.version=HTTP/1.1 http.user_agent=curl/8.7.1 environment=development request_id=81d4738c-3cd4-45f4-9c67-8abb1826712d
================================================
rescaling image
thread 'tokio-runtime-worker' panicked at src/workers/downloader.rs:35:58:
called `Result::unwrap()` on an `Err` value: InitializationError("Could not initialise VipsImage from file")
stack backtrace:
   0: rust_begin_unwind
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
   1: core::panicking::panic_fmt
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
   2: core::result::unwrap_failed
             at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1654:5
             
             
             
             
             
             
=curl/8.7.1 environment=development request_id=e310b28c-d567-4edf-9ef7-f08fc4bf0272
worker off: Ok(())
================================================
rescaling image
2024-09-20T07:03:49.232088Z DEBUG http-request: tower_http::trace::on_response: finished processing request latency=1 ms status=200 http.method=GET http.uri=/api/notes http.version=HTTP/1.1 http.user_agent=curl/8.7.1 environment=development request_id=e310b28c-d567-4edf-9ef7-f08fc4bf0272
Great Success!
================================================





## Sharing in controllers


Set it

async fn after_routes(router: axum::Router, _ctx: &AppContext) -> Result<axum::Router> {
    Ok(router.layer(Extension(VIPS.clone())))
}



And get it

#[debug_handler]
pub async fn list(
    State(ctx): State<AppContext>,
    Extension(vips): Extension<Arc<VipsApp>>,
) -> Result<Response> {
