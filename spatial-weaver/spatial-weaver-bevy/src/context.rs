use bevy::prelude::Resource;
use bevy_rapier3d::rapier::geometry::{ColliderBuilder, Group, InteractionGroups};

#[derive(Resource)]
pub struct WeaverContext {
    pub(crate) collider_group: Group,
    pub(crate) filter_group: Group,
}

trait WeaverBuilder {
    /// Sets the current collider to have the collision rules needed for weaver interactables.
    /// Will override any existing collision rules previously set.
    fn make_weaver_collider(self, ctx: &WeaverContext) -> Self;

    /// Adds the collision rules needed to interact with weaver interactables.
    /// Will keep existing collision rules previously set.
    fn make_weaver_interactor(self, ctx: &WeaverContext) -> Self;
}

impl WeaverBuilder for ColliderBuilder {
    fn make_weaver_collider(self, ctx: &WeaverContext) -> Self {
        self.collision_groups(InteractionGroups::new(ctx.collider_group, ctx.filter_group))
    }

    fn make_weaver_interactor(self, ctx: &WeaverContext) -> Self {
        let groups = self.collision_groups;
        let new_groups = InteractionGroups::new(
            groups.memberships | ctx.collider_group,
            groups.filter | ctx.filter_group,
        );

        self.collision_groups(new_groups)
    }
}
