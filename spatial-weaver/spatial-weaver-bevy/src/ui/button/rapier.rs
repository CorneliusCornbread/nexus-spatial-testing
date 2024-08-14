use bevy::prelude::Bundle;
use bevy_rapier3d::{
    dynamics::RigidBody,
    geometry::{Collider, CollisionGroups},
};

use crate::ui::context::WeaverContext;

use super::BevyButton3D;

const BUTTON_RB_TYPE: RigidBody = RigidBody::KinematicPositionBased;

#[derive(Bundle)]
pub struct ButtonRapier {
    rigid_body: RigidBody,
    collider: Collider,
    col_groups: CollisionGroups,
    button: BevyButton3D,
}

impl ButtonRapier {
    pub fn new(ctx: &WeaverContext, hx: f32, hy: f32, hz: f32) -> Self {
        Self {
            rigid_body: BUTTON_RB_TYPE,
            collider: Collider::cuboid(hx, hy, hz),
            col_groups: CollisionGroups::new(ctx.collider_group, ctx.filter_group),
            button: Default::default(),
        }
    }
}
