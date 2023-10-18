extends Node2D

# Called when the node enters the scene tree for the first time.
func _ready():
	var rust_string: String = test.new().helloworld("String from Godot!")
	print(rust_string)

	var dict = {"question": 21.05}
	print(dict)
	var rust_thing: Dictionary = test.new().dictionary(dict)
	print(rust_thing)
	var pva : PackedVector3Array = rust_thing["fromRust"]
	print(pva[1])
