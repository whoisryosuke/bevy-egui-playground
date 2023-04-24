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

struct ClickwheelImageProps {
    position: egui::Pos2,
    size: egui::Vec2,
}

const CLICKWHEEL_DATA: [ClickwheelImageProps; 8] = [
    // Segment #1
    ClickwheelImageProps {
        position: egui::Pos2 { x: 113.36, y: 0.05 },
        size: egui::Vec2::new(254.5, 362.73),
    },
    // Segment #2
    ClickwheelImageProps {
        position: egui::Pos2 { x: 387.21, y: 0.0 },
        size: egui::Vec2::new(255.45, 362.78),
    },
    // Segment #3
    ClickwheelImageProps {
        position: egui::Pos2 {
            x: 400.91,
            y: 121.19,
        },
        size: egui::Vec2::new(356.1, 255.28),
    },
    // Segment #4
    ClickwheelImageProps {
        position: egui::Pos2 {
            x: 400.91,
            y: 395.83,
        },
        size: egui::Vec2::new(355.71, 247.72),
    },
    // Segment #5
    ClickwheelImageProps {
        position: egui::Pos2 {
            x: 387.21,
            y: 409.52,
        },
        size: egui::Vec2::new(247.5, 347.29),
    },
    // Segment #6
    ClickwheelImageProps {
        position: egui::Pos2 {
            x: 121.29,
            y: 409.52,
        },
        size: egui::Vec2::new(246.57, 347.25),
    },
    // Segment #7
    ClickwheelImageProps {
        position: egui::Pos2 { x: 0.39, y: 395.83 },
        size: egui::Vec2::new(353.77, 246.74),
    },
    // Segment #8
    ClickwheelImageProps {
        position: egui::Pos2 { x: 0.0, y: 122.2 },
        size: egui::Vec2::new(354.16, 254.27),
    },
];

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
    let svg_order = vec![
        &svgs.clickwheel_segment_1,
        &svgs.clickwheel_segment_2,
        &svgs.clickwheel_segment_3,
        &svgs.clickwheel_segment_4,
        &svgs.clickwheel_segment_5,
        &svgs.clickwheel_segment_6,
        &svgs.clickwheel_segment_7,
        &svgs.clickwheel_segment_8,
    ];

    let ctx = contexts.ctx_mut();

    // Make the window transparent
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

    // Create a new full screen window
    egui::Window::new("Hello")
        .fixed_size(ctx.available_rect().size())
        .title_bar(false)
        .collapsible(false)
        .movable(false)
        .resizable(false)
        .show(ctx, |ui| {
            // Render UI
            // Loop over all the clickwheel SVGs and render them
            for (index, svg) in svg_order.iter().enumerate() {
                // Absolutely position the SVG
                ui.put(
                    // Absolute position
                    egui::Rect {
                        // Coordinates of "top left"
                        min: CLICKWHEEL_DATA[index].position,
                        // Coordinates of "bottom right"
                        max: egui::Pos2 {
                            x: CLICKWHEEL_DATA[index].position.x + CLICKWHEEL_DATA[index].size.x,
                            y: CLICKWHEEL_DATA[index].position.y + CLICKWHEEL_DATA[index].size.y,
                        },
                    },
                    egui::Image::new(svg.texture_id(ctx), CLICKWHEEL_DATA[index].size),
                );
            }
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
