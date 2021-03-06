use std::sync::{Arc, RwLock};

use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};

use crate::behaviour::entity::entity_behaviour_provider::MqttEntityBehaviourProviderImpl;
use crate::behaviour::relation::relation_behaviour_provider::MqttRelationBehaviourProviderImpl;
use crate::builder::EntityInstanceBuilder;
use crate::plugins::plugin::PluginMetadata;
use crate::plugins::plugin_context::PluginContext;
use crate::plugins::{
    ComponentBehaviourProvider, ComponentProvider, EntityBehaviourProvider, EntityTypeProvider,
    FlowProvider, Plugin, PluginError, RelationBehaviourProvider, RelationTypeProvider,
    WebResourceProvider,
};
use crate::provider::{
    MqttComponentProviderImpl, MqttEntityTypeProviderImpl, MqttFlowProviderImpl,
    MqttRelationTypeProviderImpl,
};
use serde_json::json;

#[wrapper]
pub struct PluginContextContainer(RwLock<Option<std::sync::Arc<dyn PluginContext>>>);

#[provides]
fn create_empty_plugin_context_container() -> PluginContextContainer {
    return PluginContextContainer(RwLock::new(None));
}

#[async_trait]
pub trait MqttPlugin: Plugin + Send + Sync {}

#[module]
pub struct MqttPluginImpl {
    component_provider: Wrc<MqttComponentProviderImpl>,
    entity_type_provider: Wrc<MqttEntityTypeProviderImpl>,
    relation_type_provider: Wrc<MqttRelationTypeProviderImpl>,
    flow_provider: Wrc<MqttFlowProviderImpl>,
    entity_behaviour_provider: Wrc<MqttEntityBehaviourProviderImpl>,
    relation_behaviour_provider: Wrc<MqttRelationBehaviourProviderImpl>,

    context: PluginContextContainer,
}

interfaces!(MqttPluginImpl: dyn Plugin);

#[async_trait]
#[provides]
impl MqttPlugin for MqttPluginImpl {}

impl Plugin for MqttPluginImpl {
    fn metadata(&self) -> Result<PluginMetadata, PluginError> {
        Ok(PluginMetadata {
            name: env!("CARGO_PKG_NAME").into(),
            description: env!("CARGO_PKG_DESCRIPTION").into(),
            version: env!("CARGO_PKG_VERSION").into(),
        })
    }

    fn init(&self) -> Result<(), PluginError> {
        debug!("MqttPluginModuleImpl::init()");
        Ok(())
    }

    fn post_init(&self) -> Result<(), PluginError> {
        debug!("MqttPluginModuleImpl::post_init()");
        let reader = self.context.0.read().unwrap();
        let entity_instance_manager = reader
            .as_ref()
            .unwrap()
            .get_entity_instance_manager()
            .clone();
        let entity_instance = EntityInstanceBuilder::new("value")
            .property("value", json!(0))
            .get();
        let reactive_entity_instance = entity_instance_manager.create(entity_instance);
        match reactive_entity_instance {
            Ok(reactive_entity_instance) => {
                debug!(
                    "Successfully created entity instance {}",
                    reactive_entity_instance.id
                );
            }
            Err(_) => {
                error!("Failed to create entity instance!");
            }
        }
        Ok(())
    }

    fn pre_shutdown(&self) -> Result<(), PluginError> {
        debug!("MqttPluginModuleImpl::pre_shutdown()");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), PluginError> {
        debug!("MqttPluginModuleImpl::shutdown()");
        Ok(())
    }

    fn set_context(&self, context: Arc<dyn PluginContext>) -> Result<(), PluginError> {
        self.context.0.write().unwrap().replace(context);
        Ok(())
    }

    fn get_component_provider(&self) -> Result<Arc<dyn ComponentProvider>, PluginError> {
        let component_provider = self.component_provider.clone();
        let component_provider: Result<Arc<dyn ComponentProvider>, _> =
            <dyn query_interface::Object>::query_arc(component_provider);
        if component_provider.is_err() {
            return Err(PluginError::NoComponentProvider);
        }
        Ok(component_provider.unwrap())
    }

    fn get_entity_type_provider(&self) -> Result<Arc<dyn EntityTypeProvider>, PluginError> {
        let entity_type_provider = self.entity_type_provider.clone();
        let entity_type_provider: Result<Arc<dyn EntityTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_type_provider);
        if entity_type_provider.is_err() {
            return Err(PluginError::NoEntityTypeProvider);
        }
        Ok(entity_type_provider.unwrap())
    }

    fn get_relation_type_provider(&self) -> Result<Arc<dyn RelationTypeProvider>, PluginError> {
        let relation_type_provider = self.relation_type_provider.clone();
        let relation_type_provider: Result<Arc<dyn RelationTypeProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_type_provider);
        if relation_type_provider.is_err() {
            return Err(PluginError::NoRelationTypeProvider);
        }
        Ok(relation_type_provider.unwrap())
    }

    fn get_component_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn ComponentBehaviourProvider>, PluginError> {
        Err(PluginError::NoComponentBehaviourProvider)
    }

    fn get_entity_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn EntityBehaviourProvider>, PluginError> {
        let entity_behaviour_provider = self.entity_behaviour_provider.clone();
        let entity_behaviour_provider: Result<Arc<dyn EntityBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(entity_behaviour_provider);
        if entity_behaviour_provider.is_err() {
            return Err(PluginError::NoEntityBehaviourProvider);
        }
        Ok(entity_behaviour_provider.unwrap())
    }

    fn get_relation_behaviour_provider(
        &self,
    ) -> Result<Arc<dyn RelationBehaviourProvider>, PluginError> {
        let relation_behaviour_provider = self.relation_behaviour_provider.clone();
        let relation_behaviour_provider: Result<Arc<dyn RelationBehaviourProvider>, _> =
            <dyn query_interface::Object>::query_arc(relation_behaviour_provider);
        if relation_behaviour_provider.is_err() {
            return Err(PluginError::NoRelationBehaviourProvider);
        }
        Ok(relation_behaviour_provider.unwrap())
    }

    fn get_flow_provider(&self) -> Result<Arc<dyn FlowProvider>, PluginError> {
        let flow_provider = self.flow_provider.clone();
        let flow_provider: Result<Arc<dyn FlowProvider>, _> =
            <dyn query_interface::Object>::query_arc(flow_provider);
        if flow_provider.is_err() {
            return Err(PluginError::NoFlowProvider);
        }
        Ok(flow_provider.unwrap())
    }

    fn get_web_resource_provider(&self) -> Result<Arc<dyn WebResourceProvider>, PluginError> {
        Err(PluginError::NoWebResourceProvider)
    }
}
