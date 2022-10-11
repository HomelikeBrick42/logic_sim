use std::{collections::HashSet, fmt::Display};

use derive_more::Display;

use crate::{Component, ComponentType};

#[derive(Clone, Copy, Display, PartialEq, Eq)]
#[display(fmt = "{_0}")]
pub struct ComponentID(usize);

#[derive(Clone)]
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

    pub fn add_component(&mut self, component: Component) -> ComponentID {
        let id = self.components.len();
        self.components.push(component);
        self.changed.insert(id);
        ComponentID(id)
    }

    pub fn get_component(&self, id: ComponentID) -> &Component {
        &self.components[id.0]
    }

    pub fn get_component_mut(&mut self, id: ComponentID) -> &mut Component {
        self.changed.insert(id.0);
        &mut self.components[id.0]
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
                if self.components[id].ignore_cyclic() {
                    continue;
                }
                if !checked.insert(id) {
                    return true;
                }
                for output in self.components[id]
                    .get_outputs()
                    .iter()
                    .filter_map(|output| *output)
                {
                    to_check_this_iteration.insert(output.component.0);
                    to_check.remove(&output.component.0);
                }
            }
        }
        false
    }

    pub fn update(&mut self) -> bool {
        assert!(
            !self.has_cyclic_dependency(),
            "There were cyclic connections"
        );
        let mut needs_update_next_frame = HashSet::new();
        let had_changes = self.changed.len() > 0;
        while let Some(&id) = self.changed.iter().next() {
            self.changed.remove(&id);
            match &mut self.components[id].typ {
                &mut ComponentType::Not { input, output } => {
                    if let Some(output) = output {
                        let input_output =
                            &mut self.components[output.component.0].get_inputs_mut()[output.index];
                        if input_output.state == input.state {
                            input_output.state = !input.state;
                            self.changed.insert(output.component.0);
                        }
                    }
                }
                &mut ComponentType::Or { inputs, output } => {
                    if let Some(output) = output {
                        let input_output =
                            &mut self.components[output.component.0].get_inputs_mut()[output.index];
                        let old_state = input_output.state;
                        input_output.state = inputs.iter().any(|input| input.state);
                        if input_output.state != old_state {
                            self.changed.insert(output.component.0);
                        }
                    }
                }
                ComponentType::Delay {
                    input,
                    output,
                    state_last_frame,
                } => {
                    if !needs_update_next_frame.contains(&id) {
                        let old_state = *state_last_frame;
                        *state_last_frame = input.state;
                        if old_state != *state_last_frame {
                            needs_update_next_frame.insert(id);
                        }
                        if let Some(output) = *output {
                            let input_output = &mut self.components[output.component.0]
                                .get_inputs_mut()[output.index];
                            if input_output.state != old_state {
                                input_output.state = old_state;
                                if output.component.0 != id {
                                    self.changed.insert(output.component.0);
                                }
                            }
                        }
                    }
                }
            }
        }
        self.changed.extend(needs_update_next_frame.iter());
        had_changes
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

impl IntoIterator for &Scene {
    type Item = ComponentID;

    type IntoIter = std::iter::Map<std::ops::Range<usize>, fn(usize) -> ComponentID>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.components.len()).map(|id| ComponentID(id))
    }
}
