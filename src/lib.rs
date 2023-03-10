#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

lazy_static::lazy_static! {
    static ref LOG_GUARD: tracing_appender::non_blocking::WorkerGuard = {
      let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
      };
      let file_appender = tracing_appender::rolling::daily(home_dir + "/logs/nacos", "nacos.log");
      let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

      tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_level(true)
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

      guard
    };

}

/// log print to console or file
fn init_logger() -> &'static tracing_appender::non_blocking::WorkerGuard {
  &LOG_GUARD
}

#[napi(object)]
pub struct ClientOptions {
  /// Server Addr, e.g. address:port[,address:port],...]
  pub server_addr: String,
  /// Namespace/Tenant
  pub namespace: String,
  /// AppName
  pub app_name: Option<String>,
  /// Username for Auth
  pub username: Option<String>,
  /// Password for Auth
  pub password: Option<String>,
}

mod config;
pub use config::*;

mod naming;
pub use naming::*;

mod plugin;
pub use plugin::*;
