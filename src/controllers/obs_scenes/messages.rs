use std::collections::HashMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

use crate::services::ftclive::messages::FieldUpdateType;

pub type OBSSceneMapping = HashMap<FieldUpdateType, String>;
pub type OBSSceneFieldMapping = HashMap<u32, OBSSceneMapping>;

#[derive(Serialize, Deserialize)]
pub struct SetMapping {
    pub field: u32,
    pub update_type: FieldUpdateType,
    pub scene: String,
}

pub enum OBSSceneRequest {
    Close,
    SetSceneMapping {
        new_mapping: SetMapping,
        sender: Sender<Result<()>>,
    },
    GetSceneMapping(Sender<Result<OBSSceneFieldMapping>>),
    StartListener(Sender<Result<()>>),
    CheckListener(Sender<Result<bool>>),
}
