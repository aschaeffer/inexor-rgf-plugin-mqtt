use indradb::{Identifier, NamedProperty};
use inexor_rgf_core_reactive::NamedProperties;
use serde_json::json;
use strum_macros::{AsRefStr, Display, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(AsRefStr, IntoStaticStr, Display)]
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
            name: Identifier::new(p.to_string()).unwrap(),
            value: json!(p.default_value()),
        }
    }
}

impl From<MqttBrokerProperties> for String {
    fn from(p: MqttBrokerProperties) -> Self {
        p.to_string()
    }
}
