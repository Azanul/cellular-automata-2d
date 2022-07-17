mod resources;

use bevy::{
    math::Vec2,
    prelude::{
        App, ClearColor, Color, Commands, DefaultPlugins, OrthographicCameraBundle, Transform,
        WindowDescriptor,
    },
    sprite::{Sprite, SpriteBundle},
};
use rand::Rng;
use resources::BoardMaterials;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Immigration game".to_string(),
            width: 1200.,
            height: 800.,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_map(mut commands: Commands) {
    commands.insert_resource(BoardMaterials::new());
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::Rgba {
                    red: 0.1,
                    green: 0.2,
                    blue: 0.4,
                    alpha: 0.8,
                },
                custom_size: Some(Vec2::new(23.0, 27.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, rng.gen_range(-400.0..400.0), 0.0),
            ..Default::default()
        });
    }
}
