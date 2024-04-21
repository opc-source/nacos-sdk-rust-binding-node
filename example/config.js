'use strict';

const { NacosConfigClient, NacosConfigResponse } = require('../index')

// If it fails, pay attention to err
// 请注意！一般情况下，应用下仅需一个 Config 客户端，而且需要长期持有直至应用停止。
// 因为它内部会初始化与服务端的长链接，后续的数据交互及变更订阅，都是实时地通过长链接告知客户端的。
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
