// Demonstrates shapes respecting global bloom settings

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

mod gallery_3d;
use gallery_3d::gallery;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, draw_gallery)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform::from_xyz(0., 0., 16.).looking_at(Vec3::ZERO, Vec3::Y),
            msaa: Msaa::Off,
            ..default()
        },
        BloomSettings::default(),
    ));
}

fn draw_gallery(time: Res<Time>, painter: ShapePainter) {
    gallery(painter, time.elapsed_secs(), 0..15);
}
