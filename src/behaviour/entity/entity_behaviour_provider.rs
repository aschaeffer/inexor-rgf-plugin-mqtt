use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use uuid::Uuid;
use waiter_di::*;

use crate::behaviour::entity::mqtt_broker::MqttBroker;
use crate::model::ReactiveEntityInstance;
use crate::plugins::EntityBehaviourProvider;

#[wrapper]
pub struct MqttBrokerStorage(
    std::sync::RwLock<std::collections::HashMap<Uuid, std::sync::Arc<MqttBroker>>>,
);

#[waiter_di::provides]
fn create_mqtt_brokers_storage() -> MqttBrokerStorage {
    MqttBrokerStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait MqttEntityBehaviourProvider: EntityBehaviourProvider + Send + Sync {
    fn create_broker(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_broker(&self, entity_instance: Arc<ReactiveEntityInstance>);

    fn remove_by_id(&self, id: Uuid);
}

// #[derive(Clone)]
pub struct MqttEntityBehaviourProviderImpl {
    mqtt_brokers: MqttBrokerStorage,
}

interfaces!(MqttEntityBehaviourProviderImpl: dyn EntityBehaviourProvider);

#[component]
impl MqttEntityBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            mqtt_brokers: create_mqtt_brokers_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl MqttEntityBehaviourProvider for MqttEntityBehaviourProviderImpl {
    fn create_broker(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        let id = entity_instance.id;
        let broker = MqttBroker::new(entity_instance);
        if broker.is_ok() {
            let broker = Arc::new(broker.unwrap());
            self.mqtt_brokers.0.write().unwrap().insert(id, broker);
            debug!("Added behaviour mqtt_brokers to entity instance {}", id);
        }
    }

    fn remove_broker(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        self.mqtt_brokers
            .0
            .write()
            .unwrap()
            .remove(&entity_instance.id);
        debug!(
            "Removed behaviour mqtt_publisher to entity instance {}",
            entity_instance.id
        );
    }

    fn remove_by_id(&self, id: Uuid) {
        if self.mqtt_brokers.0.write().unwrap().contains_key(&id) {
            self.mqtt_brokers.0.write().unwrap().remove(&id);
            debug!("Added behaviour mqtt_broker to entity instance {}", id);
        }
    }
}

impl EntityBehaviourProvider for MqttEntityBehaviourProviderImpl {
    fn add_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            // TODO: Use constants: EntityType::TYPE_NAME
            "mqtt_broker" => self.create_broker(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, entity_instance: Arc<ReactiveEntityInstance>) {
        match entity_instance.clone().type_name.as_str() {
            // TODO: Use constants: EntityType::TYPE_NAME
            "mqtt_broker" => self.remove_broker(entity_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_id(&self, id: Uuid) {
        self.remove_by_id(id);
    }
}