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
        assert!(
            !self.has_cyclic_dependency(),
            "There were cyclic connections"
        );
        id
    }

    pub fn get_component(&self, id: usize) -> &Component {
        &self.components[id]
    }

    pub fn has_cyclic_dependency(&self) -> bool {
        let mut checked = HashSet::new();
        let mut to_check = (0..self.components.len()).collect::<HashSet<_>>();
        while let Some(&id) = to_check.iter().next() {
            to_check.remove(&id);
            if !checked.insert(id) {
                return true;
            }
            for output in self.components[id]
                .get_outputs()
                .iter()
                .filter_map(|output| *output)
            {
                to_check.insert(output.component);
            }
        }
        false
    }

    pub fn update(&mut self) {
        assert!(
            !self.has_cyclic_dependency(),
            "There were cyclic connections"
        );
        while let Some(&id) = self.changed.iter().next() {
            self.changed.remove(&id);
            match self.components[id] {
                Component::Not { input, output } => {
                    if let Some(output) = output {
                        let input_output =
                            &mut self.components[output.component].get_inputs_mut()[output.index];
                        if input_output.state == input.state {
                            input_output.state = !input.state;
                            self.changed.insert(output.component);
                        }
                    }
                }
                Component::Or { inputs, output } => {
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
            }
        }
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
