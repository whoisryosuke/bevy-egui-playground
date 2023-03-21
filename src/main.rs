use bevy::{
    prelude::*,
    render::{camera::Projection, mesh::Indices},
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
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
}

impl Default for AnimationState {
    fn default() -> Self {
        AnimationState {
            speed: 1.0,
            paused: false,
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
                ui.horizontal(|ui| {
                    ui.label("Elapsed Time:");
                    ui.strong(
                        animation_players
                            .get_single()
                            .unwrap()
                            .elapsed()
                            .to_string(),
                    );
                });
            });

            // Background (with hover)
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
    player.play(animations.add(animation)).repeat();

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
    animation_state: Res<AnimationState>,
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
    }
}
