use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

const EXPERIENCE_UI_POSITION: Vec2 = Vec2::new(-435., -355.);
const EXPERIENCE_UI_SCALE: f32 = 0.28;

const EXPERIENCE_UI_LEVEL_LABEL_POSITION: Vec2 = Vec2::new(-565., -45.);
const EXPERIENCE_UI_LEVEL_LABEL_FONT_SIZE: f32 = 75.;

const EXPERIENCE_UI_LEVEL_POSITION: Vec2 = Vec2::new(-477., -60.);
const EXPERIENCE_UI_LEVEL_FONT_SIZE: f32 = 110.;

const EXPERIENCE_UI_SKILLPOINT_POSITION: Vec2 = Vec2::new(-130., 430.);
const EXPERIENCE_UI_SKILLPOINT_BG_SIZE: f32 = 1.4;
const EXPERIENCE_UI_SKILLPOINT_TEXT_FONT_SIZE: f32 = 72.;

pub struct ExperienceUiPlugin;

impl Plugin for ExperienceUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExperienceUiSpawnEvent>()
            .add_system(experience_ui_spawn)
            .add_system(experience_ui_bar_update)
            .add_system(experience_ui_level_update)
            .add_system(experience_ui_skill_points_update);
    }
}

#[derive(Default, Clone, Copy)]
pub struct ExperienceUiSpawnEvent;

#[derive(Component)]
pub struct ExperienceUiBar;

#[derive(Component)]
pub struct ExperienceUiLevelText;

#[derive(Component)]
pub struct ExperienceUiSkillPointsBg;

#[derive(Component)]
pub struct ExperienceUiSkillPointsText;

fn experience_ui_spawn(
    mut ev_spawn: EventReader<ExperienceUiSpawnEvent>,
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
                        Transform2Bundle {
                            transform2: Transform2::from_translation(EXPERIENCE_UI_POSITION)
                                .with_scale(Vec2::ONE * EXPERIENCE_UI_SCALE),
                            ..Default::default()
                        },
                        VisibilityBundle::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            SpriteBundle {
                                texture: asset_library.sprite_experience_bar_bg.clone(),
                                ..Default::default()
                            },
                            Transform2::from_xy(7., -5.)
                                .with_depth(DEPTH_LAYER_UI_EXPERIENCE_BAR_BACK),
                        ));
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Vec2::new(586., 50.).into(),
                                    color: Color::rgb_u8(255, 209, 22),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            Transform2::from_xy(0., 0.).with_depth(DEPTH_LAYER_UI_EXPERIENCE_BAR),
                            ExperienceUiBar,
                        ));
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    "Lvl",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_LEVEL_LABEL_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Left),
                                text_anchor: Anchor::BottomRight,
                                ..Default::default()
                            },
                            Transform2::from_translation(EXPERIENCE_UI_LEVEL_LABEL_POSITION)
                                .with_depth(DEPTH_LAYER_UI_EXPERIENCE_LEVEL),
                        ));
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_LEVEL_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Left),
                                text_anchor: Anchor::BottomRight,
                                ..Default::default()
                            },
                            Transform2::from_translation(EXPERIENCE_UI_LEVEL_POSITION)
                                .with_depth(DEPTH_LAYER_UI_EXPERIENCE_LEVEL),
                            ExperienceUiLevelText,
                        ));
                        parent.spawn((
                            SpriteBundle {
                                texture: asset_library.sprite_experience_skill_point_bg.clone(),
                                ..Default::default()
                            },
                            Transform2::from_translation(EXPERIENCE_UI_SKILLPOINT_POSITION)
                                .with_depth(DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_BG)
                                .with_scale(Vec2::ONE * EXPERIENCE_UI_SKILLPOINT_BG_SIZE),
                            ExperienceUiSkillPointsBg,
                        ));
                        parent.spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_library.font_bold.clone(),
                                        font_size: EXPERIENCE_UI_SKILLPOINT_TEXT_FONT_SIZE,
                                        color: Color::WHITE,
                                    },
                                )
                                .with_alignment(TextAlignment::Center),
                                text_anchor: Anchor::Center,
                                ..Default::default()
                            },
                            Transform2::from_translation(
                                EXPERIENCE_UI_SKILLPOINT_POSITION + Vec2::new(60., -10.),
                            )
                            .with_depth(DEPTH_LAYER_UI_EXPERIENCE_SKILLPOINT_TEXT),
                            ExperienceUiSkillPointsText,
                        ));
                    });
            });
    }
}

fn experience_ui_bar_update(
    mut query: Query<&mut Transform2, With<ExperienceUiBar>>,
    game_state: Res<GameState>,
) {
    let experience_percent = game_state.experience / game_state.experience_max();
    for mut transform in query.iter_mut() {
        transform.scale.x = experience_percent;
        transform.translation.x = -(586. * 0.5) * (1.0 - experience_percent);
    }
}

fn experience_ui_level_update(
    mut query: Query<&mut Text, With<ExperienceUiLevelText>>,
    game_state: Res<GameState>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{}", game_state.level);
    }
}

fn experience_ui_skill_points_update(
    mut bg_query: Query<&mut Visibility, With<ExperienceUiSkillPointsBg>>,
    mut text_query: Query<&mut Text, With<ExperienceUiSkillPointsText>>,
    game_state: Res<GameState>,
) {
    let skill_points = game_state.skill_points;
    if skill_points > 0 {
        for mut bg_visibility in bg_query.iter_mut() {
            *bg_visibility = Visibility::Inherited;
        }
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!(
                "{} Skill Point{} to spend at town!",
                skill_points,
                if skill_points == 1 { "" } else { "s" }
            );
        }
    } else {
        for mut bg_visibility in bg_query.iter_mut() {
            *bg_visibility = Visibility::Hidden;
        }
        for mut text in text_query.iter_mut() {
            text.sections[0].value = "".into();
        }
    }
}
