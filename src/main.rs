use bevy::{
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .add_startup_system(setup_system)
        .add_system(ui_example_system)
        .add_system(update_camera_transform_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Left Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Right Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Top Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Bottom Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane {
    //         size: 5.0,
    //         subdivisions: 0,
    //     })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..Default::default()
    // });

    // Test mutating mesh

    let mut test_mesh = Mesh::from(shape::Plane {
        size: 5.0,
        subdivisions: 0,
    });

    let subdivisions = 10;
    let z_vertex_count = subdivisions + 2;
    let x_vertex_count = subdivisions + 2;
    let num_vertices = (z_vertex_count * x_vertex_count) as usize;
    let num_indices = ((z_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;
    let up = Vec3::Y.to_array();
    // We resize mesh to 8 (instead of 5 like above)
    let mesh_size = 8.0;

    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

    for y in 0..z_vertex_count {
        for x in 0..x_vertex_count {
            // Get coordinates for a plane. if it's 10x9 - it scales to 1x1
            let tx = x as f32 / (x_vertex_count - 1) as f32;
            let ty = y as f32 / (z_vertex_count - 1) as f32;
            // Scale the mesh up using the size
            // let x = (-0.5 + tx) * mesh_size + x_offset;
            // let y = (-0.5 + ty) * mesh_size + y_offset;
            let x = (-0.5 + tx) * mesh_size;
            let y = (-0.5 + ty) * mesh_size;
            let x_offset = x.sin();
            let y_offset = y.cos();
            // let x = ((-0.5 + tx) * mesh_size).sin();
            // let y = ((-0.5 + ty) * mesh_size).sin();
            positions.push([x, x_offset, y]);
            normals.push(up);
            uvs.push([tx, 1.0 - ty]);
        }
    }

    for y in 0..z_vertex_count - 1 {
        for x in 0..x_vertex_count - 1 {
            let quad = y * x_vertex_count + x;
            indices.push(quad + x_vertex_count + 1);
            indices.push(quad + 1);
            indices.push(quad + x_vertex_count);
            indices.push(quad);
            indices.push(quad + x_vertex_count);
            indices.push(quad + 1);
        }
    }
    // let mut test_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    test_mesh.set_indices(Some(Indices::U32(indices)));
    test_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    test_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    test_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands.spawn(PbrBundle {
        mesh: meshes.add(test_mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    // Demo code

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
    let camera_transform =
        Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
    commands.insert_resource(OriginalCameraTransform(camera_transform));

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..Default::default()
    });
}

fn update_camera_transform_system(
    occupied_screen_space: Res<OccupiedScreenSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&Projection, &mut Transform)>,
) {
    let (camera_projection, mut transform) = match camera_query.get_single_mut() {
        Ok((Projection::Perspective(projection), transform)) => (projection, transform),
        _ => unreachable!(),
    };

    let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
    let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
    let frustum_width = frustum_height * camera_projection.aspect_ratio;

    let window = windows.single();

    let left_taken = occupied_screen_space.left / window.width();
    let right_taken = occupied_screen_space.right / window.width();
    let top_taken = occupied_screen_space.top / window.height();
    let bottom_taken = occupied_screen_space.bottom / window.height();
    transform.translation = original_camera_transform.translation
        + transform.rotation.mul_vec3(Vec3::new(
            (right_taken - left_taken) * frustum_width * 0.5,
            (top_taken - bottom_taken) * frustum_height * 0.5,
            0.0,
        ));
}
