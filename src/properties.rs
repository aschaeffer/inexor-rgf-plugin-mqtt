use indradb::NamedProperty;
use serde_json::{json, Value};
use strum_macros::{AsRefStr, IntoStaticStr, ToString};

use crate::reactive::property::NamedProperties;

#[derive(Copy, Clone, AsRefStr, IntoStaticStr, ToString)]
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
#[derive(AsRefStr, IntoStaticStr, ToString)]
pub enum MqttBrokerProperties {
    #[strum(serialize = "hostname")]
    HOSTNAME,
    #[strum(serialize = "port")]
    PORT,
    #[strum(serialize = "send_package")]
    SEND_PACKAGE,
    #[strum(serialize = "received_package")]
    RECEIVED_PACKAGE,
}

impl MqttBrokerProperties {
    pub fn default_value(&self) -> String {
        match self {
            MqttBrokerProperties::HOSTNAME => String::from("localhost"),
            MqttBrokerProperties::PORT => String::from("1833"), // TODO: i64
            MqttBrokerProperties::SEND_PACKAGE => String::from("{}"),
            MqttBrokerProperties::RECEIVED_PACKAGE => String::from("{}"),
        }
    }
    pub fn properties() -> NamedProperties {
        vec![
            NamedProperty::from(MqttBrokerProperties::HOSTNAME),
            NamedProperty::from(MqttBrokerProperties::PORT),
            NamedProperty::from(MqttBrokerProperties::SEND_PACKAGE),
            NamedProperty::from(MqttBrokerProperties::RECEIVED_PACKAGE),
        ]
    }
}

impl From<MqttBrokerProperties> for NamedProperty {
    fn from(p: MqttBrokerProperties) -> Self {
        NamedProperty {
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
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
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, ToString)]
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
            name: p.to_string(),
            value: json!(p.default_value()),
        }
    }
}
