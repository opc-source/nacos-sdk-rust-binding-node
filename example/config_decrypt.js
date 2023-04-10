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
    // config_req or config_resp only one not null. e.g. you can do encrypt for config_req, decrypt for config_resp.
    if (config_resp != null) {
        // TODO by customize and please care about encryptedDataKey whether not null
        //  config_resp.content = aes decrypt config_resp.content with config_resp.encryptedDataKey ??
        config_resp.content = "func config_decrypt change it."
    }
    if (config_req != null) {
        // TODO by customize and please care about encryptedDataKey whether not null
        //  config_req.content = aes encrypt config_req.content with config_req.encryptedDataKey ??
    }
    // !!! must return them !!!
    return [config_req, config_resp];
}
);

try {
    // If it fails, pay attention to err
    nacos_config_client.getConfig('hongwen.properties', 'LOVE').then(data => {
        console.log('getConfig => ' + data);
    });
   
    nacos_config_client.getConfigResp('hongwen.properties', 'LOVE').then((data) => {
        console.log('getConfigResp => ' + JSON.stringify(data));
    });
} catch(e) {
    console.log(e);
}

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, config_resp) => { console.log(config_resp) });
