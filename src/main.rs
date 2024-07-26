#![allow(unused)]

const W: f32 = 1080.;
const H: f32 = 608.;
use bevy::{
    prelude::*,
    reflect::Typed,
    sprite::{MaterialMesh2dBundle, NoWireframe2d},
    window::Cursor,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some("Void".to_string()),
                title: "Void".to_string(),
                cursor: Cursor {
                    icon: CursorIcon::VerticalText,
                    ..Default::default()
                },
                resolution: (W, H).into(),
                transparent: true,
                resizable: false,
                decorations: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Triangle2d::new(
                    Vec2::new(80.0, 0.0),
                    Vec2::new(30.0, 30.0),
                    Vec2::new(30.0, -30.0),
                ))
                .into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-540.0, 0.0, 0.0),
            ..default()
        },
        NoWireframe2d,
    ));
}
