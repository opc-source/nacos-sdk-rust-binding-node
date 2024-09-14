#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

lazy_static::lazy_static! {
    static ref LOG_GUARD: tracing_appender::non_blocking::WorkerGuard = {
      use std::str::FromStr;
      use tracing_subscriber::filter::LevelFilter;
      let log_level = match std::env::var("NACOS_CLIENT_LOGGER_LEVEL") {
        Ok(level) => LevelFilter::from_str(&level).unwrap_or(LevelFilter::INFO),
        Err(_) => LevelFilter::INFO,
      };

      let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
      };
      let file_appender = tracing_appender::rolling::daily(home_dir + "/logs/nacos", "nacos.log");
      let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

      tracing_subscriber::fmt()
        .with_writer(non_blocking)
        // .with_timer(tracing_subscriber::fmt::time::LocalTime::rfc_3339()) // occur `<unknown time>`
        .with_thread_names(true)
        .with_thread_ids(true)
        .with_max_level(log_level)
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
  /// Username for Auth, Login by Http with Token
  pub username: Option<String>,
  /// Password for Auth, Login by Http with Token
  pub password: Option<String>,
  /// Access_Key for Auth, Login by Aliyun Ram
  pub access_key: Option<String>,
  /// Access_Secret for Auth, Login by Aliyun Ram
  pub access_secret: Option<String>,
  /// Signature_Region_Id for Auth, Login by Aliyun Ram
  pub signature_region_id: Option<String>,
  /// naming push_empty_protection, default true
  pub naming_push_empty_protection: Option<bool>,
  /// naming load_cache_at_start, default false
  pub naming_load_cache_at_start: Option<bool>,
}

mod config;
pub use config::*;

mod naming;
pub use naming::*;

mod plugin;
pub use plugin::*;
