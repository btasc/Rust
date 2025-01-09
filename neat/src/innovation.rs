use std::collections::HashMap;

#[derive(Clone)]
pub struct Innovation<'a> {
    pub id: i32,
    pub from: i32,
    pub to: i32,
    pub r#type: &'a str,
    pub neuron_id: Option<i32>,
}

pub struct InnovationTable<'a> {
    table: Vec<Innovation<'a>>,
    innov_search: HashMap<(i32, i32, &'a str), Innovation<'a>>,
    current_innovation_id: i32,
    current_neuron_id: i32
}

impl<'a> InnovationTable<'a> {
    pub fn new() -> InnovationTable<'a> {
        InnovationTable {
            table: Vec::new(),
            innov_search: HashMap::new(),
            current_innovation_id: 0,
            current_neuron_id: 0
        }
    }

    pub fn add_connector(&mut self, from: i32, to: i32) {
        let new_innovation = Innovation {
            id: self.current_innovation_id,
            from: from,
            to: to,
            r#type: "Connector",
            neuron_id: None,
        };

        self.current_innovation_id += 1;

        self.table.push(new_innovation.clone());
        self.innov_search.insert((from, to, "Connector"), new_innovation);
    }

    pub fn add_neuron_connector(&mut self, from: i32, to: i32) {
        let new_neuron_id: i32 = self.current_neuron_id;

        let new_neuron_innovation = Innovation {
            id: self.current_innovation_id,
            from: from,
            to: to,
            r#type: "Neuron",
            neuron_id: Some(new_neuron_id),
        };

        self.current_innovation_id += 1;

        self.current_neuron_id += 1;

        println!("{:?}", new_neuron_id);

        self.table.push(new_neuron_innovation.clone());
        self.innov_search.insert((from, to, "Neuron"), new_neuron_innovation);
    }

    pub fn search(&self, from: i32, to: i32, r#type: &str) -> Option<Innovation> {
        self.innov_search.get(&(from, to, r#type)).cloned()
    }

    pub fn get(&self, id: i32) -> Innovation {
        self.table[id as usize].clone()
    }
}