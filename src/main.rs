use bevy::{
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    window::PrimaryWindow,
};
use bevy_egui::{
    egui::{self, epaint, Color32},
    EguiContexts, EguiPlugin,
};
use egui_extras::RetainedImage;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

#[derive(Resource)]
struct UISVGs {
    clickwheel_segment_1: RetainedImage,
    clickwheel_segment_2: RetainedImage,
    clickwheel_segment_3: RetainedImage,
    clickwheel_segment_4: RetainedImage,
    clickwheel_segment_5: RetainedImage,
    clickwheel_segment_6: RetainedImage,
    clickwheel_segment_7: RetainedImage,
    clickwheel_segment_8: RetainedImage,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(UISVGs {
            clickwheel_segment_1: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_01.svg",
                include_bytes!("assets/clickwheel_segment_01.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_2: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_02.svg",
                include_bytes!("assets/clickwheel_segment_02.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_3: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_03.svg",
                include_bytes!("assets/clickwheel_segment_03.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_4: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_04.svg",
                include_bytes!("assets/clickwheel_segment_04.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_5: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_05.svg",
                include_bytes!("assets/clickwheel_segment_05.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_6: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_06.svg",
                include_bytes!("assets/clickwheel_segment_06.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_7: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_07.svg",
                include_bytes!("assets/clickwheel_segment_07.svg"),
                egui_extras::image::FitTo::Original,
            )
            .unwrap(),
            clickwheel_segment_8: RetainedImage::from_svg_bytes_with_size(
                "assets/clickwheel_segment_08.svg",
                include_bytes!("assets/clickwheel_segment_08.svg"),
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
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        window_fill: Color32::TRANSPARENT,
        panel_fill: Color32::TRANSPARENT,
        window_stroke: egui::Stroke {
            color: Color32::TRANSPARENT,
            width: 0.0,
        },
        window_shadow: epaint::Shadow {
            color: Color32::TRANSPARENT,
            ..old.window_shadow
        },
        ..old
    });

    egui::Window::new("Hello")
        .fixed_size(ctx.available_rect().size())
        .title_bar(false)
        .show(ctx, |ui| {
            let max_size = ui.available_size();
            let size_1 = egui::Vec2::new(254.5, 362.73);
            let size_2 = egui::Vec2::new(255.45, 362.78);
            let size_3 = egui::Vec2::new(356.1, 255.28);
            // svgs.clickwheel_segment_1.show_size(ui, size_1);
            // svgs.clickwheel_segment_2.show_size(ui, size_2);
            // svgs.clickwheel_segment_3.show_size(ui, size_3);

            ui.put(
                egui::Rect {
                    // Coordinates of "top left"
                    min: egui::Pos2 { x: 30.0, y: 30.0 },
                    // Coordinates of "bottom right"
                    max: egui::Pos2 {
                        x: size_1.x,
                        y: size_1.y,
                    },
                },
                egui::Image::new(svgs.clickwheel_segment_1.texture_id(ctx), size_1),
            );
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
