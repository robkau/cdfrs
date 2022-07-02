#![allow(clippy::redundant_field_names)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

use crate::KeyCode::Z;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::render::camera::Camera2d;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::VertexAttributeValues::Float32x2;
use bevy::render::render_resource::VertexFormat;
use bevy::sprite::{Mesh2dHandle, SpecializedMaterial2d};
use bevy::window::WindowMode::BorderlessFullscreen;
use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::ScalingMode,
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
    window::PresentMode,
};
use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};

pub const CLEAR: Color = Color::rgb(0.3, 0.3, 0.3);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "cfdrs".to_string(),
            mode: BorderlessFullscreen,
            //present_mode: PresentMode::Fifo,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Material2dPlugin::<MyMaterial>::default())
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
        .run();
}

fn spawn_quad(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut my_material_assets: ResMut<Assets<MyMaterial>>,
) {
    let mut m = Mesh::from(shape::Quad::default());
    let mut uvs1 = Vec::with_capacity(4);
    uvs1.push([-10.0, 10.0]);
    uvs1.push([-10.0, -10.0]);
    uvs1.push([10.0, -10.0]);
    uvs1.push([10.0, 10.0]);
    m.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs1);

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: mesh_assets.add(m).into(),
        material: my_material_assets.add(MyMaterial),
        ..default()
    });
}

#[derive(TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct MyMaterial;

struct MyMaterialGPU {
    bind_group: BindGroup,
}

impl Material2d for MyMaterial {
    fn bind_group(material: &MyMaterialGPU) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[],
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        asset_server.watch_for_changes().unwrap();
        Some(asset_server.load("my_material.wgsl"))
    }
}

impl RenderAsset for MyMaterial {
    type ExtractedAsset = MyMaterial;
    type PreparedAsset = MyMaterialGPU;
    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<MyMaterial>>);

    fn extract_asset(&self) -> MyMaterial {
        self.clone()
    }

    fn prepare_asset(
        _extracted_asset: MyMaterial,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<MyMaterialGPU, PrepareAssetError<MyMaterial>> {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &pipeline.material2d_layout,
            entries: &[],
        });
        Ok(MyMaterialGPU { bind_group })
    }
}

fn spawn_camera(mut commands: Commands, wnds: Res<Windows>) {
    let mut camera = OrthographicCameraBundle::new_2d();

    let wnd = wnds.get_primary().unwrap();
    let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    let scale = f32::min(size.x, size.y) / 1.0;

    //camera.transform = Transform::from_translation(Vec3::new(
    //    size.x / (scale * 2.0),
    //    size.y / (scale * 2.0),
    //    0.0,
    //));
    camera.orthographic_projection.scale = 1.0 / scale;
    //camera.orthographic_projection.right = 1.0;
    //camera.orthographic_projection.left = -1.0;
    //camera.orthographic_projection.top = 1.0;
    //camera.orthographic_projection.bottom = -1.0;
    //camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
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
    mut mesh: Query<(&mut Mesh2dHandle)>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut zs: Local<ZoomState>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let cursor_offset_from_center = (size / 2.0 - cursor_pos) / size;
        zs.offset = zs.offset + (cursor_offset_from_center * zs.width / 18.);

        if input.pressed(KeyCode::W) {
            zs.width *= 0.99;
        }
        if input.pressed(KeyCode::S) {
            zs.width *= 1.01;
        }

        // zoom in
        let mesh_asset = mesh_assets.get_mut(mesh.single_mut().clone().0).unwrap();
        let mut uvs = mesh_asset.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
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

        /*
        // todo lag.
        let mesh_asset = mesh_assets.get_mut(mesh.single_mut().clone().0).unwrap();

        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(4);
        uvs.push([-zs.width - zs.offset.x, zs.width + zs.offset.y]);
        uvs.push([-zs.width - zs.offset.x, -zs.width + zs.offset.y]);
        uvs.push([zs.width - zs.offset.x, -zs.width + zs.offset.y]);
        uvs.push([zs.width - zs.offset.x, zs.width + zs.offset.y]);
        let uvss = VertexAttributeValues::Float32x2(uvs);

        mesh_asset.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvss);
        */
    }
}

// classifying image outputs:
// symmetries (xy/etc)
// cohesion
// detail
// color balance
