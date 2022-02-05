use crate::di::*;
use async_trait::async_trait;
use log::{debug, error};
use rust_embed::RustEmbed;

use crate::model::flow::Flow;
use crate::plugins::FlowProvider;

#[derive(RustEmbed)]
#[folder = "./assets/flows"]
struct MqttFlowAsset;

#[async_trait]
pub trait MqttFlowProvider: FlowProvider + Send + Sync {}

#[derive(Clone)]
pub struct MqttFlowProviderImpl {}

interfaces!(MqttFlowProviderImpl: dyn FlowProvider);

#[component]
impl MqttFlowProviderImpl {
    #[provides]
    fn new() -> Self {
        Self {}
    }
}

#[async_trait]
#[provides]
impl MqttFlowProvider for MqttFlowProviderImpl {}

impl FlowProvider for MqttFlowProviderImpl {
    fn get_flows(&self) -> Vec<Flow> {
        let mut flows = Vec::new();
        for file in MqttFlowAsset::iter() {
            let filename = file.as_ref();
            debug!("Loading flow from resource {}", filename);
            let asset = MqttFlowAsset::get(filename).unwrap();
            let json_str = std::str::from_utf8(asset.data.as_ref());
            if json_str.is_err() {
                error!("Could not decode UTF-8 {}", filename);
                continue;
            }
            let flow: Flow = match serde_json::from_str(json_str.unwrap()) {
                Result::Ok(flow) => flow,
                Result::Err(err) => {
                    error!("Error in parsing JSON file {}: {}", filename, err);
                    continue;
                }
            };
            flows.push(flow);
        }
        flows
    }
}
