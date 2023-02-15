#![deny(clippy::all)]

use napi::{bindgen_prelude::*, threadsafe_function::*};

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
}
