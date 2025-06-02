use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{random::GlobalRng, screen_rect, screens::Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Arena), spawn_minions)
        .add_systems(
            Update,
            (set_targets, get_target_position, go_to_target)
                .chain()
                .run_if(in_state(Screen::Arena)),
        );
}

const MINION_SIZE: f32 = 64.;
const MINION_Z: f32 = 0.;
const MINION_COUNT: usize = 10;
const MINION_SPEED: f32 = 100.0;

#[derive(Component, Default)]
struct BattleEntity {
    target_pos: Vec2,
}

#[derive(Component)]
struct Enemy;

#[derive(Component)]
#[relationship(relationship_target = TargetedBy)]
struct Targeting(Entity);

#[derive(Component)]
#[relationship_target(relationship = Targeting)]
struct TargetedBy(Vec<Entity>);

fn spawn_minions(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rng: ResMut<GlobalRng>,
) {
    let mesh = meshes.add(Rectangle::new(MINION_SIZE, MINION_SIZE));
    let ally_color = materials.add(Color::linear_rgb(0.2, 0.6, 1.0));
    let enemy_color = materials.add(Color::linear_rgb(1.0, 0.2, 0.2));

    let ally_spawn_area = screen_rect(Vec2::new(-0.375, 0.0), Vec2::new(0.25, 1.0));
    let enemy_spawn_area = screen_rect(Vec2::new(0.375, 0.0), Vec2::new(0.25, 1.0));

    for _ in 0..MINION_COUNT {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(ally_color.clone()),
            Transform::from_translation(rng.next_in_rect(ally_spawn_area).extend(MINION_Z)),
            BattleEntity::default(),
        ));
    }

    for _ in 0..MINION_COUNT {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(enemy_color.clone()),
            Transform::from_translation(rng.next_in_rect(enemy_spawn_area).extend(MINION_Z)),
            BattleEntity::default(),
            Enemy,
        ));
    }
}

fn set_targets(
    mut commands: Commands,
    targetless_query: Query<
        (Entity, Option<&Enemy>, Option<&TargetedBy>),
        (With<BattleEntity>, Without<Targeting>),
    >,
    mut ally_query: Query<(Entity, Option<&Targeting>), (With<BattleEntity>, Without<Enemy>)>,
    mut enemy_query: Query<(Entity, Option<&Targeting>), (With<BattleEntity>, With<Enemy>)>,
    mut rng: ResMut<GlobalRng>,
) {
    let mut ally_query = ally_query.transmute_lens();
    let ally_query: Query<(Entity, Option<&Targeting>)> = ally_query.query();
    let mut enemy_query = enemy_query.transmute_lens();
    let enemy_query: Query<(Entity, Option<&Targeting>)> = enemy_query.query();

    for (entity, maybe_enemy, maybe_targeted_by) in targetless_query.iter() {
        // 1. if targeted, assign targeter as target
        if let Some(target) =
            maybe_targeted_by.and_then(|TargetedBy(targeters)| targeters.first().copied())
        {
            commands.entity(entity).insert(Targeting(target));
            continue;
        }

        // 2. otherwise assign random target
        let opponents_query = if maybe_enemy.is_some() {
            ally_query
        } else {
            enemy_query
        };
        let Some((target_entity, target_maybe_targeting)) =
            opponents_query.iter().choose(&mut rng.0)
        else {
            continue;
        };
        commands.entity(entity).insert(Targeting(target_entity));

        // 3. if target don't have target, assign itself as his target
        if target_maybe_targeting.is_none() {
            commands.entity(target_entity).insert(Targeting(entity));
        }
    }
}

fn get_target_position(
    mut battle_entity_query: Query<(&mut BattleEntity, &Targeting)>,
    transform_query: Query<&Transform, With<BattleEntity>>,
) {
    for (mut battle_entity, &Targeting(target)) in battle_entity_query.iter_mut() {
        battle_entity.target_pos = transform_query.get(target).unwrap().translation.truncate();
    }
}

fn go_to_target(
    // all targets are ensured to also have a target (thanks to logic of set_targets system)
    mut query: Query<(&mut Transform, &BattleEntity)>,
    time: Res<Time>,
) {
    for (mut transform, battle_entity) in query.iter_mut() {
        let direction =
            (battle_entity.target_pos - transform.translation.truncate()).normalize_or_zero();
        let delta = direction * MINION_SPEED * time.delta_secs();
        let z = transform.translation.z;
        transform.translation += delta.extend(z);
    }
}
