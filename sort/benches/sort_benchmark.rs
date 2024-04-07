use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sort::{bubble_sort, quick_sort};

fn read_data_from_file<P>(filename: P) -> io::Result<Vec<i32>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buffer = io::BufReader::new(file);
    Ok(buffer
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>())
}

// #[allow(unused)]
fn benchmark_sort(c: &mut Criterion) {
    let file_sizes = vec![10, 100, 1000, 10000, 100000];

    match std::env::current_dir() {
        Ok(cwd) => println!("{}", cwd.display()),
        Err(_) => panic!("err getting CWD"),
    };

    for &size in &file_sizes {
        let data = read_data_from_file(format!("./benches/data_{}.txt", size))
            .expect("Failed to read data");

        let mut group = c.benchmark_group(format!("Sort - {} items", size));

        group.bench_function("bubble_sort", |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                bubble_sort(black_box(&mut data_clone));
            })
        });

        group.bench_function("quick sort", |b| {
            b.iter(|| {
                let mut data_clone = data.clone();
                quick_sort(black_box(&mut data_clone));
            })
        });

        group.finish();
    }
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);
