use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::client::controller::data::Action;

#[derive(Clone, Serialize, Deserialize)]
pub struct ControllerUpstreamEvent {
    pub id: usize,
    pub data: ControllerEventData,
}

impl ControllerUpstreamEvent {
    pub fn new(id: usize, data: ControllerEventData) -> Self {
        Self { id, data }
    }

    pub fn new_local(data: ControllerEventData) -> Self {
        Self::new(0, data)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ControllerEventData {
    Info(Uuid),
    Action(Action),
}
