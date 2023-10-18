extends Label


# Called when the node enters the scene tree for the first time.
func _ready():
	var rust_string: String = test.new().helloworld("String from Godot!")
	self.text = rust_string


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
