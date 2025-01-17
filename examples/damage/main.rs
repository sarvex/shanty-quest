use asset_struct::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use jam::{
    common::prelude::*,
    game::overworld::character_controller::CharacterControllerPlugin,
    game::overworld::{damage::DamagePlugin, healthbar::HealthbarPlugin},
    game::prelude::*,
};

#[derive(Component)]
pub struct Editable;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Damage".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(CommonPlugin)
        .add_plugin(CharacterControllerPlugin)
        .add_plugin(DamagePlugin)
        .add_plugin(HealthbarPlugin)
        .add_startup_system(init)
        .add_system(player_control)
        .add_system(bullet_update)
        .run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec2,
}

#[derive(Component)]
pub struct Crate;

pub fn init(
    mut commands: Commands,
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut ev_healthbar_spawn: EventWriter<HealthbarSpawnEvent>,
) {
    asset_library.load_assets(&asset_server);
    commands.spawn(Camera2dBundle::default());
    let player_entity = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(32., 32.).into(),
                    color: Color::GREEN,
                    ..Default::default()
                },
                ..Default::default()
            },
            Transform2::from_xy(0., 0.),
            Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(32., 32.),
                },
                flags: 1,
            },
            CharacterController {
                movement: Vec2::ZERO,
                speed: 300.,
                ..Default::default()
            },
            Hitbox {
                shape: CollisionShape::Rect {
                    size: Vec2::new(32., 32.),
                },
                for_entity: None,
                flags: 1,
            },
            Player,
            Health::new(3.),
        ))
        .id();
    ev_healthbar_spawn.send(HealthbarSpawnEvent {
        entity: Some(player_entity),
        offset: Vec2::new(0., 30.),
        size: Vec2::new(30., 8.),
    });
    for _ in 0..100 {
        let x = (100. + rand::random::<f32>() * 400.) * if rand::random() { 1. } else { -1. };
        let y = (100. + rand::random::<f32>() * 200.) * if rand::random() { 1. } else { -1. };
        let crate_entity = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(20., 20.).into(),
                        color: Color::ORANGE,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Transform2::from_xy(x, y),
                Collision {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(20., 20.),
                    },
                    flags: 1,
                },
                CharacterController {
                    movement: Vec2::ZERO,
                    speed: 300.,
                    ..Default::default()
                },
                Hitbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(20., 20.),
                    },
                    for_entity: None,
                    flags: 1,
                },
                Crate,
                Health::new(3.),
            ))
            .id();
        ev_healthbar_spawn.send(HealthbarSpawnEvent {
            entity: Some(crate_entity),
            offset: Vec2::new(0., 20.),
            size: Vec2::new(15., 4.),
        });
    }
}

fn player_control(
    mut query: Query<(Entity, &mut CharacterController, &Transform2), With<Player>>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mouse_buttons: Res<Input<MouseButton>>,
    mouse: Res<Mouse>,
) {
    for (player_entity, mut character_controller, transform) in query.iter_mut() {
        character_controller.movement = Vec2::ZERO;
        if mouse_buttons.just_pressed(MouseButton::Left) {
            let velocity = (mouse.position - transform.translation).normalize() * 900.;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Vec2::new(8., 8.).into(),
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Transform2::from_translation(transform.translation),
                Bullet { velocity },
                Hurtbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(8., 8.),
                    },
                    for_entity: Some(player_entity),
                    auto_despawn: true,
                    flags: 1,
                    knockback_type: HurtboxKnockbackType::None,
                    damage: 1.,
                },
            ));
        }
        if keys.pressed(KeyCode::W) {
            character_controller.movement.y += 1.;
        }
        if keys.pressed(KeyCode::S) {
            character_controller.movement.y -= 1.;
        }
        if keys.pressed(KeyCode::A) {
            character_controller.movement.x -= 1.;
        }
        if keys.pressed(KeyCode::D) {
            character_controller.movement.x += 1.;
        }
    }
}

fn bullet_update(mut query: Query<(&mut Transform2, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in query.iter_mut() {
        transform.translation += bullet.velocity * time.delta_seconds();
    }
}
