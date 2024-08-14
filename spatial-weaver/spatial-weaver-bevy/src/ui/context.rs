use bevy::prelude::Resource;
use bevy_rapier3d::geometry::{CollisionGroups, Group};

#[derive(Resource)]
pub struct WeaverContext {
    pub(crate) collider_group: Group,
    pub(crate) filter_group: Group,
}

impl WeaverContext {
    /// Creates the collision rules needed for weaver interactables.
    pub fn interactable_col_groups(&self) -> CollisionGroups {
        CollisionGroups::new(self.collider_group, self.filter_group)
    }

    /// Adds the collision rules needed to interact with weaver interactables to
    /// the current given collision rules.
    pub fn interactor_col_groups(
        &self,
        CollisionGroups {
            memberships,
            filters,
        }: CollisionGroups,
    ) -> CollisionGroups {
        CollisionGroups::new(
            memberships | self.filter_group,
            filters | self.collider_group,
        )
    }

    /// Adds the collision rules needed to interact with weaver interactables to
    /// the current given collision rules.
    pub fn exclusive_interactor_col_groups(&self) -> CollisionGroups {
        CollisionGroups::new(self.filter_group, self.collider_group)
    }
}
