use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Language {
    VI,
    EN,
}
