'use strict';

const { NacosNamingClient, NacosServiceInstance } = require('../index')

// If it fails, pay attention to err
// 请注意！一般情况下，应用下仅需一个 Naming 客户端，而且需要长期持有直至应用停止。
// 因为它内部会初始化与服务端的长链接，后续的数据交互及变更订阅，都是实时地通过长链接告知客户端的。
const nacos_naming_client = new NacosNamingClient({
    serverAddr: '127.0.0.1:8848',
    namespace: "love",
    appName: "binding-node-example-app"
});

const instance1 = {
  ip: '127.0.0.1',
  port: 9090,
  metadata: { 'application' : 'example-naming' },
}


const instance2 = {
  ip: '127.0.0.2',
  port: 9099,
  weight: 2.0,
  healthy: true,
  enabled: true,
  ephemeral: true,
  metadata: { 'application' : 'example-naming' },
}

function sleep(time){
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve();
    }, time);
  })
}

(async () => {

  const serviceName = 'TestServiceName';
  const group = 'LOVE';

  nacos_naming_client.subscribe(serviceName, group, null, (err, instance_array) => { console.log('subscribe instance_array => ' + JSON.stringify(instance_array)) });
  await sleep(2000);

  console.log('--------- registerInstance instance1 ------------');
  nacos_naming_client.registerInstance(serviceName, group, instance1); // If it fails, pay attention to err
  await sleep(1000);

  console.log('--------- get all instances 1 ------------');
  nacos_naming_client.getAllInstances(serviceName, group).then(instance_arr => {
    console.log(instance_arr);
  });
  await sleep(1000);

  console.log('--------- batchRegisterInstance instance2 ------------');
  nacos_naming_client.batchRegisterInstance(serviceName, group, [instance1, instance2]); // If it fails, pay attention to err
  await sleep(1000);

  console.log('--------- get all instances 2 ------------');
  nacos_naming_client.getAllInstances(serviceName, group).then(instance_arr => {
    console.log(instance_arr);
  });

  await sleep(300000);
  process.exit(0);
})();
