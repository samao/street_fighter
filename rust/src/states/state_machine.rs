use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(crate) struct StateMachine {
    base: Base<Node>,
    #[init(val = None)]
    current_state: Option<Gd<Node>>,
}

#[godot_api]
impl INode for StateMachine {
    fn physics_process(&mut self, delta: f64) {
        if let Some(ref mut state) = self.current_state {
            let next_state = state.call("update", &[delta.to_variant()]);
            if next_state.is_nil() {
                return;
            }
            self.transition(String::from_variant(&next_state));
        }
    }
}

#[godot_api]
impl StateMachine {
    #[func]
    pub(crate) fn transition(&mut self, state_name: String) {
        if let Some(mut state) = self.base().try_get_node_as::<Node>(&state_name) {
            if let Some(mut prev_state) = self.current_state.take() {
                if prev_state.get_name().to_string() == state_name {
                    self.current_state = Some(prev_state);
                    return;
                }
                prev_state.call_deferred("exit", &[]);
            }
            self.current_state = Some(state.clone());
            state.call_deferred("enter", &[]);
        } else {
            godot_print!("不存在状态: {state_name}");
        }
    }
}
