use bevy::{
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_extras::RetainedImage;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource)]
struct UISVGs {
    clickwheel_segment: RetainedImage,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(UISVGs {
            clickwheel_segment: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment.svg",
                include_bytes!("assets/clickwheel_segment.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
        })
        .add_startup_system(setup_system)
        .add_system(ui_example_system)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts, svgs: Res<UISVGs>) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("world");

        let max_size = ui.available_size();
        svgs.clickwheel_segment.show_size(ui, max_size);
    });
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
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

    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..Default::default()
    });
}
