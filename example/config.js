'use strict';

const { NacosConfigClient, NacosConfigResponse } = require('../index')

// If it fails, pay attention to err
const nacos_config_client = new NacosConfigClient({
    serverAddr: '0.0.0.0:8848',
    namespace: "hongwen",
    appName: "binding-node-example-app"
});

try {
    // If it fails, pay attention to err
    nacos_config_client.getConfig('hongwen.properties', 'LOVE').then(data => {
        console.log('getConfig => ' + data);
    });

    nacos_config_client.getConfigResp('hongwen.properties', 'LOVE').then(data => {
        console.log('getConfigResp => ' + JSON.stringify(data));
    });
} catch(e) {
    console.log(e);
}

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, config_resp) => { console.log(config_resp) });
