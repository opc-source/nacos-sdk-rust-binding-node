#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*, tokio::sync::Mutex};
use std::sync::Arc;


/// Client api of Nacos Config.
#[napi]
pub struct NacosConfigClient {
  inner: Arc<Mutex<dyn nacos_sdk::api::config::ConfigService + Send + Sync + 'static>>,
}

#[napi]
impl NacosConfigClient {
  /// Build a Config Client.
  #[napi(constructor)]
  pub fn new(
    client_options: crate::ClientOptions,
    config_filter: Option<
      ThreadsafeFunction<(
        Option<crate::NacosConfigReq>,
        Option<crate::NacosConfigResp>,
      )>,
    >,
  ) -> Result<NacosConfigClient> {
    // print to console or file
    crate::log_print_to_console_or_file();

    let props = nacos_sdk::api::props::ClientProps::new()
      .server_addr(client_options.server_addr)
      .namespace(client_options.namespace)
      .app_name(
        client_options
          .app_name
          .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
      );

    // need enable_auth_plugin_http with username & password
    let is_enable_auth = client_options.username.is_some() && client_options.password.is_some();

    let props = if is_enable_auth {
      props
        .auth_username(client_options.username.unwrap())
        .auth_password(client_options.password.unwrap())
    } else {
      props
    };

    let config_service_builder = if is_enable_auth {
      nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_http()
    } else {
      nacos_sdk::api::config::ConfigServiceBuilder::new(props)
    };

    let config_service_builder = if config_filter.is_some() {
      config_service_builder.add_config_filter(Box::new(crate::NacosConfigFilter {
        func: Arc::new(config_filter.unwrap()),
      }))
    } else {
      config_service_builder
    };

    let config_service = config_service_builder
      .build()
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    Ok(NacosConfigClient {
      inner: Arc::new(Mutex::new(config_service)),
    })
  }

  /// Get config's content.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn get_config(&self, data_id: String, group: String) -> Result<String> {
    let resp = self.get_config_resp(data_id, group).await?;
    Ok(resp.content)
  }

  /// Get NacosConfigResponse.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn get_config_resp(&self, data_id: String, group: String) -> Result<NacosConfigResponse> {

    let mut inner = self.inner.lock().await;
    let config_resp = inner.get_config(data_id, group).map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;
    Ok(transfer_conf_resp(config_resp))
   

    // let config_resp = self
    //   .inner
    //   .get_config(data_id, group)
    //   .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    // Ok(transfer_conf_resp(config_resp))
  }

  /// Publish config.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn publish_config(
    &self,
    data_id: String,
    group: String,
    content: String,
  ) -> Result<bool> {
    let mut inner = self.inner.lock().await;
    inner
      .publish_config(data_id, group, content, None)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))
  }

  /// Remove config.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn remove_config(&self, data_id: String, group: String) -> Result<bool> {
    let mut inner = self.inner.lock().await;
    inner
      .remove_config(data_id, group)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))
  }

  /// Add NacosConfigChangeListener callback func, which listen the config change.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn add_listener(
    &self,
    data_id: String,
    group: String,
    listener: ThreadsafeFunction<NacosConfigResponse>,
  ) -> Result<()> {
    let mut inner = self.inner.lock().await;
    inner
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
    data_id: config_resp.data_id().to_string(),
    group: config_resp.group().to_string(),
    content: config_resp.content().to_string(),
    content_type: config_resp.content_type().to_string(),
    md5: config_resp.md5().to_string(),
  }
}
