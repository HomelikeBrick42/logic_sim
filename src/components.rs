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
        }
    }

    pub fn get_inputs(&self) -> &[Input] {
        match self {
            Component::Not { input, output: _ } => std::array::from_ref(input),
            Component::Or { inputs, output: _ } => inputs,
        }
    }

    pub fn get_inputs_mut(&mut self) -> &mut [Input] {
        match self {
            Component::Not { input, output: _ } => std::array::from_mut(input),
            Component::Or { inputs, output: _ } => inputs,
        }
    }

    pub fn get_outputs(&self) -> &[Option<Output>] {
        match self {
            Component::Not { input: _, output } => std::array::from_ref(output),
            Component::Or { inputs: _, output } => std::array::from_ref(output),
        }
    }

    pub fn get_outputs_mut(&mut self) -> &mut [Option<Output>] {
        match self {
            Component::Not { input: _, output } => std::array::from_mut(output),
            Component::Or { inputs: _, output } => std::array::from_mut(output),
        }
    }
}
