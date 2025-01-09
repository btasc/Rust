use std::collections::HashMap;

use crate::innovation::InnovationTable;
use crate::innovation::Innovation;

struct Neuron {
    value: i32,
    id: i32,
    froms: Vec<i32>,
    tos: Vec<i32>,
}

impl Neuron {
    fn add_from(&mut self, connection: i32) {
        self.froms.push(connection);
    }

    fn add_to(&mut self, connection: i32) {
        self.tos.push(connection);
    }
}

#[derive(Clone)]
struct Connector {
    from: i32,
    to: i32,
    id: i32,
    weight: f64,
}

pub struct NeuralNetwork {
    layers: Vec<Vec<i32>>,
    order: Vec<Vec<i32>>,
    neurons: Vec<Neuron>,
    connectors: Vec<Connector>,
    con_search: HashMap<(i32, i32), Connector>,
    genome: (Vec<i32>, Vec<f64>, Vec<bool>),
}

fn check_genome(genome: &(Vec<i32>, Vec<f64>, Vec<bool>)) {
    let innovations_len = genome.0.len();
    let weights_len = genome.1.len();
    let statuses_len = genome.2.len();

    if innovations_len != weights_len || innovations_len != statuses_len || weights_len != statuses_len {
        panic!("Genome is not valid");
    }
}

impl NeuralNetwork {
    pub fn new() -> NeuralNetwork {
        NeuralNetwork {
            layers: vec![],
            order: vec![],
            neurons: vec![],
            connectors: vec![],
            con_search: HashMap::new(),
            genome: (vec![], vec![], vec![]),
        }
    }

    fn add_connector(&mut self, from: i32, to: i32, weight: f64, id: i32) {
        let new_connector: Connector = Connector {
            from: from,
            to: to,
            weight: weight,
            id: id,
        };

        self.con_search.insert((from, to), new_connector.clone());
        self.connectors.push(new_connector);
    }

    fn add_neuron(&mut self, id: i32) {
        let new_neuron = Neuron {
            value: 0,
            id: id,
            froms: vec![],
            tos: vec![],
        };

        self.neurons.push(new_neuron);
    }

    pub fn init(&mut self, genome: (Vec<i32>, Vec<f64>, Vec<bool>), innovation_table: InnovationTable) {
        check_genome(&genome);
        self.genome = genome;

        for i in 0..self.genome.0.len() {
            let innovation = innovation_table.get(self.genome.0[i]);

            if innovation.r#type == "Connector" {
                
            } else if innovation.r#type == "Neuron" {
                continue;
            } else {
                panic!("Innovation type is not neuron or connector")
            }
        }
    }
}