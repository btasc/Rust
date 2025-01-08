use std::collections::HashMap;

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
    weight: i32,
}

pub struct Genome {
    innovations: Vec<i32>,
    weights: Vec<f64>,
    statuses: Vec<u8>,
}

pub struct NeuralNetwork {
    layers: Vec<Vec<i32>>,
    order: Vec<Vec<i32>>,
    neurons: Vec<Neuron>,
    connectors: Vec<Connector>,
    con_search: HashMap<(i32, i32), Connector>,
    genome: Genome,
}

impl NeuralNetwork {
    fn add_connector(&mut self, from: i32, to: i32, weight: i32, id: i32) {
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

    fn init(&mut self, genome: Genome) {
        
    }
}