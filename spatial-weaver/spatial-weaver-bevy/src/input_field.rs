use bevy::ecs::component::Component;

#[derive(Component)]
pub struct InputFieldData {
	pub is_active: bool,
	pub text: String,
}

#[derive(Component)]
pub struct InputFieldStyle<'a> {
	pub label: &'a str,
}
