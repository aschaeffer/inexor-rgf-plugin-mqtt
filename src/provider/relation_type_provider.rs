use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;

use crate::model::relation_type::RelationType;
use crate::plugins::RelationTypeProvider;

#[derive(RustEmbed)]
#[folder = "./assets/types/relations"]
struct MqttRelationTypeAsset;

#[async_trait]
pub trait MqttRelationTypeProvider: RelationTypeProvider + Send + Sync {}

#[derive(Clone)]
pub struct MqttRelationTypeProviderImpl {}

interfaces!(MqttRelationTypeProviderImpl: dyn RelationTypeProvider);

#[component]
impl MqttRelationTypeProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl MqttRelationTypeProvider for MqttRelationTypeProviderImpl {}

impl RelationTypeProvider for MqttRelationTypeProviderImpl {
    fn get_relation_types(&self) -> Vec<RelationType> {
        let mut relation_types = Vec::new();
        for file in MqttRelationTypeAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading relation_type from resource {}", filename);
            let asset = MqttRelationTypeAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let relation_type: RelationType = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(relation_type) => relation_type,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            relation_types.push(relation_type);
        }
        relation_types
    }
}
