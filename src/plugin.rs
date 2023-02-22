use napi::threadsafe_function::*;
use std::sync::{Arc, mpsc::channel};

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

      let ref_config_req = config_req as *mut nacos_sdk::api::plugin::ConfigReq;
      self.func.clone().call_with_return_value(
        Ok((Some(js_config_req), None)),
        ThreadsafeFunctionCallMode::Blocking,
        move |(after_js_config_req, _after_js_config_resp): (
          Option<NacosConfigReq>,
          Option<NacosConfigResp>,
        )| {
          let after_js_config_req = after_js_config_req.unwrap();
          unsafe {
            (*ref_config_req).data_id = after_js_config_req.data_id;
            (*ref_config_req).group = after_js_config_req.group;
            (*ref_config_req).namespace = after_js_config_req.namespace;
            (*ref_config_req).content = after_js_config_req.content;
            (*ref_config_req).encrypted_data_key = after_js_config_req.encrypted_data_key;
          }
          Ok(())
        },
      );
    }

    if let Some(config_resp) = config_resp {
      let js_config_resp = NacosConfigResp {
        data_id: config_resp.data_id.clone(),
        group: config_resp.group.clone(),
        namespace: config_resp.namespace.clone(),
        content: config_resp.content.clone(),
        encrypted_data_key: config_resp.encrypted_data_key.clone(),
      };

      // let ref_config_resp= config_resp as *mut nacos_sdk::api::plugin::ConfigResp;

      let (tx, rx) = channel::<NacosConfigResp>();
      self.func.clone().call_with_return_value(
        Ok((None, Some(js_config_resp))),
        ThreadsafeFunctionCallMode::Blocking,
        move |(_after_js_config_req, after_js_config_resp): (
          Option<NacosConfigReq>,
          Option<NacosConfigResp>,
        )| {
          let after_js_config_resp = after_js_config_resp.unwrap();
          println!("after_js_config_resp.content => {}", after_js_config_resp.content);
          // FIXME now has err: pointer being freed was not allocated
          // unsafe {
          //   (*ref_config_resp).data_id = after_js_config_resp.data_id;
          //   (*ref_config_resp).group = after_js_config_resp.group;
          //   (*ref_config_resp).namespace = after_js_config_resp.namespace;
          //   (*ref_config_resp).content = after_js_config_resp.content;
          //   (*ref_config_resp).encrypted_data_key = after_js_config_resp.encrypted_data_key;
          // }
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
      println!("the end filter")
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
