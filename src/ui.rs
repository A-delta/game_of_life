use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use std::time::Duration;

use crate::camera::MainCamera;
use crate::game_of_life_logic::Universe;

#[derive(Resource)]
pub struct IterationTimer(pub Timer);

#[derive(Component)]
struct IterationDuration(f32);

// for paint brush
#[derive(Resource)]
struct LastChangedTile(usize, usize);

#[derive(Resource, Debug)]
pub enum UiState {
    Play,
    Pause,
}

fn build_ui(
    mut contexts: EguiContexts,
    state: Option<ResMut<UiState>>,
    mut query: Query<&mut IterationDuration>,
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Button::new("Play/Stop [Space]")).clicked() {
            if let Some(mut state) = state {
                invert_state(&mut state)
            };
        };
        ui.add(egui::Slider::new(&mut query.single_mut().0, 0.0..=1.0));
    });
}
fn build_grid(state: ResMut<UiState>, query_universe: Query<&Universe>, mut gizmos: Gizmos) {
    let cell_size = 25.0;
    let uni = query_universe.single();
    if let UiState::Pause = *state {
        for i in 1..uni.width {
            gizmos.line_2d(
                Vec2::new(
                    i as f32 * cell_size - (cell_size + cell_size * uni.width as f32) / 2.0,
                    -(cell_size + cell_size * uni.height as f32) / 2.0,
                ),
                Vec2::new(
                    i as f32 * cell_size - (cell_size + cell_size * uni.width as f32) / 2.0,
                    (cell_size + cell_size * (uni.height - 2) as f32) / 2.0,
                ),
                Color::GRAY,
            );
        }

        for i in 1..uni.height {
            gizmos.line_2d(
                Vec2::new(
                    -(cell_size + cell_size * uni.width as f32) / 2.0,
                    i as f32 * cell_size - (cell_size + cell_size * uni.height as f32) / 2.0,
                ),
                Vec2::new(
                    (cell_size + cell_size * (uni.width - 2) as f32) / 2.0,
                    i as f32 * cell_size - (cell_size + cell_size * uni.height as f32) / 2.0,
                ),
                Color::GRAY,
            );
        }
    }
}

fn invert_state(state: &mut UiState) {
    match *state {
        UiState::Play => *state = UiState::Pause,
        UiState::Pause => *state = UiState::Play,
    }
}
fn ui_controls(
    state: Option<ResMut<UiState>>,
    mut last_modified: ResMut<LastChangedTile>,
    keys: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut query_universe: Query<&mut Universe>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if let Some(mut state) = state {
            invert_state(&mut state)
        }
    }
    if buttons.pressed(MouseButton::Left) {
        // Games typically only have one window (the primary window)
        if let Some(position) = q_windows.single().cursor_position() {
            let cell_size = 25.0;
            let mut uni = query_universe.single_mut();

            let (camera, camera_transform) = camera_query.single();
            if let Some(position_world) = camera.viewport_to_world_2d(camera_transform, position) {
                let (i, j) = (
                    ((position_world.y) / cell_size + 0.5 + uni.height as f32 / 2.0) as usize,
                    ((position_world.x) / cell_size + 0.5 + uni.width as f32 / 2.0) as usize,
                );
                println!("{i}, {j}, {}", uni.height);
                if j < uni.width && i < uni.height && (i != last_modified.0 || j != last_modified.1)
                {
                    uni.edit_cell((i, j));
                    last_modified.0 = i;
                    last_modified.1 = j;
                }
            }
        }
    }
}

fn iteration_duration_slider_changed(
    mut timer: ResMut<IterationTimer>,
    query: Query<&IterationDuration, Changed<IterationDuration>>,
) {
    if let Ok(duration) = query.get_single() {
        timer.0.set_duration(Duration::from_secs_f32(duration.0))
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(UiState::Pause);
    commands.insert_resource(IterationTimer(Timer::from_seconds(
        0.15,
        TimerMode::Repeating,
    )));
    let duration = IterationDuration(0.15);

    commands.insert_resource(LastChangedTile(usize::MAX, usize::MAX));
    commands.spawn(duration);
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup,));
        app.add_systems(
            Update,
            (
                build_ui,
                build_grid,
                ui_controls,
                iteration_duration_slider_changed,
            ),
        );
    }
}
