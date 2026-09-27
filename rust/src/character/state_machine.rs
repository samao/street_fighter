use godot::{
    classes::{InputEvent, node::ProcessMode},
    init::is_editor_hint,
    prelude::*,
};

#[derive(GodotConvert, Debug, Clone, Copy, Export, Var, Default)]
#[godot(via = GString)]
pub enum PlayerStates {
    #[default]
    Idle,
    Walk,
    Punch,
    Jump,
    JumpKick,
    Kick,
    TakeOff,
    Land,
    Hurt,
    Death,
}

impl From<PlayerStates> for &str {
    fn from(value: PlayerStates) -> Self {
        match value {
            PlayerStates::Idle => "PlayerIdle",
            PlayerStates::Walk => "PlayerWalk",
            PlayerStates::Punch => "PlayerPunch",
            PlayerStates::Jump => "PlayerJump",
            PlayerStates::JumpKick => "PlayerJumpKick",
            PlayerStates::TakeOff => "PlayerTakeOff",
            PlayerStates::Land => "PlayerLand",
            PlayerStates::Hurt => "PlayerHurt",
            PlayerStates::Death => "PlayerDeath",
            PlayerStates::Kick => "PlayerKick",
        }
    }
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub(super) struct CharacterStateMachine {
    base: Base<Node>,

    #[init(val = None)]
    current_states: Option<Gd<Node>>,
}

#[godot_api]
impl INode for CharacterStateMachine {
    fn ready(&mut self) {
        if is_editor_hint() {
            return;
        }
        self.signals()
            .change_player_state()
            .connect_self(Self::on_player_state_change);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(current_state) = self.current_states.as_mut() {
            if current_state.has_method("input_handle") {
                let next_state = current_state.call("input_handle", &[event.to_variant()]);
                if next_state.is_nil() {
                    return;
                }

                if let Some(next_state) = CharacterStateMachine::try_state_from_variant(&next_state)
                {
                    self.on_player_state_change(next_state);
                }
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let delta = delta as f32;
        if let Some(current_state) = self.current_states.as_mut() {
            if current_state.has_method("update") {
                let next_state = current_state.call("update", &[delta.to_variant()]);
                if next_state.is_nil() {
                    return;
                }
                if let Some(next_state) = CharacterStateMachine::try_state_from_variant(&next_state)
                {
                    self.on_player_state_change(next_state);
                }
            }
        }
    }

    // fn get_configuration_warnings(&self) -> PackedStringArray {
    //     let mut warns = PackedArray::<GString>::new();
    //     if is_editor_hint() {
    //         for child in self.base().get_children().iter_shared() {
    //             let state_name = child.get_name();
    //             if CharacterStateMachine::try_state_from_variant(&state_name.to_variant()).is_none()
    //             {
    //                 warns.push(format!("状态节点不存在或者改名了: {state_name}").as_str());
    //             }
    //         }
    //     }
    //     warns
    // }
}

#[godot_api]
impl CharacterStateMachine {
    #[signal]
    pub fn change_player_state(next_state: PlayerStates);
}

impl CharacterStateMachine {
    pub fn on_player_state_change(&mut self, next_state: PlayerStates) {
        self.base_mut().set_process_mode(ProcessMode::DISABLED);
        if let Some(mut state) = self.current_states.take() {
            if state.has_method("exit") {
                godot_print!("退出状态: {:?}", state.get_name());
                state.call("exit", &[]);
            }
        }
        if let Some(mut state_node) = self.get_state_node(next_state.into()) {
            if state_node.has_method("enter") {
                godot_print!("进入状态： {:?}", state_node.get_name());
                state_node.call("enter", &[]);
            }
            self.current_states = Some(state_node);
        } else {
            godot_print!("未配置state: {:?}", next_state);
        }
        self.base_mut().set_process_mode(ProcessMode::INHERIT);
    }

    fn get_state_node(&self, state: &str) -> Option<Gd<Node>> {
        self.base().try_get_node_as::<Node>(state)
    }

    fn try_state_from_variant(state: &Variant) -> Option<PlayerStates> {
        let state_str = String::from_variant(state);
        match state_str.as_str() {
            "PlayerIdle" => Some(PlayerStates::Idle),
            "PlayerWalk" => Some(PlayerStates::Walk),
            "PlayerJump" => Some(PlayerStates::Jump),
            "PlayerJumpKick" => Some(PlayerStates::JumpKick),
            "PlayerPunch" => Some(PlayerStates::Punch),
            "PlayerKick" => Some(PlayerStates::Kick),
            "PlayerHurt" => Some(PlayerStates::Hurt),
            "PlayerDeath" => Some(PlayerStates::Death),
            "PlayerTakeOff" => Some(PlayerStates::TakeOff),
            "PlayerLand" => Some(PlayerStates::Land),
            _ => None,
        }
    }
}
