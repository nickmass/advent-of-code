use criterion::{criterion_group, criterion_main, Criterion};

use advent::solutions::{self, Solution};

pub fn all_years(c: &mut Criterion) {
    do_bench(c, 2019, solutions::days_2019());
    do_bench(c, 2020, solutions::days_2020());
    do_bench(c, 2021, solutions::days_2021());
}

fn do_bench(c: &mut Criterion, year: u32, days: Vec<Solution>) {
    let mut group = c.benchmark_group(format!("{}", year));
    for day in days {
        let input = std::fs::read_to_string(format!("input/{}/day{}.txt", year, day.day)).unwrap();
        let name_one = format!("{:02}-{}", day.day, 1);
        let name_two = format!("{:02}-{}", day.day, 2);
        group.bench_function(&name_one, |b| b.iter(|| (day.part_one)(&input)));
        group.bench_function(&name_two, |b| b.iter(|| (day.part_two)(&input)));
    }
    group.finish();
}

criterion_group!(benches, all_years);
criterion_main!(benches);
