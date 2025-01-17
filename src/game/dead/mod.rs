use crate::common::prelude::*;
use crate::game::prelude::*;
use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Default, Resource)]
struct DeadState {
    can_respawn: bool,
    can_leave: bool,
}

pub struct DeadPlugin;

impl Plugin for DeadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeadState>()
            .add_system(town_init.in_schedule(OnEnter(AppState::Dead)))
            .add_system(town_update.in_set(OnUpdate(AppState::Dead)));
    }
}

fn town_init(
    mut state: ResMut<DeadState>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut game_state: ResMut<GameState>,
    mut cutscenes: ResMut<Cutscenes>,
    mut dialogue: ResMut<Dialogue>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    cutscenes.clear();
    dialogue.clear();
    screen_fade.fade_in(0.2);
    state.can_respawn = game_state.restore_checkpoint();
    state.can_leave = false;
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_library.sprite_dead.clone(),
            ..Default::default()
        },
        Transform2::from_xy(0., 100.).with_depth(DEPTH_LAYER_DEATH_SCREEN),
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                if state.can_respawn {
                    "Yer swimmin' with the fishes now\nLeft click to respawn at last town"
                } else {
                    "Yer swimmin' with the fishes now\nLeft click to restart"
                },
                TextStyle {
                    font: asset_library.font_bold.clone(),
                    font_size: 42.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            text_anchor: Anchor::Center,
            ..Default::default()
        },
        Transform2::from_xy(0., -175.).with_depth(DEPTH_LAYER_DEATH_SCREEN),
    ));
}

fn town_update(
    mut state: ResMut<DeadState>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut screen_fade: ResMut<ScreenFade>,
) {
    if !state.can_leave
        && (keys.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left))
    {
        state.can_leave = true;
        screen_fade.fade_out(0.5);
    }
    if state.can_leave && screen_fade.faded_out() {
        if state.can_respawn {
            app_state.set(AppState::Overworld);
        } else {
            app_state.set(AppState::MainMenu);
        }
    }
}
