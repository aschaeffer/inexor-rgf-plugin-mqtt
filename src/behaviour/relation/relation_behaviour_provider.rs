use std::sync::Arc;

use async_trait::async_trait;
use indradb::EdgeKey;
use log::debug;
use waiter_di::*;

use crate::behaviour::relation::mqtt_publishes::MqttPublishes;
use crate::behaviour::relation::mqtt_subscribes::MqttSubscribes;
use crate::model::ReactiveRelationInstance;
use crate::plugins::RelationBehaviourProvider;

const MQTT_PUBLISHES: &'static str = "mqtt_publishes";

const MQTT_SUBSCRIBES: &'static str = "mqtt_subscribes";

#[wrapper]
pub struct MqttPublishesRelationBehaviourStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<MqttPublishes>>>,
);

#[wrapper]
pub struct MqttSubscribesRelationBehaviourStorage(
    std::sync::RwLock<std::collections::HashMap<EdgeKey, std::sync::Arc<MqttSubscribes>>>,
);

#[waiter_di::provides]
fn create_mqtt_publishes_relation_behaviour_storage() -> MqttPublishesRelationBehaviourStorage {
    MqttPublishesRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[waiter_di::provides]
fn create_mqtt_subscribes_relation_behaviour_storage() -> MqttSubscribesRelationBehaviourStorage {
    MqttSubscribesRelationBehaviourStorage(std::sync::RwLock::new(std::collections::HashMap::new()))
}

#[async_trait]
pub trait MqttRelationBehaviourProvider: RelationBehaviourProvider + Send + Sync {
    fn create_publishes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_publishes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn create_subscribes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_subscribes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>);

    fn remove_by_key(&self, edge_key: EdgeKey);
}

// #[derive(Clone)]
pub struct MqttRelationBehaviourProviderImpl {
    mqtt_publishes_relation_behaviour: MqttPublishesRelationBehaviourStorage,

    mqtt_subscribes_relation_behaviour: MqttSubscribesRelationBehaviourStorage,
}

interfaces!(MqttRelationBehaviourProviderImpl: dyn RelationBehaviourProvider);

#[component]
impl MqttRelationBehaviourProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {
            mqtt_publishes_relation_behaviour: create_mqtt_publishes_relation_behaviour_storage(),
            mqtt_subscribes_relation_behaviour: create_mqtt_subscribes_relation_behaviour_storage(),
        }
    }
}

#[async_trait]
#[provides]
impl MqttRelationBehaviourProvider for MqttRelationBehaviourProviderImpl {
    fn create_publishes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let mqtt_publishes = Arc::new(MqttPublishes::new(relation_instance));
        self.mqtt_publishes_relation_behaviour
            .0
            .write()
            .unwrap()
            .insert(edge_key.clone(), mqtt_publishes);
        debug!(
            "Added behaviour {} to relation instance {:?}",
            MQTT_PUBLISHES, edge_key
        );
    }

    fn remove_publishes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.mqtt_publishes_relation_behaviour
            .0
            .write()
            .unwrap()
            .remove(&edge_key);
        debug!(
            "Removed behaviour {} from relation instance {:?}",
            MQTT_PUBLISHES, edge_key
        );
    }

    fn create_subscribes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        let mqtt_subscribes = Arc::new(MqttSubscribes::new(relation_instance));
        self.mqtt_subscribes_relation_behaviour
            .0
            .write()
            .unwrap()
            .insert(edge_key.clone(), mqtt_subscribes);
        debug!(
            "Added behaviour {} to relation instance {:?}",
            MQTT_SUBSCRIBES, edge_key
        );
    }

    fn remove_subscribes_behaviour(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        let edge_key = relation_instance.get_key();
        if edge_key.is_none() {
            return;
        }
        let edge_key = edge_key.unwrap();
        self.mqtt_subscribes_relation_behaviour
            .0
            .write()
            .unwrap()
            .remove(&edge_key);
        debug!(
            "Removed behaviour {} from relation instance {:?}",
            MQTT_SUBSCRIBES, edge_key
        );
    }

    fn remove_by_key(&self, edge_key: EdgeKey) {
        if self
            .mqtt_publishes_relation_behaviour
            .0
            .write()
            .unwrap()
            .contains_key(&edge_key)
        {
            self.mqtt_publishes_relation_behaviour
                .0
                .write()
                .unwrap()
                .remove(&edge_key);
            debug!(
                "Removed behaviour {} from relation instance {:?}",
                MQTT_PUBLISHES, edge_key
            );
        }
        if self
            .mqtt_subscribes_relation_behaviour
            .0
            .write()
            .unwrap()
            .contains_key(&edge_key)
        {
            self.mqtt_subscribes_relation_behaviour
                .0
                .write()
                .unwrap()
                .remove(&edge_key);
            debug!(
                "Removed behaviour {} from relation instance {:?}",
                MQTT_SUBSCRIBES, edge_key
            );
        }
    }
}

impl RelationBehaviourProvider for MqttRelationBehaviourProviderImpl {
    fn add_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            MQTT_PUBLISHES => self.create_publishes_behaviour(relation_instance),
            MQTT_SUBSCRIBES => self.create_subscribes_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours(&self, relation_instance: Arc<ReactiveRelationInstance>) {
        match relation_instance.clone().type_name.as_str() {
            MQTT_PUBLISHES => self.remove_publishes_behaviour(relation_instance),
            MQTT_SUBSCRIBES => self.remove_subscribes_behaviour(relation_instance),
            _ => {}
        }
    }

    fn remove_behaviours_by_key(&self, edge_key: EdgeKey) {
        self.remove_by_key(edge_key);
    }
}
