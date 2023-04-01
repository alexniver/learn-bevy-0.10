use bevy::prelude::*;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource)]
pub struct HighScores {
    pub value: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { value: vec![] }
    }
}
