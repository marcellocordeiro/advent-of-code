use itertools::Itertools;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub mod part1;
pub mod part2;

#[derive(Debug, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_input(input: &str) -> Vec<Vec3> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Vec3 { x, y, z }
        })
        .collect()
}

fn distance_squared(a: &Vec3, b: &Vec3) -> usize {
    let Vec3 {
        x: p1,
        y: p2,
        z: p3,
    } = *a;

    let Vec3 {
        x: q1,
        y: q2,
        z: q3,
    } = *b;

    ((p1 - q1).pow(2) + (p2 - q2).pow(2) + (p3 - q3).pow(2)) as usize
}
