# embassy-ha

Home Assistant MQTT device library for embassy.

To create a device use the [`new`] function.
After the device is created you should create one or more entities using functions such as
[`create_button`]/[`create_sensor`]/...

Once the entities have been created either [`run`] or [`connect_and_run`] should be called in a
seperate task.

There are various examples you can run locally (ex: `cargo run --features tracing --example
button`) assuming you have a home assistant instance running. To run the examples the
environment variable `MQTT_ADDRESS` should be set to the mqtt server used by home assistant.

License: MIT OR Apache-2.0
