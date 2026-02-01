use godot::prelude::*;
use godot::init::InitStage;

struct BlockotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BlockotExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            godot_print!("Blockot plugin loaded successfully");
        }
    }
}
