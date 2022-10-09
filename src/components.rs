use derive_more::Display;

#[derive(Clone, Copy, Display)]
#[display(fmt = "{}", "if *state { \"on\" } else { \"off\" }")]
pub struct Input {
    pub state: bool,
}

#[derive(Clone, Copy, Display)]
#[display(fmt = "Component {component}, Input {index}")]
pub struct Output {
    pub component: usize,
    pub index: usize,
}

#[derive(Clone, Copy)]
pub enum Component {
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

impl Component {
    pub fn get_name(&self) -> &'static str {
        match self {
            Component::Not {
                input: _,
                output: _,
            } => "Not",
            Component::Or {
                inputs: _,
                output: _,
            } => "Or",
            Component::Delay {
                input: _,
                output: _,
                state_last_frame: _,
            } => "Delay",
        }
    }

    pub fn ignore_cyclic(&self) -> bool {
        match self {
            Component::Delay {
                input: _,
                output: _,
                state_last_frame: _,
            } => true,
            _ => false,
        }
    }

    pub fn get_inputs(&self) -> &[Input] {
        match self {
            Component::Not { input, output: _ } => std::array::from_ref(input),
            Component::Or { inputs, output: _ } => inputs,
            Component::Delay {
                input,
                output: _,
                state_last_frame: _,
            } => std::array::from_ref(input),
        }
    }

    pub fn get_inputs_mut(&mut self) -> &mut [Input] {
        match self {
            Component::Not { input, output: _ } => std::array::from_mut(input),
            Component::Or { inputs, output: _ } => inputs,
            Component::Delay {
                input,
                output: _,
                state_last_frame: _,
            } => std::array::from_mut(input),
        }
    }

    pub fn get_outputs(&self) -> &[Option<Output>] {
        match self {
            Component::Not { input: _, output } => std::array::from_ref(output),
            Component::Or { inputs: _, output } => std::array::from_ref(output),
            Component::Delay {
                input: _,
                output,
                state_last_frame: _,
            } => std::array::from_ref(output),
        }
    }

    pub fn get_outputs_mut(&mut self) -> &mut [Option<Output>] {
        match self {
            Component::Not { input: _, output } => std::array::from_mut(output),
            Component::Or { inputs: _, output } => std::array::from_mut(output),
            Component::Delay {
                input: _,
                output,
                state_last_frame: _,
            } => std::array::from_mut(output),
        }
    }
}
