use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct ShockwavePlugin;

impl Plugin for ShockwavePlugin {
    fn build(&self, app: &mut App) {
        app.add_component_child::<Shockwave, ShockwaveSound>()
            .add_system(shockwave_fire)
            .add_system(shockwave_update)
            .add_system(shockwave_sound);
    }
}

#[derive(Component, Default)]
pub struct Shockwave {
    pub shoot: bool,
    pub hurt_flags: u32,
    pub boss: bool,
}

#[derive(Component, Default)]
struct ShockwaveWave {
    time_alive: f32,
}

#[derive(Component, Default)]
struct ShockwaveSound;

#[derive(Component, Default)]
struct ShockwaveSprite;

fn shockwave_sound(
    mut commands: Commands,
    mut ev_created: EventReader<ComponentChildCreatedEvent<ShockwaveSound>>,
    asset_library: Res<AssetLibrary>,
) {
    for event in ev_created.iter() {
        commands.entity(event.entity).insert(AudioPlusSource::new(
            asset_library
                .sound_effects
                .sfx_overworld_attack_shockwave
                .clone(),
        ));
    }
}

fn shockwave_fire(
    mut query: Query<(&mut Shockwave, Entity, &Children)>,
    mut sound_query: Query<&mut AudioPlusSource, With<ShockwaveSound>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for (mut shockwave, entity, children) in query.iter_mut() {
        if shockwave.shoot {
            for child in children.iter() {
                if let Ok(mut sound) = sound_query.get_mut(*child) {
                    sound.play();
                }
            }
            let child_entity =
                commands
                    .spawn_bundle(Transform2Bundle {
                        ..Default::default()
                    })
                    .insert_bundle(VisibilityBundle::default())
                    .insert(ShockwaveWave { time_alive: 0. })
                    .insert(TimeToLive::new(0.75))
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Vec2::new(1., 1.).into(),
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                                texture: asset_library.sprite_shockwave_vfx.clone(),
                                ..Default::default()
                            })
                            .insert(Transform2::new().with_depth(DEPTH_LAYER_SHOCKWAVE))
                            .insert(ShockwaveSprite);
                        parent
                            .spawn_bundle(Transform2Bundle {
                                ..Default::default()
                            })
                            .insert(Transform2::new().with_depth((DepthLayer::Front, 0.98)))
                            .insert(Hurtbox {
                                shape: CollisionShape::Rect {
                                    size: Vec2::new(400., 400.),
                                },
                                for_entity: Some(entity),
                                auto_despawn: false,
                                flags: shockwave.hurt_flags,
                                knockback_type: HurtboxKnockbackType::Difference(
                                    if shockwave.boss { 15. } else { 5. },
                                ),
                            })
                            .insert(TimeToLive { seconds: 0.05 });
                    })
                    .id();
            commands.entity(entity).add_child(child_entity);
        }
        shockwave.shoot = false;
    }
}

fn shockwave_update(
    mut query: Query<(&mut ShockwaveWave, &Children)>,
    mut child_query: Query<(&mut Transform2, &mut Sprite), With<ShockwaveSprite>>,
    time: Res<Time>,
) {
    for (mut wave, children) in query.iter_mut() {
        wave.time_alive += time.delta_seconds();
        for child in children.iter() {
            if let Ok((mut transform, mut sprite)) = child_query.get_mut(*child) {
                transform.scale = Vec2::ONE * (32. + wave.time_alive * 2000.);
                sprite.color.set_a(1. - wave.time_alive / 0.75);
            }
        }
    }
}
