'use strict';

const { NacosConfigClient, NacosConfigResponse } = require('../index')

const nacos_config_client = new NacosConfigClient({
    serverAddr: '0.0.0.0:8848',
    namespace: "hongwen",
    appName: "binding-node-example-app"
});

var conf_content = nacos_config_client.getConfig('hongwen.properties', 'LOVE');
console.log(conf_content);

var config_resp = nacos_config_client.getConfigResp('hongwen.properties', 'LOVE');
console.log(config_resp.content);

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, config_resp) => { console.log(config_resp) });
