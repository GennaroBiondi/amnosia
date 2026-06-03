use amnosia::reminder_list::ReminderList;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{hint::black_box, path::PathBuf}; // fix name to match your crate

const FILE_10K: &str = "benches/testing_data/file_10k.test";
const FILE_100K: &str = "benches/testing_data/file_100k.test";

fn make_big_list(path_str: &str) -> ReminderList {
    let path = PathBuf::from(path_str);
    ReminderList::from_file(&path).unwrap()
}

fn bench_fuzzy_search_10k(c: &mut Criterion) {
    let list = make_big_list(FILE_10K);
    c.bench_function("fuzzy search 10k entries", |b| {
        b.iter(|| list.find_reminders_by_fuzzy_entry(black_box("meeting")))
    });
}

fn bench_fuzzy_search_100k(c: &mut Criterion) {
    let list = make_big_list(FILE_100K);
    c.bench_function("fuzzy search 100k entries", |b| {
        b.iter(|| list.find_reminders_by_fuzzy_entry(black_box("meeting")))
    });
}

criterion_group!(benches, bench_fuzzy_search_10k, bench_fuzzy_search_100k);
criterion_main!(benches);
