use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct ForwardCannonsPlugin;

impl Plugin for ForwardCannonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(forward_cannons_fire)
            .add_system(forward_cannon_ball_move);
    }
}

#[derive(Component, Default)]
pub struct ForwardCannons {
    pub shoot: bool,
    pub hurt_flags: u32,
    pub level: ForwardCannonsLevel,
}

#[derive(Default)]
pub struct ForwardCannonsLevel(pub u32);

impl ForwardCannonsLevel {
    fn stats(&self) -> ForwardCannonsStats {
        let level = self.0 as f32;
        ForwardCannonsStats {
            damage: level * 0.8,
            scale: 0.8 + level / 5.,
            speed: 1200. + level * 100.,
            hit_multiple: self.0 >= 5,
            knockback_intensity: if self.0 >= 5 { 0.004 } else { 0.0075 },
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ForwardCannonsStats {
    damage: f32,
    scale: f32,
    speed: f32,
    hit_multiple: bool,
    knockback_intensity: f32,
}

#[derive(Component)]
struct ForwardCannonBall {
    pub velocity: Vec2,
}

fn forward_cannons_fire(
    mut query: Query<(Entity, &mut ForwardCannons, &Boat, &GlobalTransform)>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (boat_entity, mut forward_cannons, boat, global_transform) in query.iter_mut() {
        if forward_cannons.shoot {
            let stats = forward_cannons.level.stats();
            let audio_entity = commands
                .spawn((
                    Transform2Bundle {
                        transform2: Transform2::new(),
                        ..Default::default()
                    },
                    AudioPlusSource::new(
                        asset_library
                            .sound_effects
                            .sfx_overworld_attack_forward_cannons
                            .clone(),
                    )
                    .as_playing(),
                    TimeToLive { seconds: 3. },
                ))
                .id();
            commands.entity(boat_entity).add_child(audio_entity);
            let forward = Vec2::from_angle(boat.direction);
            let position = global_transform.translation().truncate() + forward * 80.;
            let velocity = forward * stats.speed;
            let (mut scale, _, _) = global_transform.to_scale_rotation_translation();
            scale *= stats.scale;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        ..Default::default()
                    },
                    texture: asset_library.sprite_bullet_note.clone(),
                    ..Default::default()
                },
                Transform2::from_translation(position)
                    .with_depth((DepthLayer::Entity, 0.0))
                    .with_scale(scale.truncate()),
                Hurtbox {
                    shape: CollisionShape::Rect {
                        size: Vec2::new(14., 14.) * stats.scale,
                    },
                    for_entity: Some(boat_entity),
                    auto_despawn: if stats.hit_multiple { false } else { true },
                    flags: forward_cannons.hurt_flags,
                    knockback_type: HurtboxKnockbackType::Velocity(
                        velocity * stats.knockback_intensity,
                    ),
                    damage: stats.damage,
                },
                YDepth::default(),
                ForwardCannonBall { velocity },
                TimeToLive::new(1.0),
            ));
        }
        forward_cannons.shoot = false;
    }
}

fn forward_cannon_ball_move(
    mut query: Query<(&mut Transform2, &ForwardCannonBall)>,
    time: Res<Time>,
) {
    for (mut transform, cannon_ball) in query.iter_mut() {
        transform.translation += cannon_ball.velocity * time.delta_seconds()
    }
}
