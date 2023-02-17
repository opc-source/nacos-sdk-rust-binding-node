'use strict';

const { NacosNamingClient, NacosServiceInstance } = require('../index')

// If it fails, pay attention to err
const nacos_naming_client = new NacosNamingClient({
    serverAddr: '0.0.0.0:8848',
    namespace: "hongwen",
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
  var instance_arr = nacos_naming_client.getAllInstances(serviceName, group); // If it fails, pay attention to err
  console.log(instance_arr);
  await sleep(1000);

  console.log('--------- batchRegisterInstance instance2 ------------');
  nacos_naming_client.batchRegisterInstance(serviceName, group, [instance1, instance2]); // If it fails, pay attention to err
  await sleep(1000);

  console.log('--------- get all instances 2 ------------');
  var instance_arr = nacos_naming_client.getAllInstances(serviceName, group); // If it fails, pay attention to err
  console.log(instance_arr);

  await sleep(300000);
  process.exit(0);
})();


