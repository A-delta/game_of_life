use bevy::prelude::*;

#[derive(Component)]
struct MainCamera;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_controls);
    }
}

fn spawn_camera(mut commands: Commands, mut window: Query<&Window>) {
    // let width = 500.0;
    // let cell_size = 25.0;
    // let window = &window.single_mut();
    let scale = 10.0;
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            projection: OrthographicProjection { scale, ..default() },
            ..default()
        },
        MainCamera,
    ));
}
fn camera_controls(
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
    }
}
