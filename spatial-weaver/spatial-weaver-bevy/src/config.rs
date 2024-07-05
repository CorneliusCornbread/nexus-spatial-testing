use bevy::prelude::Resource;
use bevy_rapier3d::geometry::Group;

#[derive(Resource)]
pub(crate) struct SpatialWeaverConfig(pub(crate) Group);
