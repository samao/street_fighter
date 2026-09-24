use godot::classes::{Input, InputEvent, Os};
use godot::init::is_editor_hint;
use godot::prelude::*;

use crate::entities::actor::Actor;
use crate::resources::health::Health;
use crate::states::state_machine::StateMachine;

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct Controler {
    base: Base<Node>,

    #[init(val = None)]
    slaver: Option<Gd<Actor>>,

    #[export]
    #[init(val = 200.0)]
    speed: f32,

    #[export(range = (0.0, 1.0, or_greater))]
    #[init(val = 0.4)]
    decelerate: f32,

    #[export]
    #[init(val = None)]
    health: Option<Gd<Health>>,

    #[export]
    #[init(val = None)]
    state_machine: Option<Gd<StateMachine>>,

    #[init(val = false)]
    is_option_pressed: bool,
}

#[godot_api]
impl INode for Controler {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }

        if let Some(parent) = self.base().get_parent() {
            if let Ok(actor) = parent.try_cast::<Actor>() {
                self.state_machine = match actor.try_get_node_as::<StateMachine>("%StateMachine") {
                    Some(mut sm) => {
                        sm.call_deferred("transition", &["idle".to_variant()]);
                        Some(sm)
                    }
                    None => None,
                };
                self.health = Some(actor.bind().get_actor_health());
                self.slaver = Some(actor);
            }
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(state_machine) = self.state_machine.as_mut() {
            if event.is_action_pressed("punch") {
                //切换状态
                let anim_name = if self.is_option_pressed {
                    "heavy_punch"
                } else {
                    "punch"
                };
                state_machine.bind_mut().transition(anim_name.to_owned());
            } else if event.is_action_pressed("jump") {
                //切换状态
                state_machine.bind_mut().transition("jump".to_owned());
            } else if event.is_action_pressed("kick") {
                //切换状态
                let anim_name = if self.is_option_pressed {
                    "heavy_kick"
                } else {
                    "kick"
                };
                state_machine.bind_mut().transition(anim_name.to_owned());
            }
        }

        if Os::singleton().is_debug_build()
            && let Some(health) = self.health.as_mut()
        {
            if event.is_action_pressed("test") {
                health.bind_mut().take_damage(3.0);
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let dt = delta as f32;
        let dir = Input::singleton().get_vector("left", "right", "up", "down");
        let velocity = dir * (self.speed * dt);

        if let Some(state_machine) = self.state_machine.as_mut() {
            if velocity.length_squared() > 0.001 {
                state_machine.bind_mut().transition("run".into());
            } else {
                state_machine.bind_mut().transition("idle".into());
            }
        }

        self.is_option_pressed = Input::singleton().is_action_pressed("option");
    }
}
