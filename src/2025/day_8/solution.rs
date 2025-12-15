use std::collections::{HashMap, HashSet};

pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_8/junction-boxes.txt");

    // println!("{:?}", lines);

    let mut pos_arr: Vec<Position> = vec![];
    for i in 0..lines.len() {
        let nums: Vec<i64> = lines[i]
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        // println!("Nums: {:?}", nums);

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
    // println!("Distance Array Sorted: {:?}", dist_arr);

    let mut circuits: Vec<HashSet<String>> = vec![];
    let mut i = 0;
    // let max_connections = 1000;

    while i < dist_arr.len() {
        let (key, _dist) = dist_arr[i];

        let ids: Vec<&str> = key.split(':').collect();

        let id_1 = ids[0];
        let id_2 = ids[1];

        if (circuits.len() == 1 && circuits[0].len() == lines.len()) {
            let (key, _dist) = dist_arr[i - 1];

            let ids: Vec<&str> = key.split(':').collect();

            let a = ids[0].split(',').take(1).collect::<Vec<&str>>()[0]
                .parse::<i64>()
                .unwrap();
            let b = ids[1].split(',').take(1).collect::<Vec<&str>>()[0]
                .parse::<i64>()
                .unwrap();

            println!(
                "Done iterating!!! Last connection was from {:?} to {:?}",
                a, b
            );

            println!("Result of X values multiplied {:?}", a * b,);

            break;
        }

        // println!(
        //     "\n\nNum Connections Made So Far: {}, max_i {}",
        //     i,
        //     dist_arr.len(),
        // );
        // println!("Current Circuits: {:?}", circuits);
        // println!("Current connection: {}-{}", id_1, id_2);

        if (circuits.len() >= 3) {
            let mut circuits_clone = circuits.clone();
            circuits_clone.sort_by(|a, b| b.len().cmp(&a.len()));

            // println!(
            //     "3 Biggest Circuit lengths: {:?}, {:?}, {:?}",
            //     circuits_clone[0].len(),
            //     circuits_clone[1].len(),
            //     circuits_clone[2].len()
            // );
        }

        i += 1;

        let circuits_clone = circuits.clone();

        let idx_of_circuits_containing_id_1: Vec<(usize, &HashSet<String>)> = circuits_clone
            .iter()
            .enumerate()
            .filter(|(_, circuit)| circuit.contains(id_1))
            .collect();

        let idx_of_circuits_containing_id_2: Vec<(usize, &HashSet<String>)> = circuits_clone
            .iter()
            .enumerate()
            .filter(|(_, circuit)| circuit.contains(id_2))
            .collect();

        if (idx_of_circuits_containing_id_1.len() == 1
            && idx_of_circuits_containing_id_2.len() == 1)
        {
            let (idx_1, _dist) = idx_of_circuits_containing_id_1[0];

            let (idx_2, _dist) = idx_of_circuits_containing_id_2[0];

            if (idx_1 == idx_2) {
                // println!(
                //     "Connection {}-{} is in circuit {} already",
                //     id_1, id_2, idx_1
                // );
                // println!("Skipping...");
                continue;
            }

            // println!(
            //     "Connection {}-{} is in circuits {} and {} already",
            //     id_1, id_2, idx_1, idx_2
            // );

            let merged: HashSet<_> = circuits[idx_1].union(&circuits[idx_2]).cloned().collect();

            circuits[idx_1] = merged;
            circuits.remove(idx_2);
        }

        if (idx_of_circuits_containing_id_1.len() > 0 && idx_of_circuits_containing_id_2.len() == 0)
        {
            let (idx, _dist) = idx_of_circuits_containing_id_1[0];
            circuits[idx].insert(id_2.to_string());
        }

        if (idx_of_circuits_containing_id_2.len() > 0 && idx_of_circuits_containing_id_1.len() == 0)
        {
            let (idx, _dist) = idx_of_circuits_containing_id_2[0];
            circuits[idx].insert(id_1.to_string());
        }

        let connected_to_existing_circuit =
            idx_of_circuits_containing_id_1.len() > 0 || idx_of_circuits_containing_id_2.len() > 0;

        if (connected_to_existing_circuit) {
            print!("No new circuits created");
            continue;
        }

        let mut new_circuit: HashSet<String> = HashSet::new();
        new_circuit.insert(id_1.to_string());
        new_circuit.insert(id_2.to_string());

        circuits.push(new_circuit);
    }

    for i in 0..circuits.len() {
        for j in 0..circuits.len() {
            if (i == j) {
                continue;
            }

            circuits[i].iter().for_each(|id| {
                if (circuits[j].contains(id)) {
                    println!("Conflict, found overlapping circuits");
                }
            });
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    // println!("\n\nCircuits Final: {:?}", circuits);
    // println!("Circuits Length: {:?}", circuits.len());

    // println!("Circuit Lengths: ");
    // for i in 0..circuits.len() {
    //     print!("{:?} ", circuits[i].len());
    // }
    // print!("\n");

    // println!(
    //     "3 Biggest Circuit lengths: {:?}, {:?}, {:?}",
    //     circuits[0].len(),
    //     circuits[1].len(),
    //     circuits[2].len()
    // );

    // let out = circuits[0].len() * circuits[1].len() * circuits[2].len();

    // println!("Result: {:?}", out);
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
