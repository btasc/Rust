use std::collections::HashMap;

#[derive(Clone)]
struct Innovation {
    id: i32,
    from: i32,
    tp: i32
}

pub struct InnovationTable {
    table: Vec<Innovation>,
    innov_search: HashMap<(i32, i32), Innovation>
}

impl InnovationTable {
    let mut innovation_id = 0;

    add_connector(&mut self, from: i32, to: i32) {
        let new_innovation = Innovation {

        };
    }
}