use bevy::prelude::*;

use crate::{random::GlobalRng, screen_rect, screens::Screen};

const MINION_SIZE: f32 = 64.;
const MINIONS_Z: f32 = 0.;
const MINION_COUNT: usize = 10;

#[derive(Component)]
pub struct Ally;

#[derive(Component)]
pub struct Enemy;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Arena), spawn_minions);
}

fn spawn_minions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    let mesh = meshes.add(Rectangle::new(MINION_SIZE, MINION_SIZE));
    let ally_color  = materials.add(Color::linear_rgb(0.2, 0.6, 1.0));
    let enemy_color = materials.add(Color::linear_rgb(1.0, 0.2, 0.2));

    let ally_spawn_area  = screen_rect(Vec2::new(-0.375, 0.0), Vec2::new(0.25, 1.0));
    let enemy_spawn_area = screen_rect(Vec2::new(0.375, 0.0) , Vec2::new(0.25, 1.0));

    for _ in 0..MINION_COUNT {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ally_color.clone()),
            Transform::from_translation(rng.next_in_rect(ally_spawn_area).extend(MINIONS_Z)),
            Ally,
        ));
    }

    for _ in 0..MINION_COUNT {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(enemy_color.clone()),
            Transform::from_translation(rng.next_in_rect(enemy_spawn_area).extend(MINIONS_Z)),
            Enemy,
        ));
    }
}
