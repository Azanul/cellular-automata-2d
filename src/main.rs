use std::collections::HashMap;
use std::fmt::Debug;

use bevy::{
    math::{IVec2, Vec2},
    prelude::{
        App, BuildChildren, Commands, Component, DefaultPlugins, GlobalTransform,
        OrthographicCameraBundle, Reflect, Transform, WindowDescriptor,
    },
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
};
use bevy_life::{CellState, CellularAutomatonPlugin, MooreCell2d, SimulationBatch};
use rand::Rng;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Immigration game".to_string(),
            width: 500.,
            height: 500.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ImmigrationGame2dPlugin::default())
        .insert_resource(SimulationBatch::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_map)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_map(mut commands: Commands) {
    spawn_map(&mut commands);
}

fn spawn_map(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let (size_x, size_y) = (128, 128);
    let sprite_size = 4.;
    let color = Color::rgba(0., 0., 0., 1.);

    commands
        .spawn()
        .insert(Transform::from_xyz(
            -(size_x as f32 * sprite_size) / 2.,
            -(size_y as f32 * sprite_size) / 2.,
            0.,
        ))
        .insert(GlobalTransform::default())
        .with_children(|builder| {
            for y in 0..=size_y {
                for x in 0..=size_x {
                    let state = if rng.gen_bool(1. / 4.) {
                        ImmigrationCellState::Alive(rng.gen_range(1..=3))
                    } else {
                        ImmigrationCellState::Dead
                    };
                    builder
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color,
                                custom_size: Some(Vec2::splat(sprite_size)),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                sprite_size * x as f32,
                                sprite_size * y as f32,
                                0.,
                            ),
                            ..Default::default()
                        })
                        .insert(MooreCell2d::new(IVec2::new(x, y)))
                        .insert(state);
                }
            }
        });
}

pub type ImmigrationGame2dPlugin = CellularAutomatonPlugin<MooreCell2d, ImmigrationCellState>;

#[derive(Debug, Clone, PartialEq, Component, Reflect)]
pub enum ImmigrationCellState {
    /// A dead cell
    Dead,
    /// Alive cell with a integer sub-state
    Alive(i8),
}

impl CellState for ImmigrationCellState {
    fn new_cell_state(&self, neighbor_cells: &[Self]) -> Self {
        let alive_cells: Vec<i8> = neighbor_cells
            .iter()
            .filter_map(|c| match c {
                Self::Dead => None,
                Self::Alive(s) => Some(*s),
            })
            .collect();
        let alive_cells_count = alive_cells.len();
        if self.is_alive() {
            if (2..=3).contains(&alive_cells_count) {
                self.clone()
            } else {
                Self::Dead
            }
        } else if alive_cells_count == 3 {
            let mut map = HashMap::<i8, i32>::new();
            for alive_cell in alive_cells {
                *map.entry(alive_cell).or_insert(0) += 1;
            }
            Self::Alive(
                map.into_iter()
                    .max_by_key(|(_k, v)| *v)
                    .map(|(k, _v)| k)
                    .unwrap(),
            )
        } else {
            Self::Dead
        }
    }

    fn color(&self) -> Option<Color> {
        match self {
            Self::Dead => None,
            Self::Alive(b) => Some(if *b == 1 {
                Color::PINK
            } else if *b == 2 {
                Color::AQUAMARINE
            } else {
                Color::GOLD
            }),
        }
    }
}

impl ImmigrationCellState {
    /// Is the cell considered alive
    #[must_use]
    #[inline]
    pub const fn is_alive(&self) -> bool {
        matches!(self, Self::Alive(_))
    }
}

impl Default for ImmigrationCellState {
    fn default() -> Self {
        Self::Dead
    }
}
