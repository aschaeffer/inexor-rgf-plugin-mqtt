use std::convert::AsRef;
use std::sync::Arc;

use log::debug;

use crate::behaviour::components::{MqttEndpointProperties, MqttTopicProperties};
use crate::behaviour::entity::MqttBrokerProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;

pub struct MqttSubscribes {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl MqttSubscribes {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> MqttSubscribes {
        let topic = r
            .as_string(MqttTopicProperties::TOPIC.as_ref())
            .unwrap_or(String::new());
        let mode = r
            .as_string(MqttTopicProperties::MODE.as_ref())
            .unwrap_or(MqttTopicProperties::MODE.default_value());

        let broker = r.outbound.clone();
        let subscriber = r.inbound.clone();

        let handle_id = subscriber
            .properties
            .get(MqttEndpointProperties::PAYLOAD.as_ref())
            .unwrap()
            .id
            .as_u128();

        broker
            .properties
            .get(MqttBrokerProperties::RECEIVED_PACKAGE.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    // TODO: Unpack package (either RAW or parse JSON)
                    let packet = v.clone();
                    let received_topic = packet.get(MqttTopicProperties::TOPIC.as_ref());
                    if received_topic.is_none() {
                        return;
                    }
                    let received_topic = received_topic.unwrap().as_str().unwrap();
                    if received_topic != topic.as_str() {
                        return;
                    }
                    let received_payload = packet.get(MqttEndpointProperties::PAYLOAD.as_ref());
                    if received_payload.is_none() {
                        return;
                    }
                    let property = subscriber
                        .properties
                        .get(MqttEndpointProperties::PAYLOAD.as_ref());
                    if property.is_none() {
                        return;
                    }
                    property.unwrap().set(received_payload.unwrap().clone());
                    debug!(
                        "Forwarded payload from topic {} to subscriber {}",
                        topic.clone(),
                        subscriber.id
                    );
                },
                handle_id,
            );

        MqttSubscribes {
            relation: r.clone(),
            handle_id,
        }
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for MqttSubscribes {
    fn disconnect(&self) {
        debug!("Disconnecting mqtt_publishes {}", self.handle_id);
        // let publisher = self.relation.inbound.clone();
        self.relation
            .inbound
            .properties
            .get(MqttEndpointProperties::PAYLOAD.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .remove(self.handle_id);
    }
}

/// Automatically disconnect streams on destruction
impl Drop for MqttSubscribes {
    fn drop(&mut self) {
        self.disconnect();
    }
}
