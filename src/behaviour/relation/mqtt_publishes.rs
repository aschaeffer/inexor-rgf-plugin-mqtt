use std::convert::AsRef;
use std::sync::Arc;

use log::debug;
use serde_json::{json, Value};

use crate::behaviour::components::{MqttEndpointProperties, MqttTopicProperties};
use crate::behaviour::entity::MqttBrokerProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveRelationInstance;
use crate::reactive::entity::Disconnectable;

pub struct MqttPublishes {
    pub relation: Arc<ReactiveRelationInstance>,

    pub handle_id: u128,
}

impl MqttPublishes {
    pub fn new<'a>(r: Arc<ReactiveRelationInstance>) -> MqttPublishes {
        let topic = r
            .as_string(MqttTopicProperties::TOPIC.as_ref())
            .unwrap_or(MqttTopicProperties::TOPIC.default_value());
        let mode = r
            .as_string(MqttTopicProperties::MODE.as_ref())
            .unwrap_or(MqttTopicProperties::MODE.default_value());

        let publisher = r.outbound.clone();
        let broker = r.inbound.clone();

        let handle_id = publisher
            .properties
            .get(MqttEndpointProperties::PAYLOAD.as_ref())
            .unwrap()
            .id
            .as_u128();

        publisher
            .properties
            .get(MqttEndpointProperties::PAYLOAD.as_ref())
            .unwrap()
            .stream
            .read()
            .unwrap()
            .observe_with_handle(
                move |v| {
                    let payload = v.clone();
                    // TODO: log?
                    let package: Value = json!({
                        MqttTopicProperties::TOPIC.as_ref(): topic.clone(),
                        MqttTopicProperties::MODE.as_ref(): mode.clone(),
                        MqttEndpointProperties::PAYLOAD.as_ref(): payload
                    });
                    broker
                        .properties
                        .get(MqttBrokerProperties::SEND_PACKAGE.as_ref())
                        .unwrap()
                        .set(package);
                },
                handle_id,
            );

        MqttPublishes {
            relation: r.clone(),
            handle_id,
        }
    }

    pub fn type_name(&self) -> String {
        self.relation.type_name.clone()
    }
}

impl Disconnectable for MqttPublishes {
    fn disconnect(&self) {
        debug!("Disconnecting mqtt_publishes {}", self.handle_id);
        // let publisher = self.relation.inbound.clone();
        let property = self
            .relation
            .inbound
            .properties
            .get(MqttEndpointProperties::PAYLOAD.as_ref());
        if property.is_some() {
            property
                .unwrap()
                .stream
                .read()
                .unwrap()
                .remove(self.handle_id);
        }
    }
}

/// Automatically disconnect streams on destruction
impl Drop for MqttPublishes {
    fn drop(&mut self) {
        self.disconnect();
    }
}
