#[macro_use]
extern crate query_interface;

use std::sync::Arc;

use crate::di::{profiles, Container, Provider};
use inexor_rgf_core_builder as builder;
use inexor_rgf_core_di as di;
use inexor_rgf_core_model as model;
use inexor_rgf_core_plugins as plugins;
use inexor_rgf_core_reactive as reactive;
use log::error;

use crate::plugin::MqttPlugin;
use crate::plugins::{Plugin, PluginError};

pub mod behaviour;
pub mod plugin;
pub mod provider;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}

pub fn construct_plugin() -> Result<Arc<dyn Plugin>, PluginError> {
    let mut container = get::<profiles::Default>();
    let container = &mut container;
    let plugin = Provider::<dyn MqttPlugin>::create(container);
    let plugin = Arc::new(plugin);
    let plugin: Result<Arc<dyn Plugin>, _> =
        <dyn query_interface::Object>::query_arc(plugin.clone());
    match plugin {
        Ok(plugin) => Ok(plugin),
        Err(_) => {
            const PKG_NAME: &str = env!("CARGO_PKG_NAME");
            error!("Failed to construct plugin {}", PKG_NAME);
            return Err(PluginError::PluginCreationError);
        }
    }
}

plugins::export_plugin!(register);

#[allow(improper_ctypes_definitions)]
extern "C" fn register(registrar: &mut dyn plugins::PluginRegistrar) {
    const PKG_NAME: &str = env!("CARGO_PKG_NAME");
    if let Err(error) = log4rs::init_file("config/logging.toml", Default::default()) {
        println!("Failed to configure logger in {}: {}", PKG_NAME, error);
    }
    if let Ok(plugin) = construct_plugin() {
        registrar.register_plugin(PKG_NAME, Box::new(plugin));
    }
}
