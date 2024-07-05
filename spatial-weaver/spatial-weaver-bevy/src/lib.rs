pub mod config;
pub mod input_field;

use bevy::{app::Plugin, ecs::component::Component};
use bevy_rapier3d::{
    geometry::Group,
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use config::SpatialWeaverConfig;

pub mod ui;

#[derive(Component)]
pub struct Interactor3D {}

#[derive(Component)]
pub struct Interactor2D {}

const DEFAULT_COL_GROUP: Group = Group::GROUP_9;

pub struct SpatialWeaverPlugin {
    add_rapier: bool,
    collision_group: Group,
}

impl Plugin for SpatialWeaverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if self.add_rapier {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugins(RapierDebugRenderPlugin::default());
        }

        app.insert_resource(SpatialWeaverConfig(self.collision_group));
    }
}

impl Default for SpatialWeaverPlugin {
    fn default() -> Self {
        Self {
            add_rapier: true,
            collision_group: DEFAULT_COL_GROUP,
        }
    }
}

impl SpatialWeaverPlugin {
    pub fn no_rapier_autoconfig() -> Self {
        Self {
            add_rapier: false,
            collision_group: DEFAULT_COL_GROUP,
        }
    }

    pub fn new(add_rapier: bool, collision_group: Group) -> Self {
        Self {
            add_rapier,
            collision_group,
        }
    }
}
