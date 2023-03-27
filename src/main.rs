use bevy::{
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    window::PrimaryWindow,
};
use bevy_egui::{
    egui::{
        self,
        emath::{Rect, RectTransform},
        Color32, Painter, Pos2, Rounding, Sense, Shape, Stroke,
    },
    EguiContexts, EguiPlugin,
};
use std::f32::consts::PI;

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

#[derive(Resource)]
struct AnimationState {
    speed: f32,
    paused: bool,
    elapsed_time: f32,
    elapsed_time_update: bool,
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            speed: 1.0,
            paused: false,
            elapsed_time: 0.0,
            elapsed_time_update: false,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<AnimationState>()
        .add_startup_system(setup_system)
        .add_system(ui_example_system)
        .add_system(update_camera_transform_system)
        .add_system(update_animation_speed)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    mut animation_state: ResMut<AnimationState>,
    animation_players: Query<&mut AnimationPlayer, With<Name>>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Animation Clips");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Properties");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            // ui.heading("Top Panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Animation Timeline");

            ui.vertical(|ui| {
                // Top UI
                ui.horizontal(|ui| {
                    // Play/Pause button
                    let play_btn_label = if animation_state.paused {
                        "Play".to_string()
                    } else {
                        "Pause".to_string()
                    };
                    if ui.button(play_btn_label).clicked() {
                        animation_state.paused = !animation_state.paused;
                    }

                    // Speed buttons
                    ui.horizontal(|ui| {
                        if ui.button("-").clicked() {
                            animation_state.speed -= 0.1;
                        }
                        ui.strong("Speed");
                        if ui.button("+").clicked() {
                            animation_state.speed += 0.1;
                        }
                    });

                    // Display elapsed time from top level component
                    // @TODO: System for multiple animation clips (prob nested like After Effects)
                    // Elapsed time input (click for exact or drag)
                    if ui
                        .add(egui::DragValue::new(&mut animation_state.elapsed_time).speed(0.1))
                        .changed()
                    {
                        // We use a dirty flag to update the state in another system to keep UI clean
                        animation_state.elapsed_time_update = true;
                    };
                });

                // Render the UI and store the response in a variable for use later
                // let plot_ui = example_plot(ui);

                // Check if the user has clicked on it
                // if plot_ui.clicked() {
                //     // Grab the mouse position
                //     if let Some(position) = plot_ui.interact_pointer_pos() {
                //         println!("Timeline clicked: X: {}, Y: {}", position.x, position.y);
                //     }
                // }

                // ui.painter().line_segment(
                //     [Pos2 { x: 0.0, y: 0.0 }, Pos2 { x: 420.0, y: 420.0 }],
                //     Stroke {
                //         width: 10.0,
                //         color: Color32::BLUE,
                //     },
                // );

                // Create a "canvas" for drawing on that's 100% x 300px
                let (response, painter) = ui.allocate_painter(
                    bevy_egui::egui::Vec2::new(ui.available_width(), 300.0),
                    Sense::hover(),
                );

                // Get the relative position of our "canvas"
                let to_screen = RectTransform::from_to(
                    Rect::from_min_size(Pos2::ZERO, response.rect.size()),
                    response.rect,
                );

                // The width of the screen
                let timeline_length = ui.available_width() as usize;
                // The "gap" between lines
                let timeline_line_gap = 20;
                // The number of lines to draw given the available width and gap
                let timeline_num_lines = timeline_length / timeline_line_gap;
                // We convert the gap to a float to do positional/vector math later
                let timeline_line_gap_float = timeline_line_gap as f32;

                // Loop over the number of lines we need
                for index in 0..timeline_num_lines {
                    // Use the range index to determine an initial X position
                    let unscaled_x = 1.0 * (index as f32);
                    // Then we scale it using the gap
                    let x = unscaled_x * timeline_line_gap_float;

                    // Create our 2 points for the line segment
                    let first_point = Pos2 { x, y: 0.0 };
                    let second_point = Pos2 { x, y: 300.0 };
                    // Draw a vertical line
                    draw_line(&to_screen, &painter, first_point, second_point);
                }

                // Draw squares representing animations
                painter.add(Shape::Rect(egui::epaint::RectShape {
                    rect: Rect {
                        min: to_screen.transform_pos(Pos2 { x: 0.0, y: 0.0 }),
                        max: to_screen.transform_pos(Pos2 { x: 250.0, y: 250.0 }),
                    },
                    rounding: Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 0.0,
                        se: 0.0,
                    },
                    fill: Color32::BLUE,
                    stroke: Stroke {
                        width: 2.0,
                        color: Color32::WHITE,
                    },
                }));

                // Has timeline been hovered?
                if response.hovered() {
                    // Get the hover position
                    if let Some(position) = response.hover_pos() {
                        // Let's store a relative coordinate to the canvas at the top left.
                        // We'll use it later to determine the Y axis.
                        // When we use the mouse position, it's already in "screen space"
                        let relative_point = to_screen.transform_pos(Pos2 { x: 0.0, y: 0.0 });

                        // The 2 points we need for the line segment
                        // We use our mouse cursor's X position directly, and the relative point for Y
                        let first_point = Pos2 {
                            x: position.x,
                            y: relative_point.y,
                        };
                        let second_point = Pos2 {
                            x: position.x,
                            y: relative_point.y + 300.0,
                        };

                        // Draw a vertical line
                        painter.add(Shape::LineSegment {
                            points: [first_point, second_point],
                            stroke: Stroke {
                                width: 2.0,
                                color: Color32::RED,
                            },
                        });
                        // In case you want to see what's happening
                        // dbg!(position);
                    };
                }
            });

            // Background (with hover)
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

fn draw_line(to_screen: &RectTransform, painter: &Painter, first_point: Pos2, second_point: Pos2) {
    // Make the points relative to the "canvas"
    let first_point_in_screen = to_screen.transform_pos(first_point);
    let second_point_in_screen = to_screen.transform_pos(second_point);

    // Paint the line!
    painter.add(Shape::LineSegment {
        points: [first_point_in_screen, second_point_in_screen],
        stroke: Stroke {
            width: 2.0,
            color: Color32::GRAY,
        },
    });
}

// The "Timeline" UI - a line graph using egui's Plot and Line APIs
fn example_plot(ui: &mut egui::Ui) -> egui::Response {
    use egui::plot::{Line, PlotPoints};
    let n = 128;
    let line = Line::new(
        (0..=n)
            .map(|i| {
                use std::f64::consts::TAU;
                let x = egui::remap(i as f64, 0.0..=n as f64, -TAU..=TAU);
                // match self.plot {
                //     Plot::Sin => [x, x.sin()],
                //     Plot::Bell => [x, 10.0 * gaussian(x)],
                //     Plot::Sigmoid => [x, sigmoid(x)],
                // }
                [x, x.sin()]
            })
            .collect::<PlotPoints>(),
    );
    egui::plot::Plot::new("example_plot")
        .show_axes([true, false])
        .sharp_grid_lines(true)
        .allow_drag(true)
        .allow_zoom(false)
        .allow_scroll(false)
        .center_x_axis(false)
        .center_y_axis(true)
        // .clamp_grid(true)
        // .auto_bounds_x()
        .width(400.0)
        .height(200.0)
        .data_aspect(1.0)
        .min_size(bevy_egui::egui::Vec2::new(0.0, 0.0))
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
) {
    // Animation

    // The animation API uses the `Name` component to target entities
    let planet = Name::new("planet");
    let orbit_controller = Name::new("orbit_controller");
    let satellite = Name::new("satellite");

    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![planet.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Translation(vec![
                Vec3::new(1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, 1.0),
                Vec3::new(-1.0, 0.0, -1.0),
                Vec3::new(1.0, 0.0, -1.0),
                // in case seamless looping is wanted, the last keyframe should
                // be the same as the first one
                Vec3::new(1.0, 0.0, 1.0),
            ]),
        },
    );
    // Or it can modify the rotation of the transform.
    // To find the entity to modify, the hierarchy will be traversed looking for
    // an entity with the right name at each level
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![planet.clone(), orbit_controller.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
        },
    );
    // If a curve in an animation is shorter than the other, it will not repeat
    // until all other curves are finished. In that case, another animation should
    // be created for each part that would have a different duration / period
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![planet.clone(), orbit_controller.clone(), satellite.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
                Vec3::splat(1.2),
                Vec3::splat(0.8),
            ]),
        },
    );
    // There can be more than one curve targeting the same entity path
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![planet.clone(), orbit_controller.clone(), satellite.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            keyframes: Keyframes::Rotation(vec![
                Quat::IDENTITY,
                Quat::from_axis_angle(Vec3::Y, PI / 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 2.),
                Quat::from_axis_angle(Vec3::Y, PI / 2. * 3.),
                Quat::IDENTITY,
            ]),
        },
    );

    // Create the animation player, and set it to repeat
    let mut player = AnimationPlayer::default();
    player.play(animations.add(animation));

    // Create the scene that will be animated
    // First entity is the planet
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::try_from(shape::Icosphere::default()).unwrap()),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                ..default()
            },
            // Add the Name component, and the animation player
            planet,
            player,
        ))
        .with_children(|p| {
            // This entity is just used for animation, but doesn't display anything
            p.spawn((
                SpatialBundle::INHERITED_IDENTITY,
                // Add the Name component
                orbit_controller,
            ))
            .with_children(|p| {
                // The satellite, placed at a distance of the planet
                p.spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(1.5, 0.0, 0.0),
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                        material: materials.add(Color::rgb(0.3, 0.9, 0.3).into()),
                        ..default()
                    },
                    // Add the Name component
                    satellite,
                ));
            });
        });

    // Demo code
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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

fn update_animation_speed(
    mut animation_state: ResMut<AnimationState>,
    mut players: Query<&mut AnimationPlayer, With<Name>>,
) {
    // Loop through any animation players
    for mut player in players.iter_mut() {
        // If speed is different, update it
        if player.speed() != animation_state.speed {
            player.set_speed(animation_state.speed);
        }

        // Pause animation if changed
        if player.is_paused() != animation_state.paused {
            match animation_state.paused {
                true => player.pause(),
                false => player.resume(),
            };
        }

        // Update elapsed time
        if animation_state.elapsed_time_update {
            player.set_elapsed(animation_state.elapsed_time);
            animation_state.elapsed_time_update = false;
        } else {
            // Sync the animation's elapsed time with the slider
            animation_state.elapsed_time = player.elapsed();
        }
    }
}
