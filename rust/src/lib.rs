use godot::init::InitStage;
use godot::prelude::*;

// Core modules (pure Rust)
pub mod error;
pub mod geometry;
pub mod test_utils;
pub mod tools;

// Editor module (Godot integration)
pub mod editor;

struct BlockotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for BlockotExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            godot_print!("Blockot plugin loaded successfully");
        }
    }
}
