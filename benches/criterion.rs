use criterion::{criterion_group, criterion_main, Criterion, PlottingBackend};

use advent::solutions::{self, SolutionCollection};

pub fn all_years(c: &mut Criterion) {
    do_bench(c, 2019, solutions::days_2019());
    do_bench(c, 2020, solutions::days_2020());
    do_bench(c, 2021, solutions::days_2021());
    do_bench(c, 2022, solutions::days_2022());
    do_bench(c, 2023, solutions::days_2023());
}

fn do_bench(c: &mut Criterion, year: u32, days: SolutionCollection) {
    let mut group = c.benchmark_group(format!("{}", year));
    for day in days.solutions() {
        let path = format!("input/year_{}/day_{:02}.txt", year, day.day);
        let input =
            std::fs::read_to_string(&path).expect(&format!("input path '{}' not found", path));
        let name_one = format!("{:02}-{}", day.day, 1);
        let name_two = format!("{:02}-{}", day.day, 2);
        group.bench_function(&name_one, |b| b.iter(|| (day.part_one)(&input)));
        group.bench_function(&name_two, |b| b.iter(|| (day.part_two)(&input)));
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().plotting_backend(PlottingBackend::Plotters);
    targets = all_years
}

criterion_main!(benches);
