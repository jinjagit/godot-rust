use godot::prelude::*;
use godot::prelude::{Dictionary, PackedVector3Array, Vector3};
struct testExtension;

#[gdextension]
unsafe impl ExtensionLibrary for testExtension {}

use godot::engine::Node2D;
use godot::engine::Node2DVirtual;

#[derive(GodotClass)]
#[class(base=Node2D)]

struct test{

    #[base]
    node2d: Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for test{
    fn init(node2d: Base<Node2D>) -> Self {
        // godot_print!("Hello from Rust!");
        Self { node2d }
    }
}


#[godot_api]
impl test {
    #[func]
    fn helloworld(&mut self, gd_string: GodotString) -> GodotString {
        format!("String from Rust! {}", gd_string).into()
    }

    #[func]
    fn dictionary(&mut self, from_godot: Dictionary) ->  Dictionary {
        // haven't found a way to avoid converting to string and then parsing an integer
        // note we need to use f64, as Godot uses 64-bit floats and f32 will be slightly innacurate.
        // Godot also uses 64-bit integers, so we can use i64 here in rust land.
        let num: f64 = from_godot.get("question").unwrap().to_string().parse().unwrap();

        // Assign directly to Godot types
        let mut dict = Dictionary::new();
        let a = Vector3::new(1.4,2.7,3.2);
        let b = Vector3::new(4.4,5.9,6.1);

        let mut c = PackedVector3Array::new();
        c.push(a);
        c.push(b);

        dict.insert("fromRust", c);
        dict.insert("answer", num * 2.0);
        dict
    }
}
