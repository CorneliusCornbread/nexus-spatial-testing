use crate::TransformData;

/// A trait representing a grabbable object in 3D space.
pub trait Grabbable {
	fn on_grab_start(&self, transform: &TransformData);

	fn on_grab_move(&self, transform: &TransformData);

	fn on_release(&self, transform: &TransformData);
}

pub trait FuzzyGrabbable: Grabbable {}
