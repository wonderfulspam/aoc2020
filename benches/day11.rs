use aoc2020::solutions::*;
use criterion::{criterion_group, criterion_main, AxisScale, Criterion, PlotConfiguration};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("Solutions");
    group.plot_config(plot_config);
    group.bench_function("day11 map part 1", |b| {
        b.iter(|| day11::test_neighbour_map1())
    });
    group.bench_function("day11 map part 2", |b| {
        b.iter(|| day11::test_neighbour_map2())
    });
    group.bench_function("day11 total", |b| b.iter(|| day11::run()));
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
