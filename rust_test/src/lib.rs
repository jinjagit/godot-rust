use godot::prelude::*;
use godot::prelude::{Dictionary, PackedVector3Array, Vector3};
use godot::engine::Node2D;
use godot::engine::Node2DVirtual;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

struct TestExtension;

#[gdextension]
unsafe impl ExtensionLibrary for TestExtension {}

// Creating a node, though we never use it.
#[derive(GodotClass)]
#[class(base=Node2D)]

struct Test{

    #[base]
    node2d: Base<Node2D>
}

#[godot_api]
impl Node2DVirtual for Test{
    fn init(node2d: Base<Node2D>) -> Self {
        // godot_print!("Hello from Rust!");
        Self { node2d }
    }
}
// ------------------------------------------------------

// Define struct that will be deserialized from binary file
#[derive(Deserialize, PartialEq, Debug)]
struct Pair {
    x: f32,
    y: f32,
}

#[derive(Deserialize, PartialEq, Debug)]
struct MyStruct(Vec<Pair>);

#[godot_api]
impl Test {
    #[func]
    fn helloworld(&mut self, gd_string: GodotString) -> GodotString {
        format!("String from Rust! {}", gd_string).into()
    }

    // Passing dictionaries back and forth.
    // I really want to be able to pass groups of PackedVector3Array to Godot, so this is a test of that.
    #[func]
    fn dictionary(&mut self, from_godot: Dictionary) ->  Dictionary {
        // haven't found a way to avoid converting to string and then parsing an integer
        // note the need to use f64, as Godot uses 64-bit floats and f32 will be slightly innacurate.
        // Godot also uses 64-bit integers, so can use i64 here in Rust land.
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

    #[func]
    fn readfile(&mut self) -> GodotString {
        // Read the data back from the file
        // Note the weird path to the data folder. This doesn't match the path used to write, but it works.
        let mut file_to_read_from = BufReader::new(File::open("./data/foo.dat").unwrap());
        let deserialized_struct : MyStruct = bincode::deserialize_from(&mut file_to_read_from).unwrap();

        // drop the open file from scope
        drop(file_to_read_from);

        format!("Read from binary file by Rust! {:?}", deserialized_struct).into()
    }
}
