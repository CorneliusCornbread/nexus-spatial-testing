use std::cmp::Ordering;

pub mod locomotion;
pub mod spacial;
#[cfg(test)]
mod tests;
pub mod ui;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
	HandPhysics = 0,
	Raycast = 1,
	FuzzySelect = 2,
}

impl Ord for InteractionType {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		let self_int = *self as u32;
		let other_int = *other as u32;

		self_int.cmp(&other_int).reverse()
	}

	fn max(self, other: Self) -> Self
	where
		Self: Sized,
	{
		std::cmp::max_by(self, other, Ord::cmp)
	}

	fn min(self, other: Self) -> Self
	where
		Self: Sized,
	{
		std::cmp::min_by(self, other, Ord::cmp)
	}

	fn clamp(self, min: Self, max: Self) -> Self
	where
		Self: Sized,
		Self: PartialOrd,
	{
		assert!(min <= max);
		if self > max {
			max
		} else if self < min {
			min
		} else {
			self
		}
	}
}

impl PartialOrd for InteractionType {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub struct TransformPos {
	pub pos_x: f32,
	pub pos_y: f32,
	pub pos_z: f32,
}

pub struct TransformRot {
	pub rot_x: f32,
	pub rot_y: f32,
	pub rot_z: f32,
	pub rot_w: f32,
}

pub struct TransformData {
	pub pos: TransformPos,
	pub rot: TransformRot,
}

impl From<TransformData> for TransformPos {
	fn from(value: TransformData) -> Self {
		value.pos
	}
}

impl From<TransformData> for TransformRot {
	fn from(value: TransformData) -> Self {
		value.rot
	}
}

impl AsRef<TransformRot> for TransformData {
	fn as_ref(&self) -> &TransformRot {
		&self.rot
	}
}

impl AsRef<TransformPos> for TransformData {
	fn as_ref(&self) -> &TransformPos {
		&self.pos
	}
}
