#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  bindgen_prelude::*, threadsafe_function::*, JsGlobal, JsNull, JsObject,
  JsUndefined, Property,
};
use std::sync::Arc;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

static INIT_ONCE: std::sync::Once = std::sync::Once::new();

#[napi]
pub struct NacosConfigClient {
  inner: Box<dyn nacos_sdk::api::config::ConfigService>,
}

#[napi]
impl NacosConfigClient {
  #[napi(constructor)]
  pub fn new(
    server_addr: String,
    namespace: String,
    app_name: String,
    username: Option<String>,
    password: Option<String>,
  ) -> Result<NacosConfigClient> {
    // print to console or file
    INIT_ONCE.call_once(|| {
      let home_dir = match std::env::var("HOME") {
        Ok(dir) => dir,
        Err(_) => "/tmp".to_string(),
      };
      let file_appender = tracing_appender::rolling::daily(home_dir + "/logs/nacos", "nacos.log");
      let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

      tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_thread_names(true)
        .with_file(true)
        .with_level(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
    });

    // enable_auth_plugin_http with username & password
    if username.is_some() && password.is_some() {
      let props = nacos_sdk::api::props::ClientProps::new()
        .server_addr(server_addr)
        .namespace(namespace)
        .app_name(app_name)
        .auth_username(username.unwrap())
        .auth_password(password.unwrap());

      let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        .enable_auth_plugin_http()
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosConfigClient {
        inner: Box::new(config_service),
      })
    } else {
      let props = nacos_sdk::api::props::ClientProps::new()
        .server_addr(server_addr)
        .namespace(namespace)
        .app_name(app_name);

      let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosConfigClient {
        inner: Box::new(config_service),
      })
    }
  }

  #[napi]
  pub fn get_config(&mut self, data_id: String, group: String) -> Result<String> {
    let config_resp = self.inner.get_config(data_id, group);

    Ok(
      config_resp
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?
        .content()
        .to_string(),
    )
  }

  #[napi]
  pub fn add_listener(
    &mut self,
    data_id: String,
    group: String,
    listener: ThreadsafeFunction<NacosConfigResponse>,
  ) -> Result<()> {
    self
      .inner
      .add_listener(
        data_id,
        group,
        Arc::new(NacosConfigChangeListener {
          func: Arc::new(listener),
        }),
      )
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;
    Ok(())
  }
}

#[napi(object)]
pub struct NacosConfigResponse {
  /// Namespace/Tenant
  pub namespace: String,
  /// DataId
  pub data_id: String,
  /// Group
  pub group: String,
  /// Content
  pub content: String,
  /// Content's Type; e.g. json,properties,xml,html,text,yaml
  pub content_type: String,
  /// Content's md5
  pub md5: String,
}

pub struct NacosConfigChangeListener {
  func: Arc<ThreadsafeFunction<NacosConfigResponse>>,
}

impl nacos_sdk::api::config::ConfigChangeListener for NacosConfigChangeListener {
  fn notify(&self, config_resp: nacos_sdk::api::config::ConfigResponse) {
    let listen = self.func.clone();

    let conf_resp = NacosConfigResponse {
      namespace: config_resp.namespace().to_string(),
      data_id: config_resp.data_id().to_string().to_string(),
      group: config_resp.group().to_string(),
      content: config_resp.content().to_string(),
      content_type: config_resp.content_type().to_string(),
      md5: config_resp.md5().to_string(),
    };

    std::thread::spawn(move || {
      listen.call(Ok(conf_resp), ThreadsafeFunctionCallMode::NonBlocking);
    });
  }
}
