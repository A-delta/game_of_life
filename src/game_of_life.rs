use crate::game_of_life_logic;
use crate::ui::{IterationTimer, UiState};
use bevy::prelude::*;
use game_of_life_logic::{Cell, Universe};

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_universe)
            .add_systems(Update, (iterate_universe, update_sprites_universe).chain());
    }
}

#[derive(Component)]
struct CellSpriteId {
    i: usize,
    j: usize,
}

fn create_universe(mut commands: Commands) {
    let cell_size = 25.0;
    let mut universe = Universe::new(500, 500);
    let coords = [
        // glider mess
        (1, 1),
        (2, 2),
        (2, 3),
        (1, 3),
        (0, 3),
        (4, 10),
        (4, 11),
        (3, 12),
        (5, 11),
        (5, 12),
    ];
    for (i, j) in coords.iter() {
        universe.edit_cell((*i + 250, *j + 250));
    }
    for (index, cell) in universe.cells.iter().enumerate() {
        let (i, j) = universe.coordinates_from_linear(index);
        let mut color = Color::BLACK;
        match *cell {
            Cell::Alive => color = Color::WHITE,
            Cell::Dead => {}
        }
        let new_sprite = SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(cell_size, cell_size)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                j as f32 * cell_size - cell_size * universe.width as f32 / 2.0,
                i as f32 * cell_size - cell_size * universe.height as f32 / 2.0,
                0.,
            )),
            ..default()
        };
        commands.spawn((new_sprite, CellSpriteId { i, j }));
    }

    commands.insert_resource(universe);
}

fn iterate_universe(
    time: Res<Time>,
    mut timer: ResMut<IterationTimer>,
    ui_state: Option<ResMut<UiState>>,
    mut universe: ResMut<Universe>,
) {
    if let Some(ui_state) = ui_state {
        if let UiState::Play = ui_state.into_inner() {
            if timer.0.tick(time.delta()).just_finished() {
                *universe = universe.iterate();
            }
        }
    }
}

fn update_sprites_universe(
    universe: ResMut<Universe>,
    mut query_sprites: Query<(&CellSpriteId, &mut Sprite)>,
) {
    for (cell, mut sprite) in query_sprites.iter_mut() {
        match universe.get_cell((cell.i, cell.j)) {
            Cell::Alive => sprite.color = Color::WHITE,
            Cell::Dead => sprite.color = Color::BLACK,
        }
    }
}
