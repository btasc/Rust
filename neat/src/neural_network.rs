use std::collections::HashMap;

use crate::innovation::InnovationTable;
use crate::innovation::Innovation;

const INPUT_NEURONS: [i32; 3] = [0, 1, 2];
const OUTPUT_NEURONS: [i32; 1] = [3];

#[derive(Debug)]
#[derive(Clone)]
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

#[derive(Debug)]
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
    neurons: HashMap<i32, Neuron>,
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
            neurons: HashMap::new(),
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

        self.neurons.insert(id, new_neuron);
    }

    pub fn init(&mut self, genome: (Vec<i32>, Vec<f64>, Vec<bool>), innovation_table: InnovationTable) {
        check_genome(&genome);
        self.genome = genome;
        let mut neurons: Vec<i32> = vec![]; 

        for i in 0..self.genome.0.len() {
            let innovation = innovation_table.get(self.genome.0[i]);

            if innovation.r#type == "Connector" {
                self.add_connector(
                    innovation.from,
                    innovation.to,
                    self.genome.1[i],
                    i as i32,
                );

                if !neurons.contains(&innovation.from) {
                    neurons.push(innovation.from);
                }

                if !neurons.contains(&innovation.to) {
                    neurons.push(innovation.to);
                }

            } else if innovation.r#type == "Neuron" {
                continue;
            } else {
                panic!("Innovation type is not neuron or connector")
            }

        }

        for id in neurons.iter() {
            self.add_neuron(*id);
        }

        for connector in self.connectors.iter() {
            let from = connector.from;
            let to = connector.to;

            self.neurons.get_mut(&from).unwrap().add_from(connector.id);
            self.neurons.get_mut(&to).unwrap().add_to(connector.id);
        }

        let layers: Vec<Vec<Vec<i32>>> = Vec::new();

        let mut current_neuron_layer: Vec<i32> = Vec::from(INPUT_NEURONS);
        let mut extras: Vec<i32> = Vec::new();

        for (neuron_id, neuron) in self.neurons.iter() {
            if INPUT_NEURONS.contains(neuron_id) {
                continue;
            }

            if neuron.tos.len() == 0 {
                extras.push(*neuron_id);
            }
        }

        let mut used_connectors: Vec<i32> = Vec::new();

        for i in 0..self.genome.0.len() + 1 { // Should be while currentnl > 0 but yknow
            
        }

    } // End of init
} // End of impl NN