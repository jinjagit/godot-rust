use godot::prelude::*;

struct testExtension;

#[gdextension]
unsafe impl ExtensionLibrary for testExtension {}

use godot::engine::Node2D;

#[derive(GodotClass)]
#[class(base=Node2D)]

struct test{

    #[base]
    node2d: Base<Node2D>
}

use godot::engine::Node2DVirtual;

#[godot_api]
impl Node2DVirtual for test{
    fn init(node2d: Base<Node2D>) -> Self {
        godot_print!("Hello from Rust!");
        Self { node2d }
    }
}