use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    sprite::MaterialMesh2dBundle,
    window::PrimaryWindow,
};
use bevy_egui::{
    egui::{self, epaint, Color32},
    EguiContexts, EguiPlugin,
};
use bevy_rapier2d::prelude::*;
use egui_extras::RetainedImage;

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

struct ClickwheelImageProps {
    position: egui::Pos2,
    size: egui::Vec2,
}

const CLICKWHEEL_WIDTH: f32 = 757.0;
const CLICKWHEEL_HEIGHT: f32 = 756.82;
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

struct ShowClickwheel;

#[derive(Component)]
struct ClickwheelMouseCollider;

#[derive(Component)]
struct ClickwheelObject;

#[derive(Resource)]
struct ClickwheelState {
    active: bool,
    hovered: usize,
    initial_position: Option<Vec2>,
    current_position: Option<Vec2>,
    screen_size: Option<egui::Vec2>,
}

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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EguiPlugin)
        .add_event::<ShowClickwheel>()
        .insert_resource(ClickwheelState {
            active: false,
            hovered: 0,
            initial_position: None,
            current_position: None,
            screen_size: None,
        })
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
        .add_system(check_input)
        .add_system(spawn_clickwheel_colliders)
        .add_system(despawn_clickwheel_colliders)
        .add_system(sync_mouse_collider)
        .add_system(handle_collision_events)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    svgs: Res<UISVGs>,
    mut clickwheel_state: ResMut<ClickwheelState>,
) {
    // Not active? Don't render UI
    if !clickwheel_state.active {
        return;
    }

    // Create a collection to quickly loop over all SVGs
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
            // Sync with screen size
            clickwheel_state.screen_size = Some(ui.available_size());

            return;

            // Render UI

            // Determine the size of clickwheel
            // We want it to be 80% of the viewport size
            let max_screen_height = ui.available_height();
            let max_clickwheel_height = max_screen_height * 0.8;
            // Figure out what percent we need to scale the image to make it fit
            let clickwheel_scale = max_clickwheel_height / CLICKWHEEL_HEIGHT;

            // Find the center Y position
            let actual_clickwheel_height = CLICKWHEEL_HEIGHT * clickwheel_scale;
            let offset_y_center = (max_screen_height / 2.0) - (actual_clickwheel_height / 2.0);

            // Find the center X position
            let max_screen_width = ui.available_width();
            let actual_clickwheel_width = CLICKWHEEL_WIDTH * clickwheel_scale;
            let offset_x_center = (max_screen_width / 2.0) - (actual_clickwheel_width / 2.0);

            // Loop over all the clickwheel SVGs and render each one
            for (index, svg) in svg_order.iter().enumerate() {
                // Scale and offset accordingly
                // Coordinates of "bottom right"
                let max_x = offset_x_center
                    + (CLICKWHEEL_DATA[index].position.x + CLICKWHEEL_DATA[index].size.x)
                        * clickwheel_scale;
                let max_y = offset_y_center
                    + (CLICKWHEEL_DATA[index].position.y + CLICKWHEEL_DATA[index].size.y)
                        * clickwheel_scale;
                let max = egui::Pos2 { x: max_x, y: max_y };

                // Coordinates of "top left"
                let min = egui::Pos2 {
                    x: offset_x_center + CLICKWHEEL_DATA[index].position.x * clickwheel_scale,
                    y: offset_y_center + CLICKWHEEL_DATA[index].position.y * clickwheel_scale,
                };
                // Size of the SVG
                let hover_scale = if clickwheel_state.hovered == index {
                    1.1
                } else {
                    1.0
                };
                let size = egui::Vec2 {
                    x: CLICKWHEEL_DATA[index].size.x * clickwheel_scale * hover_scale,
                    y: CLICKWHEEL_DATA[index].size.y * clickwheel_scale * hover_scale,
                };

                // Absolutely position the SVG
                let svg = ui.put(
                    // Absolute position
                    egui::Rect {
                        // Coordinates of "top left"
                        min,
                        // Coordinates of "bottom right"
                        max,
                    },
                    egui::Image::new(svg.texture_id(ctx), size),
                );
                if svg.hovered() {
                    // println!("hovered over {}", index);
                    clickwheel_state.hovered = index;
                }
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

    commands.spawn((
        Camera3dBundle {
            transform: camera_transform,
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            ..Default::default()
        },
        // UI config is a separate component
        UiCameraConfig { show_ui: false },
    ));
}

fn check_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_events: EventReader<CursorMoved>,
    mut clickwheel_state: ResMut<ClickwheelState>,
    mut show_clickwheel_event: EventWriter<ShowClickwheel>,
) {
    if keyboard_input.just_pressed(KeyCode::V) {
        println!("[KEYBOARD] Pressed clickwheel btn");
        clickwheel_state.active = true;

        show_clickwheel_event.send(ShowClickwheel);
    }
    if keyboard_input.just_released(KeyCode::V) {
        println!("[KEYBOARD] Released clickwheel btn");
        clickwheel_state.active = false;
        clickwheel_state.initial_position = None;
        clickwheel_state.current_position = None;
    }

    if clickwheel_state.active {
        for cursor in cursor_events.iter() {
            // Do we have an initial position?
            if clickwheel_state.initial_position.is_none() {
                // println!(
                //     "New cursor position: X: {}, Y: {}",
                //     cursor.position.x, cursor.position.y
                // );
                clickwheel_state.initial_position = Some(cursor.position.clone());
            }

            // Store current position
            clickwheel_state.current_position = Some(cursor.position.clone());
        }
    }
}

fn spawn_clickwheel_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut show_clickwheel_events: EventReader<ShowClickwheel>,
) {
    if !show_clickwheel_events.is_empty() {
        for _ in show_clickwheel_events.iter() {
            println!("spawning clickwheel collider");

            // Spawn a 2D camera with a clear background for UI
            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        order: 4,
                        ..Default::default()
                    },
                    camera_2d: Camera2d {
                        clear_color: ClearColorConfig::None,
                    },

                    ..Default::default()
                },
                ClickwheelObject,
            ));

            // Circle
            commands.spawn((
                ClickwheelObject,
                ClickwheelMouseCollider,
                RigidBody::Dynamic,
                Collider::cuboid(50.0, 100.0),
                ActiveEvents::COLLISION_EVENTS,
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::PURPLE)),
                    transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                    ..default()
                },
            ));

            let center_point = Vec2::splat(0.0);
            let num_segments = 8;
            for index in 0..num_segments {
                // let position_x = (PI / 2.0 - (index as f32) * PI / 180.0).cos();
                // let position_y = (PI / 2.0 - (index as f32) * PI / 180.0).sin();

                let radius = 200.0;
                let percent_of_circle = (index as f32 / (num_segments as f32 * 2.0));
                let position_in_circle = 360.0 * percent_of_circle;
                let angle = position_in_circle * PI * 2.0 / 180.0;
                let position_x = radius * angle.cos();
                let position_y = radius * angle.sin();

                let mut transform =
                    Transform::from_translation(Vec3::new(position_x, position_y, 0.));
                // let mut transform =
                //     Transform::from_translation(Vec3::new(position_x, position_y, 0.));
                // transform
                //     .rotate_around(Vec3::splat(0.0), Quat::from_rotation_x(45.0 * index as f32));
                // Rectangle
                commands.spawn((
                    ClickwheelObject,
                    Collider::cuboid(50.0, 100.0),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.25, 0.25, 0.75),
                            custom_size: Some(Vec2::new(50.0, 100.0)),
                            ..default()
                        },
                        transform,
                        ..default()
                    },
                ));
            }
        }
    }
}

fn despawn_clickwheel_colliders(
    mut commands: Commands,
    clickwheel_state: Res<ClickwheelState>,
    colliders: Query<Entity, With<ClickwheelObject>>,
) {
    let collider_collection = colliders.iter();
    if !clickwheel_state.active && collider_collection.len() > 0 {
        println!("despawning clickwheel collider");
        for collider in collider_collection {
            commands.entity(collider).despawn_recursive();
        }
    }
}

fn sync_mouse_collider(
    clickwheel_state: Res<ClickwheelState>,
    mut mouse_colliders: Query<&mut Transform, With<ClickwheelMouseCollider>>,
) {
    if clickwheel_state.active {
        for mut mouse_collider in mouse_colliders.iter_mut() {
            if let Some(current_position) = clickwheel_state.current_position {
                if let Some(screen_offset) = clickwheel_state.screen_size {
                    mouse_collider.translation.x = current_position.x - (screen_offset.x / 2.0);
                    mouse_collider.translation.y = current_position.y - (screen_offset.y / 2.0);
                }
            }
        }
    }
}

fn handle_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    // Check for collisions
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(first_entity, second_entity, _) => {
                println!(
                    "{} collided with {}",
                    first_entity.index(),
                    second_entity.index()
                );
            }
            CollisionEvent::Stopped(first_entity, second_entity, event) => {}
        }
    }
}
