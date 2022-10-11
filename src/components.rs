use derive_more::{Display, IsVariant};
use enum_as_inner::EnumAsInner;

use crate::ComponentID;

#[derive(Clone, Copy, Display)]
#[display(fmt = "{}", "if *state { \"on\" } else { \"off\" }")]
pub struct Input {
    pub state: bool,
}

#[derive(Clone, Copy, Display)]
#[display(fmt = "Component {component}, Input {index}")]
pub struct Output {
    pub component: ComponentID,
    pub index: usize,
}

#[derive(Clone, IsVariant, EnumAsInner)]
pub enum ComponentType {
    Not {
        input: Input,
        output: Option<Output>,
    },
    Or {
        inputs: [Input; 2],
        output: Option<Output>,
    },
    Delay {
        input: Input,
        output: Option<Output>,
        state_last_frame: bool,
    },
}

#[derive(Clone)]
pub struct Component {
    pub typ: ComponentType,
    pub position: raylib::math::Vector2,
}

impl Component {
    pub fn get_name(&self) -> &str {
        match &self.typ {
            ComponentType::Not {
                input: _,
                output: _,
            } => "Not",
            ComponentType::Or {
                inputs: _,
                output: _,
            } => "Or",
            ComponentType::Delay {
                input: _,
                output: _,
                state_last_frame: _,
            } => "Delay",
        }
    }

    pub fn ignore_cyclic(&self) -> bool {
        match &self.typ {
            ComponentType::Delay {
                input: _,
                output: _,
                state_last_frame: _,
            } => true,
            _ => false,
        }
    }

    pub fn get_inputs(&self) -> &[Input] {
        match &self.typ {
            ComponentType::Not { input, output: _ } => std::array::from_ref(input),
            ComponentType::Or { inputs, output: _ } => inputs,
            ComponentType::Delay {
                input,
                output: _,
                state_last_frame: _,
            } => std::array::from_ref(input),
        }
    }

    pub fn get_inputs_mut(&mut self) -> &mut [Input] {
        match &mut self.typ {
            ComponentType::Not { input, output: _ } => std::array::from_mut(input),
            ComponentType::Or { inputs, output: _ } => inputs,
            ComponentType::Delay {
                input,
                output: _,
                state_last_frame: _,
            } => std::array::from_mut(input),
        }
    }

    pub fn get_outputs(&self) -> &[Option<Output>] {
        match &self.typ {
            ComponentType::Not { input: _, output } => std::array::from_ref(output),
            ComponentType::Or { inputs: _, output } => std::array::from_ref(output),
            ComponentType::Delay {
                input: _,
                output,
                state_last_frame: _,
            } => std::array::from_ref(output),
        }
    }

    pub fn get_outputs_mut(&mut self) -> &mut [Option<Output>] {
        match &mut self.typ {
            ComponentType::Not { input: _, output } => std::array::from_mut(output),
            ComponentType::Or { inputs: _, output } => std::array::from_mut(output),
            ComponentType::Delay {
                input: _,
                output,
                state_last_frame: _,
            } => std::array::from_mut(output),
        }
    }
}
