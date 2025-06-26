use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use enum_toggles::EnumToggles;
use std::collections::HashMap;
use std::hint::black_box;
use strum_macros::{AsRefStr, EnumIter};

#[derive(AsRefStr, EnumIter, PartialEq)]

pub enum TestToggles {
    Hearts,
    Tiles,
    Pikes,
    Spades,
}

fn enum_toggles(toggles: &EnumToggles<TestToggles>) {
    black_box(toggles.get(TestToggles::Hearts as usize));
    black_box(toggles.get(TestToggles::Tiles as usize));
    black_box(toggles.get(TestToggles::Pikes as usize));
    black_box(toggles.get(TestToggles::Spades as usize));
}

fn list_toggles(list_toggles_value: &Vec<bool>) {
    black_box(list_toggles_value[TestToggles::Hearts as usize]);
    black_box(list_toggles_value[TestToggles::Tiles as usize]);
    black_box(list_toggles_value[TestToggles::Pikes as usize]);
    black_box(list_toggles_value[TestToggles::Spades as usize]);
}

fn compare_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("Readonly-toggles");

    let toggles: EnumToggles<TestToggles> = EnumToggles::new();

    let mut hash_map_toggles: HashMap<&'static str, bool> = HashMap::new();
    hash_map_toggles.insert("Hearts", false);
    hash_map_toggles.insert("Tiles", false);
    hash_map_toggles.insert("Pikes", false);
    hash_map_toggles.insert("Spades", false);

    let list_toggles_value: Vec<bool> = vec![false; 4];

    group.bench_with_input(
        BenchmarkId::new("Readonly-toggles", "enum_toggles"),
        &toggles,
        |b, input| b.iter(|| enum_toggles(black_box(input))),
    );

    group.bench_with_input(
        BenchmarkId::new("Readonly-toggles", "List"),
        &list_toggles_value,
        |b, input| b.iter(|| list_toggles(black_box(input))),
    );

    group.finish();
}

criterion_group!(benches, compare_methods);
criterion_main!(benches);
