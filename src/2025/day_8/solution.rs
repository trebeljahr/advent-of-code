use std::collections::{HashMap, HashSet};

pub fn solution() {
    let lines =
        crate::utils::file_reader::lines_from_file("./src/2025/day_8/junction-boxes-example.txt");

    println!("{:?}", lines);

    let mut pos_arr: Vec<Position> = vec![];
    for line in lines {
        let nums: Vec<i64> = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect();

        println!("Nums: {:?}", nums);

        pos_arr.push(Position {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        })
    }

    let mut distances: HashMap<String, i64> = HashMap::new();

    for pos1 in &pos_arr {
        for pos2 in &pos_arr {
            let id1 = &pos1.join(',');
            let id2 = &pos2.join(',');
            let id = make_key_unordered(id1, id2);

            if (id1 == id2) {
                continue;
            };

            let dist_already_in_hash_map = distances.contains_key(&id);

            if (!dist_already_in_hash_map) {
                let dist = calculate_distance_squared(pos1, pos2);

                distances.insert(id, dist);
            }
        }
    }

    let mut dist_arr: Vec<(&String, &i64)> = distances.iter().collect();

    dist_arr.sort_by(|(_, dist1), (_, dist2)| dist1.cmp(dist2));
    println!("Distance Array Sorted: {:?}", dist_arr);

    let mut circuits: Vec<HashSet<String>> = vec![];
    let max_connections = 10;
    for i in 0..max_connections {
        let (key, _dist) = dist_arr[i];

        let ids: Vec<&str> = key.split(':').collect();
        println!("{:?}", ids);

        let id_1 = ids[0];
        let id_2 = ids[1];

        let mut any_circuit_had_id = false;
        for j in 0..circuits.len() {
            let circuit_contains_id_1 = circuits[j].contains(id_1);
            let circuit_contains_id_2 = circuits[j].contains(id_2);

            if (circuit_contains_id_1 && !circuit_contains_id_2) {
                circuits[j].insert(id_2.to_string());
            }

            if (circuit_contains_id_2 && !circuit_contains_id_1) {
                circuits[j].insert(id_1.to_string());
            }

            any_circuit_had_id = circuit_contains_id_1 || circuit_contains_id_2;
        }

        if (any_circuit_had_id) {
            continue;
        }

        let mut new_circuit: HashSet<String> = HashSet::new();
        new_circuit.insert(id_1.to_string());
        new_circuit.insert(id_2.to_string());

        circuits.push(new_circuit);
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    println!("Circuits: {:?}", circuits);

    println!(
        "Circuit lengths: {:?}, {:?}, {:?}",
        circuits[0].len(),
        circuits[1].len(),
        circuits[2].len()
    );

    let out = circuits[0].len() * circuits[1].len() * circuits[2].len();

    println!("Result: {:?}", out);
}

fn make_key_unordered(a: &str, b: &str) -> String {
    if a < b {
        format!("{a}:{b}")
    } else {
        format!("{b}:{a}")
    }
}

struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl Position {
    fn join(&self, sep: char) -> String {
        return format!("{}{}{}{}{}", self.x, sep, self.y, sep, self.z);
    }
}

fn calculate_distance_squared(pos1: &Position, pos2: &Position) -> i64 {
    let dx = pos1.x - pos2.x;
    let dy = pos1.y - pos2.y;
    let dz = pos1.z - pos2.z;
    return dx * dx + dy * dy + dz * dz;
}
