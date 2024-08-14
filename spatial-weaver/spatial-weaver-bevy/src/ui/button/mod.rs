pub mod rapier;

use bevy::prelude::Component;

#[derive(Component)]
pub struct BevyButton3D {
    pub activate_pressure: f32,
    pub release_pressure: f32,
    current_pressure: f32,
    pressed: bool,
}

impl BevyButton3D {
    pub fn is_pressed(&mut self) -> bool {
        if self.pressed {
            if self.current_pressure <= self.release_pressure {
                self.pressed = false;
            }
        } else if self.current_pressure >= self.activate_pressure {
            self.pressed = true;
        }

        self.pressed
    }
}

const DEFAULT_ACTIVATE_PRESSURE: f32 = 0.6;
const DEFAULT_RELEASE_PRESSURE: f32 = 0.3;

impl Default for BevyButton3D {
    fn default() -> Self {
        Self {
            activate_pressure: DEFAULT_ACTIVATE_PRESSURE,
            release_pressure: DEFAULT_RELEASE_PRESSURE,
            current_pressure: 0.0,
            pressed: false,
        }
    }
}
