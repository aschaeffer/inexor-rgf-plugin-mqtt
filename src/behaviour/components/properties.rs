use indradb::{Identifier, NamedProperty};
use serde_json::{json, Value};
use strum_macros::{AsRefStr, Display, IntoStaticStr};

use crate::reactive::property::NamedProperties;

#[derive(Copy, Clone, AsRefStr, IntoStaticStr, Display)]
pub enum MqttPayloadMode {
    Json,
    Raw,
}

impl From<&str> for MqttPayloadMode {
    fn from(mode: &str) -> Self {
        match mode {
            "json" => MqttPayloadMode::Json,
            "raw" => MqttPayloadMode::Raw,
            _ => MqttPayloadMode::Raw,
        }
    }
}

pub enum MqttPayload {
    Json(Value),
    Raw(Value),
}

impl ToString for MqttPayload {
    fn to_string(&self) -> String {
        return match self {
            Self::Json(value) => value.clone().to_string(),
            Self::Raw(value) => value.clone().as_str().unwrap_or("").to_string(),
        };
    }
}

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum MqttTopicProperties {
    #[strum(serialize = "topic")]
    TOPIC,
    #[strum(serialize = "mode")]
    MODE,
}

impl MqttTopicProperties {
    pub fn default_value(&self) -> String {
        match self {
            MqttTopicProperties::TOPIC => String::from(""),
            MqttTopicProperties::MODE => String::from("json"),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(MqttTopicProperties::TOPIC),
            NamedProperty::from(MqttTopicProperties::MODE),
        ]
    }
}

impl From<MqttTopicProperties> for NamedProperty {
    fn from(p: MqttTopicProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
pub enum MqttEndpointProperties {
    #[strum(serialize = "payload")]
    PAYLOAD,
}

impl MqttEndpointProperties {
    pub fn default_value(&self) -> String {
        match self {
            MqttEndpointProperties::PAYLOAD => String::from(""),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![NamedProperty::from(MqttEndpointProperties::PAYLOAD)]
    }
}

impl From<MqttEndpointProperties> for NamedProperty {
    fn from(p: MqttEndpointProperties) -> Self {
        NamedProperty {
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}
