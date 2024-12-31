use core::f32;
use std::{
    collections::HashMap,
    env::args,
    fs::{self, File},
    io::{BufRead, BufReader, Seek, SeekFrom},
    sync::mpsc,
    thread,
};

#[derive(Debug)]
struct Stat {
    min: f32,
    max: f32,
    avg: f32,
    count: usize,
}

fn read_chunk(filename: &str, from: u64, to: u64) -> HashMap<String, Stat> {
    let mut cities: HashMap<String, Stat> = HashMap::new();
    let mut f = BufReader::new(File::open(&filename).unwrap());
    f.seek(SeekFrom::Start(from)).unwrap();

    let mut city_str = String::new();
    let mut value_str = String::new();
    if from != 0 {
        f.read_until(0x0a, &mut Vec::new()).unwrap();
    }
    let mut count = 0usize;
    let mut pos = from;

    loop {
        city_str.clear();
        value_str.clear();
        unsafe {
            pos += f.read_until(0x3b, city_str.as_mut_vec()).unwrap() as u64;
            pos += f.read_until(0xa, value_str.as_mut_vec()).unwrap() as u64;
        }
        count += 1;
        if pos >= to {
            break;
        }
        let value = value_str[..value_str.len() - 1].parse::<f32>().unwrap();
        match cities.get_mut(&city_str[..city_str.len() - 1]) {
            None => {
                cities.insert(
                    String::from(&city_str[..city_str.len() - 1]),
                    Stat {
                        min: value,
                        max: value,
                        avg: value,
                        count: 1,
                    },
                );
            }
            Some(v) => {
                v.min = v.min.min(value);
                v.max = v.max.max(value);
                v.avg += value;
                v.count += 1;
            }
        }
    }
    println!("read {count} lines");
    return cities;
}

fn main() {
    let filename = args().nth(1).unwrap_or(String::from("input.txt"));
    let thread_num: u64 = args().nth(2).unwrap_or("8".to_string()).parse().unwrap();
    let mut cities: HashMap<String, Stat> = HashMap::new();
    let chunk_len = fs::metadata(&filename).unwrap().len() / thread_num;

    let mut threads = Vec::new();
    let (tx, rx): (
        mpsc::Sender<HashMap<String, Stat>>,
        mpsc::Receiver<HashMap<String, Stat>>,
    ) = mpsc::channel();
    for i in 0..thread_num {
        let tx = tx.clone();
        let filename = filename.clone();
        threads.push(thread::spawn(move || {
            let r = read_chunk(&filename, i * chunk_len, (i + 1) * chunk_len);
            tx.send(r).unwrap();
        }))
    }

    for _i in 0..thread_num {
        for (k, istat) in rx.recv().unwrap() {
            match cities.get_mut(&k) {
                None => {
                    cities.insert(
                        k,
                        Stat {
                            min: istat.min,
                            max: istat.max,
                            avg: istat.avg,
                            count: istat.count,
                        },
                    );
                }
                Some(stat) => {
                    stat.min = stat.min.min(istat.min);
                    stat.max = stat.max.max(istat.max);
                    stat.avg += istat.avg;
                    stat.count += istat.count;
                }
            }
        }
    }
    for thread in threads {
        thread.join().unwrap()
    }

    cities
        .iter_mut()
        .for_each(|(_, s)| s.avg = s.avg / (s.count as f32));

    for (k, v) in cities {
        println!("{k} -> {:?}", v);
    }
}
