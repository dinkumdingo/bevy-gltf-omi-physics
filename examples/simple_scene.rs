use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_gltf_omi_physics::GltfOmiPhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin)
        .add_plugins(GltfOmiPhysicsPlugin)
        .insert_resource(Time::<Physics>::default())
        .add_systems(Startup, load_simple_scene)
        .run();
}

const GLTF_PATH: &str = "simple_scene.glb";

fn load_simple_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut time: ResMut<Time<Physics>>,
) {
    // pause physics for a bit so the simulation doesn't lag due to assets still loading
    time.pause();
    let unpause_system = |mut time: ResMut<Time<Physics>>| time.unpause();
    let id = commands.register_system(unpause_system);
    commands.delayed().secs(5.0).run_system(id);

    commands.spawn(WorldAssetRoot(
        asset_server.load(GltfAssetLabel::Scene(0).from_asset(GLTF_PATH)),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
