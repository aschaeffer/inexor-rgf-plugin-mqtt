// use std::convert::AsRef;
// use std::sync::{Arc};
//
// use log::{debug, info, error};
// use serde_json::{json, Value};
//
// use crate::{MqttEndpointProperties, MqttPayloadMode, MqttPayload};
// use crate::model::{PropertyInstanceGetter};
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// // use rumqttc_async_std::{MqttOptions, AsyncClient, QoS, Client, ConnectionError, Event, Transport};
// use rumqttc::{MqttOptions, AsyncClient, QoS, Client, ConnectionError, Event, Transport};
// use std::time::Duration;
// use async_std::task;
//
// /// MQTT Publisher
// pub struct MqttPublisher {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub handle_id: u128,
//
//     stopper: crossbeam::channel::Sender<()>,
// }
//
// impl MqttPublisher {
//     pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> MqttPublisher {
//         let (tx, rx) = crossbeam::channel::bounded(1);
//
//         let handle_id = e.properties.get(MqttEndpointProperties::PAYLOAD.as_ref()).unwrap().id.as_u128();
//         let hostname = e.as_string(MqttEndpointProperties::HOSTNAME.as_ref()).unwrap_or(MqttEndpointProperties::HOSTNAME.default_value());
//         let port = e.as_i64(MqttEndpointProperties::PORT.as_ref()).unwrap_or(1833) as u16;
//         let topic = e.as_string(MqttEndpointProperties::TOPIC.as_ref()).unwrap_or(String::new());
//         let mode = e.as_string(MqttEndpointProperties::MODE.as_ref()).unwrap_or(MqttEndpointProperties::MODE.default_value());
//         let payload_mode = mode.as_str().into();
//
//         let mut mqtt_options = MqttOptions::new("inexor", hostname.clone(), port);
//         // mqtt_options.set_keep_alive(5);
//
//         let (mut mqtt_client, mut connection) = Client::new(mqtt_options, 10);
//         let mut mqtt_client_publisher = mqtt_client.clone();
//
//         let mqtt_topic = topic.clone();
//         let mqtt_hostname = hostname.clone();
//         let mqtt_port = port.clone();
//         e.properties.get(MqttEndpointProperties::PAYLOAD.as_ref()).unwrap().stream.read().unwrap()
//             .observe_with_handle(move |v| {
//                 let payload = match payload_mode {
//                     MqttPayloadMode::Json => MqttPayload::Json(v.clone()),
//                     MqttPayloadMode::Raw => MqttPayload::Raw(v.clone())
//                 };
//                 debug!("Sending Payload to topic {}:{}/{}: {}", mqtt_hostname.clone(), mqtt_port, mqtt_topic.clone(), payload.to_string());
//                 let result = mqtt_client_publisher.publish(
//                     mqtt_topic.clone(),
//                     QoS::AtLeastOnce,
//                     false,
//                     payload.to_string()
//                 );
//                 match result {
//                     Ok(_) => {},
//                     Err(err) => error!("Failed to publish data to {}:{}/{} {:?}", mqtt_hostname.clone(), mqtt_port, mqtt_topic.clone(), err)
//                 }
//             }, handle_id);
//
//         let thread_name = format!("{}-{}", e.type_name.clone(), e.id.to_string());
//         let _handler = task::Builder::new()
//             .name(thread_name)
//             .spawn(async move {
//                 for result in connection.iter() {
//                     match result {
//                         Err(err) => {
//                             match err {
//                                 ConnectionError::Io(err) => {
//                                     error!("Failed to connect to {}:{} : {:?}", hostname.clone(), port, err);
//                                 }
//                                 _ => {}
//                             }
//                             std::thread::sleep(Duration::from_millis(2000))
//                         },
//                         _ => {}
//                     }
//                     match rx.try_recv() {
//                         // Stop thread
//                         Ok(_) => break,
//                         // About ~ 100fps
//                         Err(_) => std::thread::sleep(Duration::from_millis(100))
//                     }
//                 }
//                 mqtt_client.disconnect();
//                 debug!("Disconnected mqtt client {}:{}/{}", hostname.clone(), port, topic.clone());
//             });
//
//         MqttPublisher {
//             entity: e.clone(),
//             handle_id,
//             stopper: tx.clone(),
//         }
//     }
//
//     pub fn type_name(&self) -> String {
//         self.entity.type_name.clone()
//     }
// }
//
// impl Disconnectable for MqttPublisher {
//     fn disconnect(&self) {
//         // Stop event loop thread
//         self.stopper.send(());
//         debug!("Disconnecting mqtt publisher {}", self.handle_id);
//         self.entity.properties.get(MqttEndpointProperties::PAYLOAD.as_ref()).unwrap()
//             .stream.read().unwrap().remove(self.handle_id);
//     }
// }
//
// /// Automatically disconnect streams on destruction
// impl Drop for MqttPublisher {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
