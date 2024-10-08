pub mod button;
pub mod context;
pub mod interactors;

use bevy::{
    ecs::{
        entity::Entity,
        system::{Query, Res},
        world::World,
    },
    transform::components::GlobalTransform,
};
use bevy_rapier3d::plugin::RapierContext;
use interactors::Interactor3D;

fn check_ui_intersections(
    pointer: Query<(Entity, &Interactor3D)>,
    rapier_context: Res<RapierContext>,
    world: &World,
) {
    // TODO: change this later, we're going to support multiple pointers, I'm just lazy
    let entity = pointer
        .get_single()
        .expect("FIXME: multiple pointers detected in scene");

    /* Iterate through all the intersection pairs involving a specific collider. */
    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity.0) {
        if intersecting {
            println!(
                "The entities {:?} and {:?} have intersecting colliders!",
                collider1, collider2
            );

            let col1_ref = world.entity(collider1);
            let col1_transform: &GlobalTransform =
                col1_ref.get().expect("Collider1 does not have transform");

            let col2_ref = world.entity(collider2);
            let col2_transform: &GlobalTransform =
                col2_ref.get().expect("Collider2 does not have transform");

            let transformed = col1_transform.transform_point(col2_transform.translation());
        }
    }
}
