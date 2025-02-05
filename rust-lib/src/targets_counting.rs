use godot::classes::INode3D;
use godot::classes::Node3D;
use godot::classes::Node;
use godot::classes::PackedScene;
use godot::global::godot_print;
use godot::obj::Base;
use godot::obj::Gd;
use godot::obj::WithBaseField;
use godot::prelude::godot_api;
use godot::prelude::GodotClass;

#[derive(GodotClass)]
#[class(base = Node3D, init)]
struct TargetsCounter{
    base: Base<Node3D>,
    sphere_count: i32,
    game_ended: bool
}

#[godot_api]
impl INode3D for TargetsCounter{
    fn ready(&mut self){
        self.sphere_count = self.base().get_child_count();
        self.game_ended = false;
    }
    fn process(&mut self, _delta: f64){
        //godot_print!("boop {}", self.base().get_child_count());
        if self.base().get_child_count() == 0{
            if self.game_ended{
                return;
            }
            self.game_ended = true;
            godot_print!("game ended");
            self.base_mut().emit_signal("on_completed", &[]);
        }
    }
}

#[godot_api]
impl TargetsCounter{
    #[signal]
    fn on_completed();
}

#[derive(GodotClass)]
#[class(base = Node, init)]
struct GameOverInit{
    base: Base<Node>,
    #[export]
    game_over_ui: Option<Gd<PackedScene>>
}

#[godot_api]
impl GameOverInit{
    #[func]
    fn show_game_over_screen(&mut self){
        let init = self.game_over_ui
            .as_ref()
            .expect("no instantiation set")
            .instantiate()
            .expect("failed to init scene");
        self.base_mut().add_child(&init);
    }
}