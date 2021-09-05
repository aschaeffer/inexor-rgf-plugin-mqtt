use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;
use waiter_di::*;

use crate::behaviour::relation::mqtt_publishes::MqttPublishes;
use crate::behaviour::relation::mqtt_subscribes::MqttSubscribes;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

#[wrapper]
pub struct MqttPublishesStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<MqttPublishes>>>,
);

#[wrapper]
pub struct MqttSubscribesStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<MqttSubscribes>>>,
);

#[waiter_di::provides]
fn create_mqtt_publishes_storage() -> MqttPublishesStorage {
    MqttPublishesStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_mqtt_subscribes_storage() -> MqttSubscribesStorage {
    MqttSubscribesStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait MqttRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_publishes(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_publishes(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_subscribes(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_subscribes(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

// #[derive(Clone)]
pub struct MqttRelationBehaviourProviderImpl {
    mqtt_publishes: MqttPublishesStorage,

    mqtt_subscribes: MqttSubscribesStorage,
}

interfaces!(MqttRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

#[component]
impl MqttRelationBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            mqtt_publishes: create_mqtt_publishes_storage(),
            mqtt_subscribes: create_mqtt_subscribes_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl MqttRelationBehaviourProvider for MqttRelationBehaviourProviderImpl {
    fn create_publishes(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let publisher = Arc::new(MqttPublishes::new(relation_instance));
        self.mqtt_publishes
            .0
            .write()
            .unwrap()
            .insert(edge_key.clone(), publisher);
        debug!(
            "Added behaviour mqtt_publisher to relation instance {:?}",
            edge_key
        );
    }

    fn remove_publishes(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.mqtt_publishes.0.write().unwrap().remove(&edge_key);
        debug!(
            "Removed behaviour mqtt_publisher to relation instance {:?}",
            edge_key
        );
    }

    fn create_subscribes(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let subscriber = Arc::new(MqttSubscribes::new(relation_instance));
        self.mqtt_subscribes
            .0
            .write()
            .unwrap()
            .insert(edge_key.clone(), subscriber);
        debug!(
            "Added behaviour mqtt_subscriber to relation instance {:?}",
            edge_key
        );
    }

    fn remove_subscribes(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.mqtt_subscribes.0.write().unwrap().remove(&edge_key);
        debug!(
            "Removed behaviour mqtt_subscriber to relation instance {:?}",
            edge_key
        );
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self
            .mqtt_publishes
            .0
            .write()
            .unwrap()
            .contains_key(&edge_key)
        {
            self.mqtt_publishes.0.write().unwrap().remove(&edge_key);
            debug!(
                "Added behaviour mqtt_publisher to relation instance {:?}",
                edge_key
            );
        }
        if self
            .mqtt_subscribes
            .0
            .write()
            .unwrap()
            .contains_key(&edge_key)
        {
            self.mqtt_subscribes.0.write().unwrap().remove(&edge_key);
            debug!(
                "Added behaviour mqtt_subscriber to relation instance {:?}",
                edge_key
            );
        }
    }
}

impl RelationBehaviourProvider for MqttRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            // TODO: Use constants: RelationType::TYPE_NAME
            "mqtt_publishes" => self.create_publishes(relation_instance),
            "mqtt_subscribes" => self.create_subscribes(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            // TODO: Use constants: RelationType::TYPE_NAME
            "mqtt_publishes" => self.remove_publishes(relation_instance),
            "mqtt_subscribes" => self.remove_subscribes(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
