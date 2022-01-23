# Inexor Reactive Graph Flow

| Project             | Module | Sub-Module | Functionality                                                        | Tests                                                                                                                                                |
|---------------------|--------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reactive Graph Flow | Plugin | MQTT       | <img src="https://img.shields.io/badge/state-completed-brightgreen"> | [<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-mqtt">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-mqtt) |

### About Inexor

<a href="https://inexor.org/">
<img align="right" width="200" height="200" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-mqtt/main/docs/images/inexor_2.png">
</a>

* Inexor will be a new first-person shooter game which is based on a new octree-based game engine.
* Inexor focuses on classic gameplay as we've seen in Cube2 or the Quake series.
* Inexor will be written from ground up new in C++17 and Rust.
* You can contribute anything you want: code, content, ideas..
* Inexor and all its content is 100% open source!

### About Inexor Reactive Graph Flow

The Inexor Reactive Graph Flow (RGF) manages reactive flows based on a graph database. The main interface is GraphQL.

* Semantic: Graph database with entities and relationships as first class citizens
* Reactive: entities and relationships are/can be reactive: If the input has been altered the entity processes its new state
* Interoperable: Use GraphQL for queries and mutations
* Extendable: Built in type system: components, entity types and relation types
* Memory efficient: Rust
* Fast: Rust
* Secure: Rust

### About this plugin

This plugin provides the type system, interfaces and services for the MQTT protocol.

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">](https://www.rust-lang.org/)
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/inexor-rgf-plugin-mqtt/Rust">](https://github.com/aschaeffer/inexor-rgf-plugin-mqtt/actions?query=workflow%3ARust)
[<img src="https://img.shields.io/github/last-commit/aschaeffer/inexor-rgf-plugin-mqtt">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/inexor-rgf-plugin-mqtt">]()
[<img src="https://img.shields.io/codecov/c/github/aschaeffer/inexor-rgf-plugin-mqtt">](https://app.codecov.io/gh/aschaeffer/inexor-rgf-plugin-mqtt)

[<img src="https://img.shields.io/github/license/aschaeffer/inexor-rgf-plugin-mqtt">](https://github.com/aschaeffer/inexor-rgf-plugin-mqtt/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

#### Type System

<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-mqtt/main/docs/images/type_system.png" alt="Visualisation of the graph type system">

#### Components

| Name          | Description | Properties    |
|---------------|-------------|---------------|
| mqtt_endpoint |             | payload       |
| mqtt_topic    |             | topic<br>mode |

#### Entity Types

| Name            | Description | Components    | Properties                                           |
|-----------------|-------------|---------------|------------------------------------------------------|
| mqtt_broker     |             |               | hostname<br>port<br>send_package<br>received_package |
| mqtt_publisher  |             | mqtt_endpoint | payload                                              |
| mqtt_subscriber |             | mqtt_endpoint | payload                                              |

#### Relation Types

| Name            | Description | Components   | Source Entity Type | Target Entity Type |
|-----------------|-------------|--------------|--------------------|--------------------|
| mqtt_publishes  |             | mqtt_topic   | mqtt_publisher     | mqtt_broker        |
| mqtt_subscribes |             | mqtt_topic   | mqtt_broker        | mqtt_subscriber    |

#### Instance System

<img src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-mqtt/main/docs/images/mqtt_broker_subscriber_and_publisher.png" alt="Visualisation of the graph instance system">

This is the graph representation of a publish/subscribe interaction with an MQTT-Broker. Multiple MQTT-Brokers can exist.

* Multiple `mqtt_publisher`s are `mqtt_publishes` to a topic on the `mqtt_broker`. A user can write into the `payload` property of a `mqtt_publisher` in order to publish a message.
* Multiple `mqtt_subscriber`s are `mqtt_subscribes` a topic on the `mqtt_broker`. A user can read from the `payload` property of a `mqtt_subscriber` in order to receive a new message.
* The MQTT topic is configured *on the relationships* (`mqtt_publishes`, `mqtt_subscribes`)

### Thanks to

* https://github.com/xd009642/tarpaulin
* https://codecov.io/

### Sponsors

|                                                                                                                                                                                                                               |           |                                                                   |
|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------|-------------------------------------------------------------------|
| <a href="https://www.jetbrains.com/?from=github.com/inexorgame"><img align="right" width="100" height="100" src="https://raw.githubusercontent.com/aschaeffer/inexor-rgf-plugin-logical/main/docs/images/icon_CLion.svg"></a> | JetBrains | Special thanks to JetBrains for providing us with CLion licenses! |
