use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;

pub struct MarkerPlugin;

impl Plugin for MarkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MarkerSpawnEvent>()
            .add_system(marker_spawn)
            .add_system(marker_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct MarkerSpawnEvent;

#[derive(Component)]
pub struct MarkerIcon;

#[derive(Component)]
pub struct MarkerArrow;

fn marker_spawn(
    mut ev_spawn: EventReader<MarkerSpawnEvent>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    for _ in ev_spawn.iter() {
        commands
            .spawn((
                VisibilityBundle::default(),
                TransformBundle::default(),
                FollowCamera { offset: Vec2::ZERO },
                Transform2::new().without_pixel_perfect(),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(1., 1., 1., 0.),
                                ..Default::default()
                            },
                            texture: asset_library.sprite_world_quest_marker_icon.clone(),
                            ..Default::default()
                        },
                        Transform2::from_xy(0., 0.)
                            .with_depth(DEPTH_LAYER_UI_MARKER_ICON)
                            .with_scale(Vec2::ONE * 0.25)
                            .without_pixel_perfect(),
                        MarkerIcon,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::rgba(1., 1., 1., 0.),
                                    ..Default::default()
                                },
                                texture: asset_library.sprite_world_quest_marker_arrow.clone(),
                                ..Default::default()
                            },
                            MarkerArrow,
                            Transform2::from_xy(0., 0.)
                                .with_depth(DEPTH_LAYER_UI_MARKER_ARROW)
                                .without_pixel_perfect(),
                        ));
                    });
            });
    }
}

fn marker_update(
    mut queries: ParamSet<(
        Query<&GlobalTransform, With<Camera>>,
        Query<(&mut Transform2, &mut Sprite), With<MarkerIcon>>,
        Query<(&mut Transform2, &mut Sprite), With<MarkerArrow>>,
    )>,
    game_state: Res<GameState>,
    world_locations: Res<WorldLocations>,
) {
    let camera_position = if let Ok(transform) = queries.p0().get_single() {
        transform.translation().truncate()
    } else {
        Vec2::ZERO
    };
    let objective_position = if let Some(objective_marker) = game_state.quests.marker() {
        Some(world_locations.get_single_position(objective_marker))
    } else {
        None
    };
    if let Some(objective_position) = objective_position {
        let difference = (objective_position - camera_position).normalize_or_zero();
        let distance = objective_position.distance(camera_position);
        let alpha = ((distance - 200.) / 400.).clamp(0., 1.);
        for (mut icon_transform, mut icon_sprite) in queries.p1().iter_mut() {
            icon_transform.translation = difference * 250.;
            icon_sprite.color.set_a(alpha);
        }
        for (mut arrow_transform, mut arrow_sprite) in queries.p2().iter_mut() {
            arrow_transform.rotation =
                Vec2::X.angle_between(difference) + std::f32::consts::PI * 0.5;
            arrow_sprite.color.set_a(alpha);
        }
    } else {
        for (_, mut icon_sprite) in queries.p1().iter_mut() {
            icon_sprite.color.set_a(0.);
        }
        for (_, mut arrow_sprite) in queries.p2().iter_mut() {
            arrow_sprite.color.set_a(0.);
        }
    }
}
