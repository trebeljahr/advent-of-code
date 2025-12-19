pub fn solution() {
    let lines = crate::utils::file_reader::lines_from_file("./src/2025/day_9/red-tiles.txt");

    println!("{:?}", lines);

    let mut coordinates: Vec<Coordinates> = vec![];

    for i in 0..lines.len() {
        let coords = parse_input(lines[i].clone());

        coordinates.push(coords);
    }

    let mut possible_areas: Vec<i64> = vec![];
    for i in 0..coordinates.len() {
        for j in 0..coordinates.len() {
            let coords1 = coordinates[i];
            let coords2 = coordinates[j];

            let area = ((coords1.x - coords2.x).abs() + 1) * ((coords1.y - coords2.y).abs() + 1);
            possible_areas.push(area.abs());
        }
    }

    possible_areas.sort();
    possible_areas.reverse();
    println!("Possible Areas {:?}", possible_areas);
    println!("Largest Area Found: {:?}", possible_areas[0]);
}

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: i64,
    y: i64,
}

fn parse_input(num_str: String) -> Coordinates {
    let result: Vec<i64> = num_str
        .split(',')
        .into_iter()
        .map(|str| str.parse::<i64>().unwrap())
        .collect();

    let out = Coordinates {
        x: result[0],
        y: result[1],
    };

    return out;
}
