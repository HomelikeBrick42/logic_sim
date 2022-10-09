use std::{collections::HashSet, fmt::Display};

use crate::Component;

pub struct Scene {
    components: Vec<Component>,
    changed: HashSet<usize>,
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, component) in self.components.iter().enumerate() {
            writeln!(f, "{i}: {}", component.get_name())?;
            let inputs = component.get_inputs();
            if inputs.len() > 0 {
                writeln!(f, "  Inputs:")?;
                for (i, input) in inputs.iter().enumerate() {
                    writeln!(f, "    Input {i}: {input}")?;
                }
            }
            let outputs = component.get_outputs();
            if outputs.len() > 0 {
                writeln!(f, "  Outputs:")?;
                for (i, output) in outputs.iter().enumerate() {
                    write!(f, "    Output {i}: ")?;
                    if let Some(output) = output {
                        writeln!(f, "{output}")?;
                    } else {
                        writeln!(f, "Not Connected")?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Scene {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_component(&mut self, component: Component) -> usize {
        let id = self.components.len();
        self.components.push(component);
        self.changed.insert(id);
        id
    }

    pub fn get_component(&self, id: usize) -> &Component {
        &self.components[id]
    }

    pub fn get_component_mut(&mut self, id: usize) -> &mut Component {
        self.changed.insert(id);
        &mut self.components[id]
    }

    pub fn has_cyclic_dependency(&self) -> bool {
        let mut to_check = (0..self.components.len()).collect::<HashSet<_>>();
        while to_check.len() > 0 {
            let mut to_check_this_iteration = HashSet::from([{
                let element = *to_check.iter().next().unwrap();
                to_check.remove(&element);
                element
            }]);
            let mut checked = HashSet::new();
            while let Some(&id) = to_check_this_iteration.iter().next() {
                to_check_this_iteration.remove(&id);
                if !checked.insert(id) {
                    return true;
                }
                for output in self.components[id]
                    .get_outputs()
                    .iter()
                    .filter_map(|output| *output)
                {
                    to_check_this_iteration.insert(output.component);
                    to_check.remove(&output.component);
                }
            }
        }
        false
    }

    pub fn update(&mut self) {
        assert!(
            !self.has_cyclic_dependency(),
            "There were cyclic connections"
        );
        let mut needs_update_next_frame = HashSet::new();
        while let Some(&id) = self.changed.iter().next() {
            self.changed.remove(&id);
            match &mut self.components[id] {
                &mut Component::Not { input, output } => {
                    if let Some(output) = output {
                        let input_output =
                            &mut self.components[output.component].get_inputs_mut()[output.index];
                        if input_output.state == input.state {
                            input_output.state = !input.state;
                            self.changed.insert(output.component);
                        }
                    }
                }
                &mut Component::Or { inputs, output } => {
                    if let Some(output) = output {
                        let input_output =
                            &mut self.components[output.component].get_inputs_mut()[output.index];
                        let old_state = input_output.state;
                        input_output.state = inputs.iter().any(|input| input.state);
                        if input_output.state != old_state {
                            self.changed.insert(output.component);
                        }
                    }
                }
                Component::Delay {
                    input,
                    output,
                    state_last_frame,
                } => {
                    if !needs_update_next_frame.contains(&id) {
                        let old_state = *state_last_frame;
                        *state_last_frame = input.state;
                        if let Some(output) = *output {
                            let input_output = &mut self.components[output.component]
                                .get_inputs_mut()[output.index];
                            input_output.state = old_state;
                            if input_output.state != old_state {
                                self.changed.insert(output.component);
                            }
                        }
                    }
                    needs_update_next_frame.insert(id);
                }
            }
        }
        self.changed.extend(needs_update_next_frame.iter());
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            components: Default::default(),
            changed: Default::default(),
        }
    }
}
