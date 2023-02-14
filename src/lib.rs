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

/// Client api of Nacos Config.
#[napi]
pub struct NacosConfigClient {
  inner: Box<dyn nacos_sdk::api::config::ConfigService>,
}

#[napi]
impl NacosConfigClient {
  #[napi(constructor)]
  pub fn new(client_options: ClientOptions) -> Result<NacosConfigClient> {
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
    if client_options.username.is_some() && client_options.password.is_some() {
      let props = nacos_sdk::api::props::ClientProps::new()
        .server_addr(client_options.server_addr)
        .namespace(client_options.namespace)
        .app_name(
          client_options
            .app_name
            .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
        )
        .auth_username(client_options.username.unwrap())
        .auth_password(client_options.password.unwrap());

      let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        .enable_auth_plugin_http()
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosConfigClient {
        inner: Box::new(config_service),
      })
    } else {
      let props = nacos_sdk::api::props::ClientProps::new()
        .server_addr(client_options.server_addr)
        .namespace(client_options.namespace)
        .app_name(
          client_options
            .app_name
            .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
        );

      let config_service = nacos_sdk::api::config::ConfigServiceBuilder::new(props)
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosConfigClient {
        inner: Box::new(config_service),
      })
    }
  }

  /// Get config's content.
  #[napi]
  pub fn get_config(&mut self, data_id: String, group: String) -> Result<String> {
    Ok(self.get_config_resp(data_id, group)?.content)
  }

  /// Get NacosConfigResponse.
  #[napi]
  pub fn get_config_resp(&mut self, data_id: String, group: String) -> Result<NacosConfigResponse> {
    let config_resp = self
      .inner
      .get_config(data_id, group)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    Ok(transfer_conf_resp(config_resp))
  }

  /// Publish config.
  #[napi]
  pub fn publish_config(
    &mut self,
    data_id: String,
    group: String,
    content: String,
  ) -> Result<bool> {
    Ok(
      self
        .inner
        .publish_config(data_id, group, content, None)
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?,
    )
  }

  /// Remove config.
  #[napi]
  pub fn remove_config(&mut self, data_id: String, group: String) -> Result<bool> {
    Ok(
      self
        .inner
        .remove_config(data_id, group)
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?,
    )
  }

  /// Add NacosConfigChangeListener callback func, which listen the config change.
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

    let conf_resp = transfer_conf_resp(config_resp);

    std::thread::spawn(move || {
      listen.call(Ok(conf_resp), ThreadsafeFunctionCallMode::NonBlocking);
    });
  }
}

fn transfer_conf_resp(config_resp: nacos_sdk::api::config::ConfigResponse) -> NacosConfigResponse {
  NacosConfigResponse {
    namespace: config_resp.namespace().to_string(),
    data_id: config_resp.data_id().to_string().to_string(),
    group: config_resp.group().to_string(),
    content: config_resp.content().to_string(),
    content_type: config_resp.content_type().to_string(),
    md5: config_resp.md5().to_string(),
  }
}
