use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use std::time::Duration;

#[derive(Resource)]
pub struct IterationTimer(pub Timer);

#[derive(Component)]
struct IterationDuration(f32);

#[derive(Resource, Debug)]
pub enum UiState {
    Init,
    Play,
    Pause,
}

fn build_ui(
    mut contexts: EguiContexts,
    state: Option<ResMut<UiState>>,
    mut query: Query<&mut IterationDuration>,
) {
    egui::Window::new("Menu").show(contexts.ctx_mut(), |ui| {
        if ui.add(egui::Button::new("Play/Stop")).clicked() {
            if let Some(mut state) = state {
                invert_state(&mut state)
            };
        };
        ui.add(egui::Slider::new(&mut query.single_mut().0, 0.0..=1.0));
    });
}
fn invert_state(state: &mut UiState) {
    match *state {
        UiState::Play => *state = UiState::Pause,
        UiState::Init | UiState::Pause => *state = UiState::Play,
        _ => {}
    }
}
fn ui_keyboard_controls(state: Option<ResMut<UiState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        if let Some(mut state) = state {
            invert_state(&mut state)
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
    commands.insert_resource(UiState::Init);
    commands.insert_resource(IterationTimer(Timer::from_seconds(
        0.15,
        TimerMode::Repeating,
    )));
    let duration = IterationDuration(0.15);
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
                ui_keyboard_controls,
                iteration_duration_slider_changed,
            ),
        );
    }
}
