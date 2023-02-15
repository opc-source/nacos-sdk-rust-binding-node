#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};
use std::sync::Arc;

/// Client api of Nacos Naming.
#[napi]
pub struct NacosNamingClient {
  inner: Box<dyn nacos_sdk::api::naming::NamingService>,
}

#[napi]
impl NacosNamingClient {
  #[napi(constructor)]
  pub fn new(client_options: crate::ClientOptions) -> Result<NacosNamingClient> {
    // print to console or file
    crate::log_print_to_console_or_file();

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

      let config_service = nacos_sdk::api::naming::NamingServiceBuilder::new(props)
        .enable_auth_plugin_http()
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosNamingClient {
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

      let config_service = nacos_sdk::api::naming::NamingServiceBuilder::new(props)
        .build()
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

      Ok(NacosNamingClient {
        inner: Box::new(config_service),
      })
    }
  }

  /// Register instance.
  /// If it fails, pay attention to err
  #[napi]
  pub fn register_service(
    &self,
    service_name: String,
    group: String,
    service_instance: NacosServiceInstance,
  ) -> Result<()> {
    Ok(
      self
        .inner
        .register_service(
          service_name,
          Some(group),
          transfer_js_instance_to_rust(&service_instance),
        )
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?,
    )
  }

  /// Deregister instance.
  /// If it fails, pay attention to err
  #[napi]
  pub fn deregister_instance(
    &self,
    service_name: String,
    group: String,
    service_instance: NacosServiceInstance,
  ) -> Result<()> {
    Ok(
      self
        .inner
        .deregister_instance(
          service_name,
          Some(group),
          transfer_js_instance_to_rust(&service_instance),
        )
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?,
    )
  }

  /// Batch register instance, improve interaction efficiency.
  /// If it fails, pay attention to err
  #[napi]
  pub fn batch_register_instance(
    &self,
    service_name: String,
    group: String,
    service_instances: Vec<NacosServiceInstance>,
  ) -> Result<()> {
    let rust_instances = service_instances
      .iter()
      .map(|ins| transfer_js_instance_to_rust(ins))
      .collect();

    Ok(
      self
        .inner
        .batch_register_instance(service_name, Some(group), rust_instances)
        .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?,
    )
  }

  /// Get all instances by service and group.
  /// If it fails, pay attention to err
  #[napi]
  pub fn get_all_instances(
    &self,
    service_name: String,
    group: String,
    clusters: Vec<String>,
    subscribe: bool,
  ) -> Result<Vec<NacosServiceInstance>> {
    let rust_instances = self
      .inner
      .get_all_instances(service_name, Some(group), clusters, subscribe)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    Ok(
      rust_instances
        .iter()
        .map(|ins| transfer_rust_instance_to_js(ins))
        .collect(),
    )
  }

  /// Select instances whether healthy or not.
  /// If it fails, pay attention to err
  #[napi]
  pub fn select_instance(
    &self,
    service_name: String,
    group: String,
    clusters: Vec<String>,
    subscribe: bool,
    healthy: bool,
  ) -> Result<Vec<NacosServiceInstance>> {
    let rust_instances = self
      .inner
      .select_instance(service_name, Some(group), clusters, subscribe, healthy)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    Ok(
      rust_instances
        .iter()
        .map(|ins| transfer_rust_instance_to_js(ins))
        .collect(),
    )
  }

  /// Select one healthy instance.
  /// If it fails, pay attention to err
  #[napi]
  pub fn select_one_healthy_instance(
    &self,
    service_name: String,
    group: String,
    clusters: Vec<String>,
    subscribe: bool,
  ) -> Result<NacosServiceInstance> {
    let rust_instance = self
      .inner
      .select_one_healthy_instance(service_name, Some(group), clusters, subscribe)
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;

    Ok(transfer_rust_instance_to_js(&rust_instance))
  }

  /// Add NacosNamingEventListener callback func, which listen the instance change.
  /// If it fails, pay attention to err
  #[napi]
  pub fn subscribe(
    &self,
    service_name: String,
    group: String,
    clusters: Vec<String>,
    listener: ThreadsafeFunction<Vec<NacosServiceInstance>>,
  ) -> Result<()> {
    self
      .inner
      .subscribe(
        service_name,
        Some(group),
        clusters,
        Arc::new(NacosNamingEventListener {
          func: Arc::new(listener),
        }),
      )
      .map_err(|nacos_err| Error::from_reason(nacos_err.to_string()))?;
    Ok(())
  }
}

pub struct NacosNamingEventListener {
  func: Arc<ThreadsafeFunction<Vec<NacosServiceInstance>>>,
}

impl nacos_sdk::api::naming::NamingEventListener for NacosNamingEventListener {
  fn event(&self, event: Arc<nacos_sdk::api::naming::NamingChangeEvent>) {
    let listen = self.func.clone();

    if event.instances.is_none() {
      return;
    }

    let rust_instances = event.instances.clone().unwrap();

    let js_instances = rust_instances
      .iter()
      .map(|ins| transfer_rust_instance_to_js(ins))
      .collect();

    std::thread::spawn(move || {
      listen.call(Ok(js_instances), ThreadsafeFunctionCallMode::NonBlocking);
    });
  }
}

#[napi(object)]
pub struct NacosServiceInstance {
  /// Instance Id
  pub instance_id: Option<String>,
  /// Ip
  pub ip: String,
  /// Port
  pub port: i32,
  /// Weight
  pub weight: f64,
  /// Healthy or not
  pub healthy: bool,
  /// Enabled ot not
  pub enabled: bool,
  /// Ephemeral or not
  pub ephemeral: bool,
  /// Cluster Name
  pub cluster_name: Option<String>,
  /// Service Name
  pub service_name: Option<String>,
  /// Metadata
  pub metadata: std::collections::HashMap<String, String>,
}

fn transfer_js_instance_to_rust(
  js_instance: &NacosServiceInstance,
) -> nacos_sdk::api::naming::ServiceInstance {
  nacos_sdk::api::naming::ServiceInstance {
    instance_id: js_instance.instance_id.clone(),
    ip: js_instance.ip.clone(),
    port: js_instance.port,
    weight: js_instance.weight,
    healthy: js_instance.healthy,
    enabled: js_instance.enabled,
    ephemeral: js_instance.ephemeral,
    cluster_name: js_instance.cluster_name.clone(),
    service_name: js_instance.service_name.clone(),
    metadata: js_instance.metadata.clone(),
  }
}

fn transfer_rust_instance_to_js(
  rust_instance: &nacos_sdk::api::naming::ServiceInstance,
) -> NacosServiceInstance {
  NacosServiceInstance {
    instance_id: rust_instance.instance_id.clone(),
    ip: rust_instance.ip.clone(),
    port: rust_instance.port,
    weight: rust_instance.weight,
    healthy: rust_instance.healthy,
    enabled: rust_instance.enabled,
    ephemeral: rust_instance.ephemeral,
    cluster_name: rust_instance.cluster_name.clone(),
    service_name: rust_instance.service_name.clone(),
    metadata: rust_instance.metadata.clone(),
  }
}
