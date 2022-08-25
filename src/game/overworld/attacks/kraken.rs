use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct KrakenPlugin;

impl Plugin for KrakenPlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<Kraken, KrakenSound>()
            .add_system(kraken_fire)
            .add_system(tentacle_move)
            .add_system(kraken_sound);
    }
}

#[derive(Component, Default)]
pub struct Kraken {
    pub shoot: bool,
    pub hurt_flags: u32,
}

#[derive(Component)]
struct Tentacle {
    pub velocity: Vec2,
}

#[derive(Component, Default)]
struct KrakenSound;

fn kraken_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<KrakenSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands
            .entity(event.entity)
            .insert_bundle(Transform2Bundle::default())
            .insert(AudioPlusSource::new(
                asset_library
                    .sound_effects
                    .sfx_overworld_attack_forward_cannons
                    .clone(),
            ));
    }
}

fn kraken_fire(
    mut query: Query<(Entity, &mut Kraken, &Boat, &GlobalTransform, &Children)>,
    mut sound_query: Query<&mut AudioPlusSource, With<KrakenSound>>,
    mut commands: Commands,
) {
    for (boat_entity, mut kraken, boat, global_transform, children) in query.iter_mut() {
        if kraken.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            for shoot_side in 0..2 {
                let forward = Vec2::from_angle(boat.direction);
                let mult = if shoot_side == 0 { 1. } else { -1. };
                let side = forward.perp() * mult;
                let position =
                    global_transform.translation().truncate() + forward * 150. + side * 15.;
                let velocity = forward * 100.;
                let (mut scale, _, _) = global_transform.to_scale_rotation_translation();
                scale *= 0.5;
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(28., 28.).into(),
                            color: Color::PURPLE,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(
                        Transform2::from_translation(position)
                            .with_depth((DepthLayer::Entity, 0.0))
                            .with_scale(scale.truncate()),
                    )
                    .insert(Hurtbox {
                        shape: CollisionShape::Rect {
                            size: Vec2::new(28., 28.),
                        },
                        for_entity: Some(boat_entity),
                        auto_despawn: true,
                        flags: kraken.hurt_flags,
                        knockback_type: HurtboxKnockbackType::Velocity(velocity * 0.01),
                    })
                    .insert(YDepth::default())
                    .insert(Tentacle { velocity })
                    .insert(TimeToLive::new(1.0));
            }
        }
        kraken.shoot = false;
    }
}

fn tentacle_move(mut query: Query<(&mut Transform2, &Tentacle)>, time: Res<Time>) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += cannon_ball.velocity * time.delta_seconds()
    }
}
