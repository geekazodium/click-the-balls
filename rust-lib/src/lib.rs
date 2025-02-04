use godot::init::gdextension; 
use godot::init::ExtensionLibrary;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}

pub mod character;
pub mod calltodelete;