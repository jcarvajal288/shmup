use bevy::prelude::Resource;

#[derive(Resource)]
pub struct PlayerStats {
    pub lives: usize
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            lives: 2
        }
    }
}


