use std::fs::read_to_string;

type Coord = (i64, i64);

enum Orientation {
    U,
    D,
    L,
    R,
}

struct Instruction {
    orientation: Orientation,
    distance: i64,
}

fn manhattan_distance_from_origin(p: Coord) -> u64 {
    return ((0 - p.0).abs() + (0 - p.1).abs()) as u64;
}

fn minimum_manhattan_distance(points: Vec<Coord>) -> u64 {
    let mut current_minimum :u64 = std::u64::MAX;
    for point in points {
        let distance = manhattan_distance_from_origin(point);
        if distance < current_minimum && distance > 0
        {
            current_minimum = distance;
        }
    }
    return current_minimum;
}

fn parse_instructions(inst_txt: String) -> Vec<Vec<Instruction>> {
    let mut instructions: Vec<Vec<Instruction>> = vec![];
    let wires = inst_txt.lines();
    for wire in wires {
        println!("{:?}", wire);
        let wire_instructions: Vec<Instruction> = wire.split(',').map(|s| Instruction {
            orientation: match s.chars().next().unwrap() {
                'U' => Orientation::U,
                'D' => Orientation::D,
                'L' => Orientation::L,
                'R' => Orientation::R,
                _ => unreachable!(),
            },
            distance: s[1..].parse().unwrap(),
        }).collect();
        instructions.push(wire_instructions);
    }

    return instructions;
}

fn find_path_coords(wire_path: Vec<Instruction>) -> Vec<Coord> {
    let mut path: Vec<Coord> = vec![];
    let mut pt: Coord = (0, 0);

    for direction in wire_path {
        match direction.orientation {
            Orientation::U => for i in 0..direction.distance + 1 { path.push((pt.0, pt.1 + i)) },
            Orientation::D => for i in 0..direction.distance + 1 { path.push((pt.0, pt.1 - i)) },
            Orientation::R => for i in 0..direction.distance + 1 { path.push((pt.0 + i, pt.1)) },
            Orientation::L => for i in 0..direction.distance + 1 { path.push((pt.0 - i, pt.1)) },
        }
        pt = *path.last().unwrap();
    }
    
    return path;
}

fn find_coord_duplicates(wire_1: &Vec<Coord>, wire_2: &Vec<Coord>) -> Vec<Coord> {
    let mut collisions: Vec<Coord> = vec![];

    if wire_1.len() > wire_2.len() || wire_1.len() == wire_2.len()
    {
        for point_1 in wire_1
        {
            for point_2 in wire_2
            {
                if point_1 == point_2
                {
                    collisions.push(*point_1);
                }
            }
        }
    }
    else
    {
        for point_2 in wire_2
        {
            for point_1 in wire_1
            {
                if point_1 == point_2
                {
                    collisions.push(*point_1);
                }
            }
        }
    }
    return collisions;
}

fn main() {
    let input = read_to_string("input/input.txt").unwrap();
    let wires = parse_instructions(input);
    let mut paths: Vec<Vec<Coord>> = vec![];
    for wire in wires
    {
        // convert the instructions into a vector of coords
        let path = find_path_coords(wire);
        paths.push(path);
    }

    let collisions = find_coord_duplicates(&paths[0], &paths[1]);
    println!("collisions {:?}", collisions);
    let minimum_manhattan_distance = minimum_manhattan_distance(collisions);
    println!("minimum manhattan distance to nearest crossing: {:?}", minimum_manhattan_distance);

}
