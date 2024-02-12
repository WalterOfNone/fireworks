use bevy::{prelude::*, render::primitives::Sphere};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::f32::consts::PI;

#[derive(Resource, Default)]
struct FireWorkOptions {
    titanium: bool,
    manganese: bool,
    iron: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum AppState {
    #[default]
    Builder,
    Launch,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 5.0,
        })
        .add_plugins(EguiPlugin)
        .init_resource::<FireWorkOptions>()
        .add_state::<AppState>()
        .add_systems(Update, main_menu.run_if(in_state(AppState::Builder)))
        .add_systems(OnEnter(AppState::Launch), firework_launch)
        .run();
}

fn main_menu(
    mut contexts: EguiContexts, 
    mut fw_options: ResMut<FireWorkOptions>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let mut ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("Header").show(&ctx, |ui| {
        ui.add(egui::Label::new("Firework maker!"))
    });
    egui::SidePanel::left("Options").show(&ctx, |ui| {
        ui.add(egui::Label::new("Metal Options:"));
        ui.add(egui::Checkbox::new(&mut fw_options.titanium, "Titanium"));
        ui.add(egui::Checkbox::new(&mut fw_options.manganese, "Manganese"));
        ui.add(egui::Checkbox::new(&mut fw_options.iron, "Iron"));
    });
    egui::TopBottomPanel::bottom("Launch").show(&ctx, |ui| {
        if ui.button("Launch Firework!").clicked() {
            next_state.set(AppState::Launch)
        }
    });
    egui::CentralPanel::default().show(&ctx, |ui| {
        
    });
}

fn firework_launch(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animations: ResMut<Assets<AnimationClip>>,
    asset_server: Res<AssetServer>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let firework = Name::new("firework");

    let mut fw_anim = AnimationClip::default();

    fw_anim.add_curve_to_path(EntityPath {parts: vec![firework.clone()]}, VariableCurve {
        keyframe_timestamps: vec![0.0, 1.0, 2.0, 3.0, 4.0],
        keyframes: Keyframes::Translation(vec![
            Vec3::new(0.0, -2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ]),
    });
    
    let mut fw_player = AnimationPlayer::default();

    fw_player.play(animations.add(fw_anim)).repeat();

    commands
    .spawn((
        SceneBundle {
            scene: asset_server.load("untitled.glb#Scene0"),
            transform: Transform::from_translation(Vec3 { x: 0.0, y: -2.0, z: 0.0 }),
            ..default()
        },
        // Add the Name component, and the animation player
        firework,
        fw_player,
    ));
}