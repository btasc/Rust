use std::collections::HashMap;

#[derive(Clone)]
struct Innovation {
    id: i32,
    from: i32,
    to: i32,
    r#type: &str,
}

pub struct InnovationTable {
    table: Vec<Innovation>,
    innov_search: HashMap<(i32, i32, &str), Innovation>
}

impl InnovationTable {
    let mut innovation_id = 0;
    let mut neuron_id = 0;

    add_connector(&mut self, from: i32, to: i32) {
        let new_innovation = Innovation {
            id: innovation_id,
            from: from,
            to: to,
            r#type: "Connector"
        };

        innovation_id += 1;

        self.table.push(new_innovation.clone());
        self.innov_search.insert((from, to, "Connector"), new_innovation);
    }

    add_neuron_connector(&mut self, from: i32, to: i32) {
        let new_neuron_innovation = Innovation {
            id: innovation_id,
            from: from,
            to: to,
            r#type: "Neuron"
        };

        let new_neuron_id = 0;

        innovation_id += 1;

        self.table.push(new_neuron_innovation.clone());
        self.innov_search.insert((from, to, "Neuron"), new_innovation);
    }
}