use godot::prelude::*;

struct StreetFighterExtension;

#[gdextension]
unsafe impl ExtensionLibrary for StreetFighterExtension {
    fn on_stage_init(stage: InitStage) {
        if stage == InitStage::Scene {
            // godot_print!("rust 扩展加载完毕");
        }
    }

    fn on_stage_deinit(stage: InitStage) {
        if stage == InitStage::Scene {
            // godot_print!("rust 扩展卸载了");
        }
    }

    fn on_main_loop_frame() {}
}

pub(crate) mod character;
pub(crate) mod entities;
pub(crate) mod managers;
pub(crate) mod resources;
pub(crate) mod states;
pub(crate) mod ui;
pub(crate) mod utils;
