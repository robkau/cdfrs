#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::VertexAttributeValues::Float32x2;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Mesh2dHandle;
use bevy::window::WindowMode::BorderlessFullscreen;
use bevy::{
    prelude::*,
    reflect::TypeUuid,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
use std::process;

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "cfdrs".to_string(),
            mode: BorderlessFullscreen,
            present_mode: PresentMode::Immediate,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<ComplexDivisorFractalSingleLoop>::default())
        .add_plugin(Material2dPlugin::<ComplexDivisorFractalDoubleLoop>::default())
        .add_plugin(Material2dPlugin::<Mandelbrot>::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_quad)
        .insert_resource(WorldInspectorParams {
            enabled: false,
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(spawn_camera)
        .add_system(toggle_inspector)
        .add_system(zoom_in)
        .add_system(update_material_iterations)
        .add_system(handle_exit)
        .run();
}

fn spawn_quad(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<Mandelbrot>>,
) {
    let mut m = Mesh::from(shape::Quad::default());
    let uvs1 = vec![[-10.0, 10.0], [-10.0, -10.0], [10.0, -10.0], [10.0, 10.0]];
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs1);

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(m).into(),
        material: my_material_assets.add(Mandelbrot {}),
        ..default()
    });
}

fn update_material_iterations(
    input: ResMut<Input<KeyCode>>,
    mut material_handle: Query<&mut Handle<ComplexDivisorFractalDoubleLoop>>,
    mut my_material_assets: ResMut<Assets<ComplexDivisorFractalDoubleLoop>>,
) {
    if input.just_pressed(KeyCode::A) {
        for mh in material_handle.iter_mut() {
            let m = my_material_assets.get_mut(&mh).unwrap();
            m.iterations -= 1;
        }
    }
    if input.just_pressed(KeyCode::D) {
        for mh in material_handle.iter_mut() {
            let m = my_material_assets.get_mut(&mh).unwrap();
            m.iterations += 1;
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct ComplexDivisorFractalSingleLoop {
    #[uniform(0)]
    iterations: i32,
}

impl Material2d for ComplexDivisorFractalSingleLoop {
    fn fragment_shader() -> ShaderRef {
        "cdf_single.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "196e05fa-1271-4dd7-a6b8-686c4b50bf02"]
struct ComplexDivisorFractalDoubleLoop {
    #[uniform(0)]
    iterations: i32,
}

impl Material2d for ComplexDivisorFractalDoubleLoop {
    fn fragment_shader() -> ShaderRef {
        "cdf_double.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "dcb63d2f-2261-4659-b340-9eead095160e"]
struct Mandelbrot {}

impl Material2d for Mandelbrot {
    fn fragment_shader() -> ShaderRef {
        "mandelbrot.wgsl".into()
    }
}

fn spawn_camera(mut commands: Commands, wnds: Res<Windows>) {
    let mut camera = Camera2dBundle::default();

    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    let scale = f32::min(size.x, size.y) / 1.0;

    camera.projection.scale = 1.0 / scale;

    commands.spawn_bundle(camera);
}

fn handle_exit(input: ResMut<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Q) {
        process::exit(0);
    }
}

fn toggle_inspector(
    input: ResMut<Input<KeyCode>>,
    mut window_params: ResMut<WorldInspectorParams>,
) {
    if input.just_pressed(KeyCode::Grave) {
        window_params.enabled = !window_params.enabled
    }
}

struct ZoomState {
    offset: Vec2,
    width: f32,
}

impl Default for ZoomState {
    fn default() -> Self {
        ZoomState {
            offset: Default::default(),
            width: 10.0,
        }
    }
}

fn zoom_in(
    input: ResMut<Input<KeyCode>>,
    windows: Res<Windows>,
    mut mesh: Query<&mut Mesh2dHandle>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut zs: Local<ZoomState>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let cursor_offset_from_center = (size / 2.0 - cursor_pos) / size;
        let width = zs.width;
        zs.offset += cursor_offset_from_center * width / 18.;

        if input.pressed(KeyCode::W) {
            zs.width *= 0.99;
        }
        if input.pressed(KeyCode::S) {
            zs.width *= 1.01;
        }

        // zoom in
        let mesh_asset = mesh_assets.get_mut(&mesh.single_mut().clone().0).unwrap();
        let uvs = mesh_asset.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
        match uvs {
            Float32x2(uvss) => {
                uvss[0] = [-zs.width - zs.offset.x, zs.width + zs.offset.y];
                uvss[1] = [-zs.width - zs.offset.x, -zs.width + zs.offset.y];
                uvss[2] = [zs.width - zs.offset.x, -zs.width + zs.offset.y];
                uvss[3] = [zs.width - zs.offset.x, zs.width + zs.offset.y];
            }
            _ => {
                unreachable!()
            }
        }

        let mesh_asset = mesh_assets.get_mut(&mesh.single_mut().clone().0).unwrap();

        let uvs: Vec<[f32; 2]> = vec![
            [-zs.width - zs.offset.x, zs.width + zs.offset.y],
            [-zs.width - zs.offset.x, -zs.width + zs.offset.y],
            [zs.width - zs.offset.x, -zs.width + zs.offset.y],
            [zs.width - zs.offset.x, zs.width + zs.offset.y],
        ];
        let uvss = VertexAttributeValues::Float32x2(uvs);

        mesh_asset.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvss);
    }
}

// classifying image outputs:
// symmetries (xy/etc)
// cohesion
// detail
// color balance
