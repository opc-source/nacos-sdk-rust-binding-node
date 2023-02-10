
const { NacosConfigClient, NacosConfigResponse } = require('../index')

const nacos_config_client = new NacosConfigClient('0.0.0.0:8848', "test", "binding-node-example-app");

nacos_config_client.getConfig('test-data-id', 'DEFAULT_GROUP');

nacos_config_client.addListener('test-data-id', 'DEFAULT_GROUP', (conf_resp) => { console.log(NacosConfigResponse) });
