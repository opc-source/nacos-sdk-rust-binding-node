use napi::threadsafe_function::*;
use std::sync::Arc;

/// [`config_filter`] It is an advanced feature that does not need to be used by default;
/// For example: 1. Encrypt ConfigReq.content value and then request; 2. Decrypt ConfigResp.content to get the value.
pub struct NacosConfigFilter {
  pub(crate) func: Arc<ThreadsafeFunction<(Option<NacosConfigReq>, Option<NacosConfigResp>)>>,
}

impl nacos_sdk::api::plugin::ConfigFilter for NacosConfigFilter {
  fn filter(
    &self,
    config_req: Option<&mut nacos_sdk::api::plugin::ConfigReq>,
    config_resp: Option<&mut nacos_sdk::api::plugin::ConfigResp>,
  ) {
    if let Some(config_req) = config_req {
      let js_config_req = NacosConfigReq {
        data_id: config_req.data_id.clone(),
        group: config_req.group.clone(),
        namespace: config_req.namespace.clone(),
        content: config_req.content.clone(),
        encrypted_data_key: config_req.encrypted_data_key.clone(),
      };

      let (tx, rx) = std::sync::mpsc::channel();
      self.func.clone().call_with_return_value(
        Ok((Some(js_config_req), None)),
        ThreadsafeFunctionCallMode::Blocking,
        move |(after_js_config_req, _after_js_config_resp): (
          Option<NacosConfigReq>,
          Option<NacosConfigResp>,
        )| {
          let after_js_config_req = after_js_config_req.unwrap();
          let _ = tx.send(after_js_config_req);
          Ok(())
        },
      );

      let ret = rx.recv().unwrap();
      config_req.data_id = ret.data_id;
      config_req.group = ret.group;
      config_req.namespace = ret.namespace;
      config_req.content = ret.content;
      config_req.encrypted_data_key = ret.encrypted_data_key;
    }

    if let Some(config_resp) = config_resp {
      let js_config_resp = NacosConfigResp {
        data_id: config_resp.data_id.clone(),
        group: config_resp.group.clone(),
        namespace: config_resp.namespace.clone(),
        content: config_resp.content.clone(),
        encrypted_data_key: config_resp.encrypted_data_key.clone(),
      };

      let (tx, rx) = std::sync::mpsc::channel();
      self.func.clone().call_with_return_value(
        Ok((None, Some(js_config_resp))),
        ThreadsafeFunctionCallMode::Blocking,
        move |(_after_js_config_req, after_js_config_resp): (
          Option<NacosConfigReq>,
          Option<NacosConfigResp>,
        )| {
          let after_js_config_resp = after_js_config_resp.unwrap();
          let _ = tx.send(after_js_config_resp);
          Ok(())
        },
      );

      let ret = rx.recv().unwrap();
      config_resp.data_id = ret.data_id;
      config_resp.group = ret.group;
      config_resp.namespace = ret.namespace;
      config_resp.content = ret.content;
      config_resp.encrypted_data_key = ret.encrypted_data_key;
    }
  }
}

/// ConfigReq for [`ConfigFilter`]
#[napi(object)]
pub struct NacosConfigReq {
  /// DataId
  pub data_id: String,
  /// Group
  pub group: String,
  /// Namespace/Tenant
  pub namespace: String,
  /// Content
  pub content: String,
  /// Content's Encrypted Data Key.
  pub encrypted_data_key: String,
}

/// ConfigResp for [`ConfigFilter`]
#[napi(object)]
pub struct NacosConfigResp {
  /// DataId
  pub data_id: String,
  /// Group
  pub group: String,
  /// Namespace/Tenant
  pub namespace: String,
  /// Content
  pub content: String,
  /// Content's Encrypted Data Key.
  pub encrypted_data_key: String,
}
