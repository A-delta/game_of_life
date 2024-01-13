use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use std::time::Duration;

use crate::camera::MainCamera;
use crate::game_of_life::GameOfLifePlugin;
use crate::game_of_life_logic::{Cell, Universe};
use rand::Rng;
#[derive(Resource)]
pub struct IterationTimer(pub Timer);

#[derive(Component)]
struct IterationDuration(f32);

#[derive(Component)]
struct RandomizeProbability(f32);

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
    mut universe: ResMut<Universe>,
    mut query_duration: Query<&mut IterationDuration>,
    mut query_randomizer_factor: Query<&mut RandomizeProbability>,
) {
    egui::SidePanel::left("Menu")
        .resizable(true)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Game Of Life");
            ui.label("Camera controls : Z, Q, S, D");
            ui.label("Zoom : A, E");
            if let Some(mut state) = state {
                if ui.add(egui::Button::new("Play/Stop [Space]")).clicked() {
                    invert_state(&mut state)
                };
                if ui.add(egui::Button::new("Reset")).clicked() {
                    reset_universe(&mut universe);
                };
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                };
                if ui.add(egui::Button::new("Randomize")).clicked() {
                    reset_universe(&mut universe);
                    randomize_universe(&mut universe, &query_randomizer_factor);
                };
                ui.add(
                    egui::Slider::new(&mut query_duration.single_mut().0, 0.0..=1.0)
                        .text("Iteration time"),
                );

                ui.add(
                    egui::Slider::new(&mut query_randomizer_factor.single_mut().0, 0.0..=1.0)
                        .text("Randomizer factor"),
                );
            };
        });
}

fn reset_universe(universe: &mut Universe) {
    *universe = Universe::new(universe.height, universe.width);
}

fn randomize_universe(
    universe: &mut ResMut<Universe>,
    query_randomizer_factor: &Query<&mut RandomizeProbability>,
) {
    let mut rng = rand::thread_rng();
    for i in 1..universe.height {
        for j in 1..universe.width {
            if rng.gen::<f32>() < query_randomizer_factor.single().0 {
                universe.set_cell((i, j), Cell::Alive);
            };
        }
    }
}

fn build_grid(state: ResMut<UiState>, universe: ResMut<Universe>, mut gizmos: Gizmos) {
    let cell_size = 25.0;
    let uni = universe;
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
    universe: ResMut<Universe>,
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
            let mut uni = universe;

            let (camera, camera_transform) = camera_query.single();
            if let Some(position_world) = camera.viewport_to_world_2d(camera_transform, position) {
                let (i, j) = (
                    ((position_world.y) / cell_size + 0.5 + uni.height as f32 / 2.0) as usize,
                    ((position_world.x) / cell_size + 0.5 + uni.width as f32 / 2.0) as usize,
                );
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

    commands.insert_resource(LastChangedTile(usize::MAX, usize::MAX));
    commands.spawn(IterationDuration(0.15));
    commands.spawn(RandomizeProbability(0.3));
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GameOfLifePlugin)
            .add_systems(Startup, (setup,))
            .add_systems(
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
