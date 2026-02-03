@tool
extends EditorScript

func _run() -> void:
	var root = get_scene()
	if root and root.has_node("BlockotNode"):
		root.get_node("BlockotNode").test_move_vertex(2, Vector3(1, 0, 0))
