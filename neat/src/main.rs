mod neural_network;
mod innovation;

use crate::innovation::InnovationTable;
use crate::neural_network::NeuralNetwork;

fn main() {
    //let mut _map = HashMap::new();
    let mut innovation_table = InnovationTable::new();

    innovation_table.add_connector(1, 2);

    let genome = (
        vec![],
        vec![],
        vec![],
    );

    let mut network = NeuralNetwork::new();
    network.init(genome, innovation_table);
}  