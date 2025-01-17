use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const OFFSET: Vec2 = Vec2::new(0., 20.);
const BORDER_SIZE: f32 = 10.;

pub struct BossHealthbarPlugin;

impl Plugin for BossHealthbarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BossHealthbarSpawnEvent>()
            .add_event::<BossHealthbarDespawnEvent>()
            .add_system(boss_healthbar_spawn)
            .add_system(boss_healthbar_despawn)
            .add_system(boss_healthbar_update);
    }
}

#[derive(Clone)]
pub struct BossHealthbarSpawnEvent {
    pub name: String,
    pub entity: Entity,
}

#[derive(Default, Clone, Copy)]
pub struct BossHealthbarDespawnEvent;

#[derive(Component)]
pub struct BossHealthbar {
    entity: Entity,
}

#[derive(Component)]
pub struct BossHealthbarBar;

fn boss_healthbar_spawn(
    mut ev_spawn: EventReader<BossHealthbarSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    query: Query<Entity, With<BossHealthbar>>,
) {
    for event in ev_spawn.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands
            .spawn((
                VisibilityBundle {
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
                TransformBundle::default(),
                FollowCamera { offset: Vec2::ZERO },
                Transform2::new().without_pixel_perfect(),
                BossHealthbar {
                    entity: event.entity,
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(250., 26.).into(),
                            color: Color::rgba(0., 0., 0., 0.9),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Transform2::from_translation(Vec2::new(0., 332.) + OFFSET)
                        .with_depth(DEPTH_LAYER_UI_BOSS_HEALTHBAR_NAME_BACKGROUND)
                        .without_pixel_perfect(),
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            event.name.clone(),
                            TextStyle {
                                font: asset_library.font_bold.clone(),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_alignment(TextAlignment::Center),
                        text_anchor: Anchor::Center,
                        ..Default::default()
                    },
                    Transform2::from_translation(Vec2::new(0., 332.) + OFFSET)
                        .with_depth(DEPTH_LAYER_UI_BOSS_HEALTHBAR_NAME),
                ));
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(650., 30.).into(),
                            color: Color::BLACK,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Transform2::from_translation(Vec2::new(0., 304.) + OFFSET)
                        .with_depth(DEPTH_LAYER_UI_BOSS_HEALTHBAR_BORDER)
                        .without_pixel_perfect(),
                ));
                parent.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Vec2::new(650. - BORDER_SIZE, 30. - BORDER_SIZE).into(),
                            color: Color::RED,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Transform2::from_translation(Vec2::new(0., 304.) + OFFSET)
                        .with_depth(DEPTH_LAYER_UI_BOSS_HEALTHBAR)
                        .without_pixel_perfect(),
                    BossHealthbarBar,
                ));
            });
    }
}

fn boss_healthbar_despawn(
    mut ev_despawn: EventReader<BossHealthbarDespawnEvent>,
    mut commands: Commands,
    query: Query<Entity, With<BossHealthbar>>,
) {
    for _ in ev_despawn.iter() {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn boss_healthbar_update(
    mut query: Query<(Entity, &BossHealthbar, &Children, &mut Visibility)>,
    mut bar_query: Query<&mut Transform2, With<BossHealthbarBar>>,
    health_query: Query<&Health>,
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    for (entity, healthbar, children, mut visibility) in query.iter_mut() {
        *visibility = if !game_state.quests.pirate_dialogue() {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        if let Ok(health) = health_query.get(healthbar.entity) {
            let health_percent = health.value / health.max;
            for child in children.iter() {
                if let Ok(mut bar_transform) = bar_query.get_mut(*child) {
                    bar_transform.scale.x = health_percent;
                    bar_transform.translation.x =
                        (-(650. - BORDER_SIZE) * 0.5) * (1. - health_percent);
                }
            }
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }
}
