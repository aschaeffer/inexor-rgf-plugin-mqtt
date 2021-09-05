// use std::convert::AsRef;
// use std::sync::{Arc};
//
// use log::{debug, error};
// use serde_json::{json, Value};
//
// use crate::MqttEndpointProperties;
// use crate::model::{PropertyInstanceGetter};
// use crate::model::ReactiveEntityInstance;
// use crate::reactive::entity::Disconnectable;
// // use rumqttc_async_std::{MqttOptions, AsyncClient, QoS};
// use rumqttc::{MqttOptions, AsyncClient, QoS};
//
// /// MQTT Subscriber
// pub struct MqttSubscriber {
//     pub entity: Arc<ReactiveEntityInstance>,
//
//     pub handle_id: u128,
// }
// impl MqttSubscriber {
//     pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> MqttSubscriber {
//         let handle_id = e.properties.get(MqttEndpointProperties::PAYLOAD.as_ref()).unwrap().id.as_u128();
//         MqttSubscriber {
//             entity: e,
//             handle_id
//         }
//     }
// }
//
// impl Disconnectable for MqttSubscriber {
//     fn disconnect(&self) {
//         debug!("Disconnect mqtt subscriber {}", self.handle_id);
//         // self.entity.properties.get(MqttEndpointProperties::PAYLOAD.as_ref()).unwrap()
//         //     .stream.read().unwrap().remove(self.handle_id);
//     }
// }
//
// /// Automatically disconnect streams on destruction
// impl Drop for MqttSubscriber {
//     fn drop(&mut self) {
//         self.disconnect();
//     }
// }
