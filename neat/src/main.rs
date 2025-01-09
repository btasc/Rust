mod neural_network;
mod innovation;

use crate::innovation::InnovationTable;
use crate::neural_network::NeuralNetwork;

fn main() {
    //let mut _map = HashMap::new();
    let mut innovation_table = InnovationTable::new();

    innovation_table.add_connector(1, 2);
    innovation_table.add_connector(1, 3);
    innovation_table.add_connector(1, 4);
    innovation_table.add_connector(2, 4);
    innovation_table.add_connector(3, 4);
    innovation_table.add_connector(2, 3);
    innovation_table.add_connector(4, 5);


    let genome = (
        vec![0, 1, 2, 3, 4, 5],
        vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        vec![true, true, true, true, true, true],
    );

    let mut network = NeuralNetwork::new();
    network.init(genome, innovation_table);
}  