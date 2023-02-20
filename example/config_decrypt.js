'use strict';

const { NacosConfigClient, NacosConfigResponse } = require('../index')

// If it fails, pay attention to err
const nacos_config_client = new NacosConfigClient(
{
    serverAddr: '0.0.0.0:8848',
    namespace: "hongwen",
    appName: "binding-node-example-app"
},
(err, config_req, config_resp) => {
 console.log(config_resp)
 config_resp.content = "func config_decrypt change it."
 return [config_req, config_resp];
}
);

try {
    // If it fails, pay attention to err
    var conf_content = nacos_config_client.getConfig('hongwen.properties', 'LOVE');
    console.log(conf_content);

    var config_resp = nacos_config_client.getConfigResp('hongwen.properties', 'LOVE');
    console.log(config_resp.content);
} catch(e) {
    console.log(e);
}

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, config_resp) => { console.log(config_resp) });
