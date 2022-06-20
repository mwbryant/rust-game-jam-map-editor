use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
    pub rotation: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Map {
    pub rects: Vec<Rect>,
}
