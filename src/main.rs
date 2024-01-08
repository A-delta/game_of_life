mod game_of_life;
use bevy::prelude::*;
use game_of_life::game::*;

#[derive(Component)]
struct MainCamera;

#[derive(Resource)]
struct IterationTimer(Timer);
fn create_universe(mut commands: Commands) {
    let mut universe = Universe::new(40, 40);

    universe.edit_cell((20, 20), Cell::Alive);
    universe.edit_cell((21, 21), Cell::Alive);
    universe.edit_cell((22, 21), Cell::Alive);
    universe.edit_cell((22, 20), Cell::Alive);
    universe.edit_cell((22, 22), Cell::Alive);
    commands.spawn(universe);
}
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
    let translation_speed = 50.0;
    if keys.pressed(KeyCode::Z) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, 1.0, 0.0) * translation_speed * timer.delta_seconds();
    }
    if keys.pressed(KeyCode::Q) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(-1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds();
        // Space was pressed
    }
    if keys.pressed(KeyCode::S) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(0.0, -1.0, 0.0) * translation_speed * timer.delta_seconds();
        // Space was pressed
    }
    if keys.pressed(KeyCode::D) {
        let mut transform = query_transform_camera.single_mut();
        transform.translation +=
            Vec3::new(1.0, 0.0, 0.0) * translation_speed * timer.delta_seconds();
        // Space was pressed
    }

    if keys.just_pressed(KeyCode::Plus) {
        let mut projection = query_camera.single_mut();
        projection.scale /= 1.25;
    }

    if keys.just_pressed(KeyCode::Minus) {
        let mut projection = query_camera.single_mut();
        projection.scale *= 1.25;
    }
}

fn display_universe(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<IterationTimer>,
    mut query: Query<&Universe>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let cell_size = 25.0;
    for uni in &query {
        for (index, cell) in uni.cells.iter().enumerate() {
            let (i, j) = uni.coordinates_from_linear(index);
            let mut color = Color::BLACK;
            match *cell {
                Cell::Alive => color = Color::WHITE,
                Cell::Dead => {}
            }
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(cell_size, cell_size)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    j as f32 * cell_size - cell_size * uni.width as f32 / 2.0,
                    i as f32 * cell_size - cell_size * uni.height as f32 / 2.0,
                    0.,
                )),
                ..default()
            });
        }
    }
}

fn main() {
    App::new()
        .insert_resource(IterationTimer(Timer::from_seconds(
            0.2,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (create_universe, spawn_camera))
        .add_systems(
            Update,
            ((iterate_universe, display_universe), keyboard_input),
        )
        .run();
}
