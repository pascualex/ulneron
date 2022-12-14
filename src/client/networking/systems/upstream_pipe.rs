use std::io::Write;

use bevy::prelude::*;

use crate::client::{
    controller::events::ControllerUpstreamEvent,
    networking::{resources::Connection, UpstreamMessage},
};

pub fn upstream_pipe(
    mut reader: EventReader<ControllerUpstreamEvent>,
    mut connection: ResMut<Connection>,
) {
    if let Some(event) = reader.iter().last() {
        let msg = UpstreamMessage::new(event.data.clone());
        let bytes = bincode::serialize(&msg).unwrap();
        connection.stream.write_all(&bytes).ok();
    }
}
