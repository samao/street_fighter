use godot::{classes::Texture2D, prelude::*};

#[derive(GodotClass)]
#[class(init, base = Resource)]
pub(crate) struct EnemySetting {
    base: Base<Resource>,

    #[export]
    #[var(pub)]
    texture: OnEditor<Gd<Texture2D>>,

    #[export]
    #[init(val = 3.0)]
    #[var(pub)]
    max_hp: f32,

    #[export]
    #[init(val = 1.0)]
    #[var(pub)]
    damage: f32,
}
