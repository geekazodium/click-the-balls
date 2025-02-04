use godot::classes::Node;
use godot::obj::Base;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node, init)]
struct CallToDelete{
    base: Base<Node>
}

#[godot_api]
impl CallToDelete{
    #[func]
    fn delete(&mut self){
        self.base()
            .get_parent()
            .expect("can't run call to delete on object with no parent")
            .queue_free();
    }
}