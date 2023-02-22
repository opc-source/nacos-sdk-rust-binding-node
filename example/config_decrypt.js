'use strict';

const { NacosConfigClient, NacosConfigResponse } = require('../index')

// If it fails, pay attention to err
const nacos_config_client = new NacosConfigClient(
{
    serverAddr: '127.0.0.1:8848',
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
    var promise1 = nacos_config_client.getConfig('hongwen.properties', 'LOVE').then(data => {
        console.log(data);
    });
   

    var promise2 = nacos_config_client.getConfigResp('hongwen.properties', 'LOVE').then((data) => {
        console.log(data);
    });
} catch(e) {
    console.log(e);
}

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, config_resp) => { console.log(config_resp) });
