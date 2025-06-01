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

fn map_toggles(hash_map_toggles: &HashMap<&'static str, bool>) {
    black_box(hash_map_toggles.get("Hearts"));
    black_box(hash_map_toggles.get("Tiles"));
    black_box(hash_map_toggles.get("Pikes"));
    black_box(hash_map_toggles.get("Spades"));
}

fn compare_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("Readonly-toggles");

    let toggles: EnumToggles<TestToggles> = EnumToggles::new();

    let mut hash_map_toggles: HashMap<&'static str, bool> = HashMap::new();
    hash_map_toggles.insert("Hearts", false);
    hash_map_toggles.insert("Tiles", false);
    hash_map_toggles.insert("Pikes", false);
    hash_map_toggles.insert("Spades", false);

    group.bench_with_input(
        BenchmarkId::new("Readonly-toggles", "enum_toggles"),
        &toggles,
        |b, input| b.iter(|| enum_toggles(black_box(input))),
    );

    group.bench_with_input(
        BenchmarkId::new("Readonly-toggles", "HashMap"),
        &hash_map_toggles,
        |b, input| b.iter(|| map_toggles(black_box(input))),
    );

    group.finish();
}

criterion_group!(benches, compare_methods);
criterion_main!(benches);
