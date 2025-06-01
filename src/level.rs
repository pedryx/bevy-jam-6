use bevy::prelude::*;

mod minions;

pub fn plugin(app: &mut App) {
    app.add_plugins(minions::plugin);
}