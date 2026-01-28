#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
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
