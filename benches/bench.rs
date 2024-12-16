// benches/bench_days.rs

use criterion::{criterion_group, criterion_main, Criterion};

use codspeed_aoc::day15;
pub fn bench_day_14(c: &mut Criterion) {
    let mut group = c.benchmark_group(concat!("day", 14));
    let input = include_str!("../inp.txt");
    group.bench_function(format!("part1"), |b| b.iter(|| day15::part1(input)));
    group.bench_function(format!("part2"), |b| b.iter(|| day15::part2(input)));
}

criterion_group!(benches, bench_day_14);
criterion_main!(benches);
