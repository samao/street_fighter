use godot::{classes::Area2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Area2D)]
pub struct DamageEmitter {
    base: Base<Area2D>,
}
