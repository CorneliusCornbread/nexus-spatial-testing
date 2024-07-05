pub trait Button {
	fn on_press_down(&self, callback: fn());

	fn on_press_release(&self, callback: fn());
}

pub trait Button3D: Button {
	/// Called when the depth of the user's press on the 3D button changes.
	/// Ranges from 0 - 1.0 where 0 is not pressed at all and 1.0 is fully pressed.
	fn on_press_pressure(&self, callback: fn(&f32));
}

pub trait TextInput {
	fn on_activate_keyboard(&self);

	fn on_deactivate_keyboard(&self);

	fn on_key_press(&self, key: &char);
}
