use std::convert::AsRef;
use std::sync::Arc;
use std::time::Duration;

use crate::reactive::BehaviourCreationError;
use async_std::task;
use log::{debug, error, trace};
use rumqttc::Packet::Publish;
use rumqttc::{Client, ConnectionError, Event, MqttOptions, QoS};
use serde_json::{json, Error, Value};

use crate::behaviour::components::{
    MqttEndpointProperties, MqttPayload, MqttPayloadMode, MqttTopicProperties,
};
use crate::behaviour::entity::MqttBrokerProperties;
use crate::model::PropertyInstanceGetter;
use crate::model::ReactiveEntityInstance;
use crate::reactive::entity::Disconnectable;

pub struct MqttBroker {
    pub entity: Arc<ReactiveEntityInstance>,

    pub handle_id: u128,

    stopper: crossbeam::channel::Sender<()>,
}

impl MqttBroker {
    pub fn new<'a>(e: Arc<ReactiveEntityInstance>) -> Result<MqttBroker, BehaviourCreationError> {
        let (tx, rx) = crossbeam::channel::bounded(1);

        // TODO: Validate properties
        let send_package = e
            .properties
            .get(MqttBrokerProperties::SEND_PACKAGE.as_ref());
        if send_package.is_none() {
            return Err(BehaviourCreationError.into());
        }
        let send_package = send_package.unwrap();

        let received_package = e
            .properties
            .get(MqttBrokerProperties::RECEIVED_PACKAGE.as_ref());
        if received_package.is_none() {
            return Err(BehaviourCreationError.into());
        }
        // let received_package = received_package.unwrap();

        let handle_id = send_package.id.as_u128();
        let hostname = e
            .as_string(MqttBrokerProperties::HOSTNAME.as_ref())
            .unwrap_or(MqttBrokerProperties::HOSTNAME.default_value());
        let port = e
            .as_i64(MqttBrokerProperties::PORT.as_ref())
            .unwrap_or(1833) as u16;

        let mqtt_client_id = format!("inexor-{}", e.id);
        let mqtt_options = MqttOptions::new(mqtt_client_id, hostname.clone(), port);
        // mqtt_options.set_keep_alive(5);

        let (mut mqtt_client, mut connection) = Client::new(mqtt_options, 10);

        // Subscribe all topics (the routing is done in the relationship
        let _ = mqtt_client.subscribe("#", QoS::AtMostOnce);

        let mut mqtt_client_publisher = mqtt_client.clone();

        let mqtt_hostname = hostname.clone();
        let mqtt_port = port.clone();
        send_package.stream.read().unwrap().observe_with_handle(
            move |v| {
                let topic = v.get(MqttTopicProperties::TOPIC.as_ref());
                let mode = v.get(MqttTopicProperties::MODE.as_ref());
                let payload = v.get(MqttEndpointProperties::PAYLOAD.as_ref());
                if topic.is_none() || mode.is_none() || payload.is_none() {
                    return;
                }
                let topic = topic.unwrap().as_str().unwrap();
                let mode = mode.unwrap().as_str().unwrap().into();
                // let mode = mode.unwrap().as_str().unwrap().into();
                let payload = match mode {
                    MqttPayloadMode::Json => MqttPayload::Json(payload.unwrap().clone()),
                    MqttPayloadMode::Raw => MqttPayload::Raw(payload.unwrap().clone()),
                };
                debug!(
                    "Publishing to topic {}:{}/{} ---> {}",
                    mqtt_hostname.clone(),
                    mqtt_port,
                    topic.clone(),
                    payload.to_string()
                );
                let result = mqtt_client_publisher.publish(
                    topic.clone(),
                    QoS::AtLeastOnce,
                    false,
                    payload.to_string(),
                );
                match result {
                    Ok(_) => {}
                    Err(err) => error!(
                        "Failed to publish to topic {}:{}/{} Error: {:?}",
                        mqtt_hostname.clone(),
                        mqtt_port,
                        topic.clone(),
                        err
                    ),
                }
            },
            handle_id,
        );

        let mut mqtt_client_subscriber = mqtt_client.clone();
        let entity = e.clone();
        let thread_name = format!("{}-{}", e.type_name.clone(), e.id.to_string());
        let _handler = task::Builder::new().name(thread_name).spawn(async move {
            debug!("Connecting to MQTT broker {}:{}", hostname.clone(), port);

            let received_package = entity
                .properties
                .get(MqttBrokerProperties::RECEIVED_PACKAGE.as_ref())
                .unwrap();
            // if received_package.is_none() {
            //     return Err(BehaviourCreationError.into());
            // }
            // let received_package = received_package.unwrap();

            for result in connection.iter() {
                match result {
                    Err(err) => {
                        match err {
                            ConnectionError::Io(err) => {
                                error!(
                                    "Failed to connect to MQTT broker {}:{} : {:?}",
                                    hostname.clone(),
                                    port,
                                    err
                                );
                            }
                            _ => {}
                        }
                        std::thread::sleep(Duration::from_millis(2000))
                    }
                    Ok(Event::Incoming(event)) => {
                        trace!("Incoming event {:?}", event);
                        match event {
                            Publish(publish) => {
                                trace!("Topic: {}", publish.topic);
                                let payload = String::from_utf8_lossy(publish.payload.as_ref());
                                trace!("Payload (RAW): {}", payload);
                                let payload_json: Result<Value, Error> =
                                    serde_json::from_str(payload.as_ref());
                                if payload_json.is_ok() {
                                    let payload = payload_json.unwrap();
                                    trace!("Payload (JSON): {}", payload);
                                    let value: Value = json!({
                                        MqttTopicProperties::TOPIC.as_ref(): publish.topic,
                                        // MqttTopicProperties::MODE.as_ref(): mode.clone(),
                                        MqttEndpointProperties::PAYLOAD.as_ref(): payload
                                    });
                                    received_package.set(value);
                                } else {
                                    trace!("Payload (JSON): {}", payload);
                                    let value: Value = json!({
                                        MqttTopicProperties::TOPIC.as_ref(): publish.topic,
                                        // MqttTopicProperties::MODE.as_ref(): mode.clone(),
                                        MqttEndpointProperties::PAYLOAD.as_ref(): payload.clone()
                                    });
                                    received_package.set(value);
                                }
                            }
                            _ => {}
                        }
                        // TODO: Handle received data
                        // event.
                        // let mut map: Map<String, Value> = Map::new();
                        // map.insert("", )
                        // let value = Value::Object(map);
                        // let package: Value = json!({
                        //     MqttTopicProperties::TOPIC.as_ref(): topic,
                        //     MqttTopicProperties::MODE.as_ref(): mode.clone(),
                        //     MqttEndpointProperties::PAYLOAD.as_ref(): payload
                        // });
                        // received_package.set(value);
                    }
                    _ => {}
                }
                match rx.try_recv() {
                    // Stop thread
                    Ok(_) => break,
                    // About ~ 100fps
                    Err(_) => std::thread::sleep(Duration::from_millis(100)),
                }
            }
            let _ = mqtt_client_subscriber.disconnect();
            debug!(
                "Disconnected client connection to MQTT broker {}:{}",
                hostname.clone(),
                port
            );
        });

        Ok(MqttBroker {
            entity: e.clone(),
            handle_id,
            stopper: tx.clone(),
        })
    }

    pub fn type_name(&self) -> String {
        self.entity.type_name.clone()
    }
}

impl Disconnectable for MqttBroker {
    fn disconnect(&self) {
        // Stop event loop thread
        let _ = self.stopper.send(());
        debug!("Disconnecting mqtt broker {}", self.handle_id);
        let property = self
            .entity
            .properties
            .get(MqttBrokerProperties::SEND_PACKAGE.as_ref());
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
impl Drop for MqttBroker {
    fn drop(&mut self) {
        self.disconnect();
    }
}
