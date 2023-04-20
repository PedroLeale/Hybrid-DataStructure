use hybrid_data_structure::Hybrid;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use criterion::{
    criterion_group, criterion_main, Criterion, black_box
};

fn insertion_benchmark(c: &mut Criterion) {
    let addresses = black_box(create_addresses(1000, 1));
    let mut hybrid = Hybrid::new(1000, 0.01);

    c.bench_function("Hybrid insert", |b| b.iter(|| {
        for address in &addresses {
            hybrid.insert(address);
        }
    }));
}

fn contains_benchmark(c: &mut Criterion) {
    let addresses = black_box(create_addresses(1000, 1));
    let mut hybrid = Hybrid::new(1000, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    c.bench_function("Hybrid contains", |b| b.iter(|| {
        for address in &addresses {
            hybrid.contains(address);
        }
    }));
}

fn has_intersection_benchmark(c: &mut Criterion) {
    let addresses = black_box(create_addresses(1000, 1));
    let mut hybrid = Hybrid::new(1000, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    let addresses2 = black_box(create_addresses(1000, 1));
    let mut hybrid2 = Hybrid::new(1000, 0.01);
    for address in &addresses2 {
        hybrid2.insert(address);
    }

    c.bench_function("Hybrid has_intersection", |b| b.iter(|| {
        hybrid.has_intersection(&hybrid2);
    }));
}

fn union_benchmark(c: &mut Criterion) {
    //This will measure the time it takes to union 5 hybrid data structures
    //one of them is the same as the base one, so it is expected to have the 
    //contains function avoid insertions on the base one if they are equal.
    let addresses = create_addresses(1000, 1);
    let mut hybrid = Hybrid::new(1000, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    let mut hybrid_array = black_box(Vec::<Hybrid>::new());
    let mut address_array = Vec::<Vec<String>>::new();
    for i in 0..5 {
        address_array.push(create_addresses(1000, i as u64));
    }
    for i in 0..address_array.len() {
        let mut hybrid = Hybrid::new(1000, 0.01);
        for address in &address_array[i] {
            hybrid.insert(address);
        }
        hybrid_array.push(hybrid);
    }

    c.bench_function("Hybrid union", |b| b.iter(|| {
        for i in &hybrid_array {
            hybrid.union(&i);
        }
    }));
}


criterion_group!(benches, insertion_benchmark, contains_benchmark, has_intersection_benchmark, union_benchmark);
criterion_main!(benches);


const BASE58_ALPHABET: &'static str = &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn to_base58_dictionary(value: Vec<u8>) -> String {
    let mut result = String::new();
    for i in value {
        result.push(BASE58_ALPHABET.chars().nth(i as usize).unwrap());
    }
    result
}

fn create_addresses(amount: usize, seed: u64) -> Vec<String> {
    let mut addresses = Vec::new();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for _ in 0..amount {
        let mut temp_vec = Vec::new();
        let mut _temp_string = String::new();
        let rng_num:f64 = rng.gen();

        if rng_num <= 0.5f64 {
            _temp_string = "1".to_string();
        } else {
            _temp_string = "3".to_string();
        }

        for _ in 0..33 {
            let random_character = rng.gen_range(0..58);
            temp_vec.push(random_character);
        }
        _temp_string.push_str(to_base58_dictionary(temp_vec).as_str());
        addresses.push(_temp_string);
    }
    addresses
}