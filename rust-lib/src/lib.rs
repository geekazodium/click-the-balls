use godot::init::gdextension; 
use godot::init::ExtensionLibrary;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

pub mod character;
pub mod targets_counting;
pub mod load_scene;
pub mod calltodelete;
pub mod random_spawner;
pub mod random_util;
pub mod random_init_velocity;
pub mod ui_display;