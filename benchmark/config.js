
const { NacosConfigClient, NacosConfigResponse } = require('../index')

const nacos_config_client = new NacosConfigClient('0.0.0.0:8848', "hongwen", "binding-node-example-app");

var conf_resp = nacos_config_client.getConfig('hongwen.properties', 'LOVE');
console.log(conf_resp);
console.log(conf_resp.content);

nacos_config_client.addListener('hongwen.properties', 'LOVE', (err, conf_resp) => { console.log(conf_resp) });
