use std::cmp::Ordering;

use crate::InteractionType;

#[test]
pub fn test_eq() {
	let type1 = InteractionType::Raycast;
	let type2 = InteractionType::Raycast;

	assert!(type1.cmp(&type2) == Ordering::Equal);
}

#[test]
pub fn test_lower_priority() {
	let type1 = InteractionType::FuzzySelect;
	let type2 = InteractionType::Raycast;

	assert!(type1.cmp(&type2) == Ordering::Less);
}

#[test]
pub fn test_higher_priority() {
	let type1 = InteractionType::HandPhysics;
	let type2 = InteractionType::Raycast;

	assert!(type1.cmp(&type2) == Ordering::Greater);
}
