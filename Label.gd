extends Label


# Called when the node enters the scene tree for the first time.
func _ready():
	var rust_string: String = Test.new().helloworld("String from Godot!")
	self.text = rust_string
	var dict = {"question": 21.05}
	self.text += "\n" + str(dict)
	var rust_thing: Dictionary = Test.new().dictionary(dict)
	self.text += "\n" + str(rust_thing)
	var pva : PackedVector3Array = rust_thing["fromRust"]
	self.text += "\n" + str(pva[1])
	var file_data = Test.new().readfile()
	self.text += "\n" + str(file_data)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	pass
