#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use std::sync::Arc;

/// Client api of Nacos Config.
#[napi]
pub struct NacosConfigClient {
  inner: nacos_sdk::api::config::ConfigService,
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
    let props = nacos_sdk::api::props::ClientProps::new()
      .server_addr(client_options.server_addr)
      .namespace(client_options.namespace)
      .app_name(
        client_options
          .app_name
          .unwrap_or(nacos_sdk::api::constants::UNKNOWN.to_string()),
      )
      .config_load_cache_at_start(client_options.config_load_cache_at_start.unwrap_or(false));

    // need enable_auth_plugin_http with username & password
    let is_enable_auth_http =
      client_options.username.is_some() && client_options.password.is_some();
    // need enable_auth_plugin_aliyun with access_key & access_secret
    let is_enable_auth_aliyun =
      client_options.access_key.is_some() && client_options.access_secret.is_some();

    let props = if is_enable_auth_http {
      props
        .auth_username(client_options.username.unwrap())
        .auth_password(client_options.password.unwrap())
    } else if is_enable_auth_aliyun {
      props
        .auth_access_key(client_options.access_key.unwrap())
        .auth_access_secret(client_options.access_secret.unwrap())
        .auth_signature_region_id(client_options.signature_region_id.unwrap())
    } else {
      props
    };

    let config_service_builder = if is_enable_auth_http {
      nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_http()
    } else if is_enable_auth_aliyun {
      nacos_sdk::api::config::ConfigServiceBuilder::new(props).enable_auth_plugin_aliyun()
    } else {
      nacos_sdk::api::config::ConfigServiceBuilder::new(props)
    };

    let config_service_builder = if let Some(filter) = config_filter {
      config_service_builder.add_config_filter(Box::new(crate::NacosConfigFilter {
        func: Arc::new(filter),
      }))
    } else {
      config_service_builder
    };

    let config_service = crate::get_runtime().block_on(async {
      config_service_builder
        .build()
        .await
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))
    })?;

    Ok(NacosConfigClient {
      inner: config_service,
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
  pub async fn get_config_resp(
    &self,
    data_id: String,
    group: String,
  ) -> Result<NacosConfigResponse> {
    let config_resp = self
      .inner
      .get_config(data_id, group)
      .await
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;
    Ok(transfer_conf_resp(config_resp))
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
    self
      .inner
      .publish_config(data_id, group, content, None)
      .await
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))
  }

  /// Remove config.
  /// If it fails, pay attention to err
  #[napi]
  pub async fn remove_config(&self, data_id: String, group: String) -> Result<bool> {
    self
      .inner
      .remove_config(data_id, group)
      .await
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
    self
      .inner
      .add_listener(
        data_id,
        group,
        Arc::new(NacosConfigChangeListener {
          func: Arc::new(listener),
        }),
      )
      .await
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;
    Ok(())
  }

  /// Remove NacosConfigChangeListener callback func, but noop....
  /// The logic is not implemented internally, and only APIs are provided as compatibility.
  /// Users maybe do not need it? Not removing the listener is not a big problem, Sorry!
  #[napi]
  pub async fn remove_listener(
    &self,
    _data_id: String,
    _group: String,
    _listener: ThreadsafeFunction<NacosConfigResponse>,
  ) -> Result<()> {
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
