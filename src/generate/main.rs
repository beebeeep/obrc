use std::{
    env::args,
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let mut f = BufWriter::new(File::create(args().nth(1).unwrap()).unwrap());
    let cities: Vec<&str> = include_str!("cities.txt").split("\n").collect();

    for _i in 0..1_000_000_000 {
        let n = (rand::random::<i32>() % 1000) as f32 / 10.0;
        write!(
            f,
            "{};{n:.1}\n",
            cities[rand::random::<usize>() % cities.len()]
        )
        .unwrap();
    }
}
