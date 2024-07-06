pub mod context;
pub mod input_field;

use bevy::app::Plugin;
use bevy_rapier3d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    rapier::geometry::Group,
    render::RapierDebugRenderPlugin,
};
use context::WeaverContext;

pub mod ui;

const DEFAULT_COL_GROUP: Group = Group::GROUP_9; // Group that the colliders are a part of
const DEFAULT_FILTER_GROUP: Group = Group::GROUP_10; // Group that the colliders filter for/expect collision from

pub struct SpatialWeaverPlugin {
    add_rapier: bool,
    collider_group: Group,
    filter_group: Group,
}

impl Plugin for SpatialWeaverPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        if self.add_rapier {
            app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
                .add_plugins(RapierDebugRenderPlugin::default());
        }

        app.insert_resource(WeaverContext {
            collider_group: self.collider_group,
            filter_group: self.filter_group,
        });
    }
}

impl Default for SpatialWeaverPlugin {
    fn default() -> Self {
        Self {
            add_rapier: true,
            collider_group: DEFAULT_COL_GROUP,
            filter_group: DEFAULT_FILTER_GROUP,
        }
    }
}

impl SpatialWeaverPlugin {
    pub fn no_rapier_autoconfig() -> Self {
        Self {
            add_rapier: false,
            collider_group: DEFAULT_COL_GROUP,
            filter_group: DEFAULT_FILTER_GROUP,
        }
    }

    pub fn new(add_rapier: bool, collider_group: Group, filter_group: Group) -> Self {
        Self {
            add_rapier,
            collider_group,
            filter_group,
        }
    }
}
