use bevy::{app::App, DefaultPlugins};
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use spatial_weaver_bevy::SpatialWeaverPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(SpatialWeaverPlugin::no_rapier_autoconfig())
        .run();
}
