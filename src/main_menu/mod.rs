use crate::{
    common::{label::Label, prelude::*},
    game::state::GameState,
    DEV_BUILD,
};
use audio_plus::prelude::*;
use bevy::{prelude::*, sprite::Anchor, window::WindowMode};

use self::slider::VolumeSliderSpawnEvent;

const LOGO_POSITION: Vec2 = Vec2::new(0., 115.);
const LOGO_SCALE: Vec2 = Vec2::new(0.84, 0.84);
const LOGO_MOVEMENT_GROW: Vec2 = Vec2::new(1., 1.4);
const BUTTON_SCALE: Vec2 = Vec2::new(0.72, 0.72);
const BUTTON_POSITION: Vec2 = Vec2::new(80., -200.);
const BUTTON_TEXT_SCALE: Vec2 = Vec2::new(0.8, 0.8);

#[derive(Default, Resource)]
struct MenuState {
    play: bool,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(slider::VolumeSliderPlugin)
            .init_resource::<MenuState>()
            .add_system(menu_setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(menu_fade.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(menu_logo)
            .add_system(menu_shine)
            .add_system(menu_button)
            .add_system(menu_background_move)
            .add_system(menu_outro_debug.in_set(OnUpdate(AppState::MainMenu)))
            .add_system(menu_fullscreen);
    }
}

#[derive(Component)]
struct Button {
    shape: CollisionShape,
    last_hover: bool,
    clicked: bool,
    audio_hover: Entity,
    audio_click: Entity,
    audio_click_confirm: Entity,
}

#[derive(Component)]
struct ButtonText {
    normal: Handle<Image>,
    hover: Handle<Image>,
    press: Handle<Image>,
}

#[derive(Component)]
struct Sound;

#[derive(Component)]
struct Logo {
    x: f32,
}

#[derive(Component)]
struct Shine {
    x: f32,
}

#[derive(Component)]
struct Background;

#[derive(Component)]
struct Fullscreen;

fn menu_setup(
    mut menu_state: ResMut<MenuState>,
    mut screen_fade: ResMut<ScreenFade>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut cutscenes: ResMut<Cutscenes>,
    mut dialogue: ResMut<Dialogue>,
    mut ev_volume_slider_spawn: EventWriter<VolumeSliderSpawnEvent>,
) {
    *menu_state = MenuState::default();
    cutscenes.clear();
    dialogue.clear();
    screen_fade.fade_in(1.);
    ev_volume_slider_spawn.send_default();
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        AudioPlusSource::new(asset_library.sound_effects.sfx_menu_ambient.clone()).as_looping(),
        Sound,
    ));
    commands.spawn((
        AudioPlusSource::new(asset_library.sound_effects.sfx_menu_music.clone()).as_looping(),
        Sound,
    ));
    let hover_audio = commands
        .spawn(AudioPlusSource::new(
            asset_library.sound_effects.sfx_menu_button_hover.clone(),
        ))
        .id();
    let click_audio = commands
        .spawn(AudioPlusSource::new(
            asset_library.sound_effects.sfx_menu_button_click.clone(),
        ))
        .id();
    let click_confirm_audio = commands
        .spawn(AudioPlusSource::new(
            asset_library
                .sound_effects
                .sfx_menu_button_click_confirm
                .clone(),
        ))
        .id();
    commands.spawn((
        SpriteBundle {
            texture: asset_library.menu_sprite_back.clone(),
            ..Default::default()
        },
        Transform2::new()
            .with_scale(Vec2::ONE * 0.73)
            .with_depth((DepthLayer::Front, 0.))
            .without_pixel_perfect(),
        Background,
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            texture: asset_library.menu_sprite_logo.clone(),
            ..Default::default()
        },
        Transform2::from_xy(0., 90.).with_depth((DepthLayer::Front, 0.2)),
        Label("Logo".to_owned()),
        Logo { x: 0. },
    ));
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1., 1., 1., 0.),
                ..Default::default()
            },
            texture: asset_library.menu_sprite_shine.clone(),
            ..Default::default()
        },
        Transform2::from_xy(0., 90.)
            .with_scale(LOGO_SCALE * 0.8)
            .with_depth((DepthLayer::Front, 0.1)),
        Shine { x: 0. },
        Label("Shine".to_owned()),
    ));
    commands
        .spawn((
            SpriteBundle {
                texture: asset_library.menu_sprite_button_back.clone(),
                ..Default::default()
            },
            Button {
                shape: CollisionShape::Rect {
                    size: Vec2::new(406., 159.) * BUTTON_SCALE,
                },
                last_hover: false,
                clicked: false,
                audio_hover: hover_audio,
                audio_click: click_audio,
                audio_click_confirm: click_confirm_audio,
            },
            Transform2::from_translation(BUTTON_POSITION)
                .with_scale(Vec2::ONE * BUTTON_SCALE)
                .with_depth((DepthLayer::Front, 0.3)),
            Label("Play Button".to_owned()),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: asset_library.menu_sprite_button_play_normal.clone(),
                    ..Default::default()
                },
                Transform2::new()
                    .with_scale(BUTTON_TEXT_SCALE)
                    .with_depth((DepthLayer::Front, 0.4)),
                ButtonText {
                    normal: asset_library.menu_sprite_button_play_normal.clone(),
                    hover: asset_library.menu_sprite_button_play_hover.clone(),
                    press: asset_library.menu_sprite_button_play_press.clone(),
                },
            ));
            parent.spawn((
                SpriteBundle {
                    texture: asset_library.menu_sprite_skull.clone(),
                    ..Default::default()
                },
                Transform2::from_xy(-210., 12.)
                    .with_scale(Vec2::ONE * 1.2)
                    .with_depth((DepthLayer::Front, 0.4)),
            ));
        });

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        SpriteBundle {
            texture: asset_library.menu_fullscreen_recommended.clone(),
            ..Default::default()
        },
        Transform2::from_xy(553., -321.)
            .with_depth((DepthLayer::Front, 0.2))
            .with_scale(Vec2::ONE * 0.32),
    ));

    #[cfg(not(target_arch = "wasm32"))]
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(40.)),
                ..Default::default()
            },
            texture: asset_library.menu_fullscreen.clone(),
            ..Default::default()
        },
        Transform2::from_xy(640. - 40., -384. + 30.).with_depth((DepthLayer::Front, 0.2)),
        Clickable {
            shape: CollisionShape::Rect {
                size: Vec2::splat(40.),
            },
            ..Default::default()
        },
        Fullscreen,
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "A game for Bevy Jam #2",
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 48.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Left),
            text_anchor: Anchor::BottomRight,
            ..Default::default()
        },
        Transform2::from_xy(-632., -378.)
            .with_depth((DepthLayer::Front, 0.2))
            .with_scale(Vec2::ONE * 0.5),
    ));

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "v1.2 (Bevy 0.10.0)",
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 48.0,
                    color: Color::BLACK,
                },
            )
            .with_alignment(TextAlignment::Left),
            text_anchor: Anchor::BottomRight,
            ..Default::default()
        },
        Transform2::from_xy(-632., -352.)
            .with_depth((DepthLayer::Front, 0.2))
            .with_scale(Vec2::ONE * 0.5),
    ));
}

fn menu_logo(mut query: Query<(&mut Logo, &mut Transform2, &mut Sprite)>, time: Res<Time>) {
    for (mut logo, mut transform, mut sprite) in query.iter_mut() {
        logo.x += time.delta_seconds() * 3.;
        logo.x = logo.x.clamp(0., 1.);
        transform.translation =
            Vec2::new(0., 300.).lerp(LOGO_POSITION, ease(Easing::BackOut, logo.x));
        transform.scale =
            (LOGO_SCALE * LOGO_MOVEMENT_GROW).lerp(LOGO_SCALE, ease(Easing::BackOut, logo.x));
        sprite.color.set_a(ease(Easing::QuartOut, logo.x));
    }
}

fn menu_shine(mut query: Query<(&mut Shine, &mut Transform2, &mut Sprite)>, time: Res<Time>) {
    for (mut shine, mut transform, mut sprite) in query.iter_mut() {
        shine.x += time.delta_seconds();
        sprite
            .color
            .set_a(ease(Easing::QuartOut, (shine.x * 3.).clamp(0., 1.)));
        transform.rotation = shine.x * 0.2;
        transform.scale = Vec2::new(shine.x.sin() * 0.2 + 0.6, shine.x.cos() * 0.2 + 0.6)
    }
}

fn play_sound(entity: Entity, sfx_query: &mut Query<&mut AudioPlusSource>) {
    if let Ok(mut source) = sfx_query.get_mut(entity) {
        source.play();
    }
}

fn menu_button(
    mut screen_fade: ResMut<ScreenFade>,
    mut button_query: Query<(&mut Button, &GlobalTransform, &Children, &mut Transform2)>,
    mut text_query: Query<(&ButtonText, &mut Handle<Image>)>,
    mut sfx_query: Query<&mut AudioPlusSource>,
    sound_query: Query<Entity, With<Sound>>,
    mouse: Res<Mouse>,
    input: Res<Input<MouseButton>>,
    mut menu_state: ResMut<MenuState>,
) {
    for (mut button, transform, children, mut transform2) in button_query.iter_mut() {
        let hover = !menu_state.play
            && button.shape.overlaps(
                transform.translation().truncate(),
                CollisionShape::Point,
                mouse.position,
            );
        if hover != button.last_hover {
            if hover && !button.clicked {
                play_sound(button.audio_hover, &mut sfx_query);
            }
            button.last_hover = hover;
        }
        if hover && input.just_pressed(MouseButton::Left) {
            button.clicked = true;
            play_sound(button.audio_click, &mut sfx_query);
        }
        if button.clicked && input.just_released(MouseButton::Left) {
            if hover {
                for entity in sound_query.iter() {
                    if let Ok(mut source) = sfx_query.get_mut(entity) {
                        source.stop();
                    }
                }
                menu_state.play = true;
                play_sound(button.audio_click_confirm, &mut sfx_query);
                screen_fade.fade_out(1.8);
            }
            button.clicked = false;
        }
        transform2.translation = BUTTON_POSITION;
        if button.clicked && hover {
            transform2.translation += Vec2::new(-2., -2.);
        }
        for child in children.iter() {
            if let Ok((text, mut image)) = text_query.get_mut(*child) {
                if button.clicked && hover {
                    *image = text.press.clone();
                } else if hover {
                    *image = text.hover.clone();
                } else {
                    *image = text.normal.clone();
                }
            }
        }
    }
}

fn menu_fade(
    menu_state: Res<MenuState>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
    screen_fade: Res<ScreenFade>,
) {
    if menu_state.play && screen_fade.faded_out() {
        *game_state = GameState::default();
        app_state.set(AppState::IntroCutscene);
    }
}

fn menu_background_move(mut query: Query<&mut Transform2, With<Background>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        let time = time.elapsed_seconds();
        let time_x = (time * 0.1) % 2.;
        let time_y = (time * 0.12) % 2.;
        let baf_x = if time_x < 1. { time_x } else { 2.0 - time_x };
        let baf_y = if time_y < 1. { time_y } else { 2.0 - time_y };
        let x = ease(Easing::BackInOut, baf_x) * 10. - 5.;
        let y = ease(Easing::BackInOut, baf_y) * 10. - 5.;
        transform.translation = Vec2::new(x, y);
    }
}

fn menu_outro_debug(mut input: ResMut<Input<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if DEV_BUILD {
        if input.just_pressed(KeyCode::Key0) {
            app_state.set(AppState::OutroCutscene);
            input.reset(KeyCode::Key0);
        }
    }
}

fn menu_fullscreen(
    mut fullscreen_query: Query<(&mut Sprite, &Clickable), With<Fullscreen>>,
    mut window_query: Query<&mut Window>,
) {
    for (mut fullscreen_sprite, fullscreen_clickable) in fullscreen_query.iter_mut() {
        fullscreen_sprite
            .color
            .set_a(if fullscreen_clickable.hovered {
                1.
            } else {
                0.6
            });
        if fullscreen_clickable.confirmed {
            if let Some(mut window) = window_query.get_single_mut().ok() {
                if window.mode == WindowMode::BorderlessFullscreen {
                    window.mode = WindowMode::Windowed;
                } else {
                    window.mode = WindowMode::BorderlessFullscreen;
                }
            }
        }
    }
}

pub mod slider;
