mod game_of_life;
use bevy::prelude::*;
use game_of_life::game::*;

#[derive(Component)]
struct CellSpriteId {
    i: usize,
    j: usize,
}

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct IterationTimer(Timer);
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
        universe.edit_cell((*i + 250, *j + 250), Cell::Alive);
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

    commands.spawn(universe);
}
fn spawn_camera(mut commands: Commands, mut window: Query<&Window>) {
    let width = 500.0;
    let cell_size = 25.0;
    let window = &window.single_mut();
    let scale = 10.0;
    println!("{scale}");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            projection: OrthographicProjection { scale, ..default() },
            ..default()
        },
        MainCamera,
    ));
}

fn iterate_universe(
    time: Res<Time>,
    mut timer: ResMut<IterationTimer>,
    mut query: Query<&mut Universe>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut uni in &mut query {
            *uni = uni.iterate();
            //println!("{}", *uni);
        }
    }
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    timer: ResMut<Time>,
    mut query_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut query_transform_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let mut projection = query_camera.single_mut();

    let translation_speed = 400.0 * projection.scale;
    let boost = if keys.pressed(KeyCode::ShiftLeft) {
        3.0
    } else {
        1.0
    };

    if keys.pressed(KeyCode::Z) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, 1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::Q) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(-1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::S) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, -1.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }
    if keys.pressed(KeyCode::D) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds() * boost;
    }

    if keys.pressed(KeyCode::E) {
        projection.scale /= 1.1;
    }

    if keys.pressed(KeyCode::A) {
        projection.scale *= 1.1;
        println!("New scaling factor : {}", projection.scale);
    }
}

fn update_sprites_universe(
    query_universe: Query<&mut Universe>,
    mut query_sprites: Query<(&CellSpriteId, &mut Sprite)>,
) {
    for (cell, mut sprite) in query_sprites.iter_mut() {
        for uni in &query_universe {
            match uni.get_cell((cell.i, cell.j)) {
                Cell::Alive => sprite.color = Color::WHITE,
                Cell::Dead => sprite.color = Color::BLACK,
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(IterationTimer(Timer::from_seconds(
            0.0,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_universe, spawn_camera).chain())
        .add_systems(Update, ((iterate_universe), keyboard_input))
        .add_systems(FixedUpdate, update_sprites_universe)
        .run();
}
