extern crate rand;
use rand::Rng;

struct Map<K, V> {
    values: Vec<(K, V)>,
}

impl<K: PartialEq, V> Map <K, V> {
    fn new() -> Map<K, V> {
        Map { values: Vec::new(), }
    }

    fn insert(&mut self, key: K , value: V) {
        self.values.push((key, value));
    }

    fn get(&self, key: K) -> Option<&V> {
        for i in 0..self.values.len() {
            if self.values[i].0 == key {
                return Some(&self.values[i].1);
            }
        }

        None
    }
}

fn main() { 
    let mut map = Map::new();

    map.insert("hi", 10);

    let map_answer = map.get("hi").unwrap();
    let map_times_two = map_answer * 2;

    println!("{:?}", map_times_two);
}