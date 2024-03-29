use hybrid_data_structure::Hybrid;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn insertion_benchmark(c: &mut Criterion) {
    let addresses = black_box(create_addresses(1000, 1));
    let mut hybrid = Hybrid::new(1000, 0.01);

    c.bench_function("Hybrid insert", |b| {
        b.iter(|| {
            for address in &addresses {
                hybrid.insert(address);
            }
        })
    });
}

fn contains_benchmark(c: &mut Criterion) {
    let addresses = black_box(create_addresses(1000, 1));
    let mut hybrid = Hybrid::new(1000, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    c.bench_function("Hybrid contains", |b| {
        b.iter(|| {
            for address in &addresses {
                hybrid.contains(address);
            }
        })
    });
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

    c.bench_function("Hybrid has_intersection", |b| {
        b.iter(|| {
            hybrid.has_intersection(&hybrid2);
        })
    });
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

    c.bench_function("Hybrid union", |b| {
        b.iter(|| {
            for i in &hybrid_array {
                hybrid.union(&i);
            }
        })
    });
}

fn heavy_union_benchmark(c: &mut Criterion) {
    //Same as union_benchmark but with way more data
    //May take some minutes if you run it with "address_amount" equal to millions
    let address_amount = 10000;
    let addresses = create_addresses(address_amount, 1);
    let mut hybrid = Hybrid::new(address_amount, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    let mut hybrid_array = black_box(Vec::<Hybrid>::new());
    let mut address_array = Vec::<Vec<String>>::new();
    for i in 0..5 {
        address_array.push(create_addresses(address_amount, i as u64));
    }
    for i in 0..address_array.len() {
        let mut hybrid = Hybrid::new(address_amount, 0.01);
        for address in &address_array[i] {
            hybrid.insert(address);
        }
        hybrid_array.push(hybrid);
    }

    c.bench_function("Heavy Hybrid union", |b| {
        b.iter(|| {
            for i in &hybrid_array {
                hybrid.union(&i);
            }
        })
    });
}

fn get_iter(c: &mut Criterion) {
    let amount = 10000;
    let slices = 100;
    let addresses = black_box(create_addresses(amount, 1));
    let mut hybrid = Hybrid::new(slices, 0.01);
    for address in &addresses {
        hybrid.insert(address);
    }

    c.bench_function("Hybrid get_iter", |b| {
        b.iter(|| {
            for address in hybrid.iter() {
                black_box(address);
            }
        })
    });
}

fn h1_heuristic(c: &mut Criterion) {
    //This heuristic works as follows:
    //If there is an intersection between two sets of inputs from bitcoin transactions,
    //then make an union between then, and repeat until you've checked all the inputs.
    //For this test I will simulate a starting set of inputs, and a list of transaction inputs to check.
    //If you want to change the sizes, just change "amount" and "buckets" variables, with big numbers it may take some minutes.
    //But keep in mind: Amount size should be equal to buckets * buckets, otherwise you may need to change the code in the insertion part.
    let amount = 1000000;
    let buckets = 1000;

    let addresses = create_addresses(amount, 1);
    let mut starting_set = Hybrid::new(amount, 0.01);
    let mut full_set = black_box(Vec::<Hybrid>::new());

    //For this test, the starting set will have the first "buckets" addresses
    //so intersection will be guaranteed and unions will happen at every iteration of the loop

    for i in 0..buckets {
        starting_set.insert(&addresses[i * buckets + (buckets - 1)]);
    }

    for i in 0..buckets {
        let mut hybrid = Hybrid::new(buckets, 0.01);
        for j in 0..buckets {
            hybrid.insert(&addresses[i * buckets + j]);
        }
        full_set.push(hybrid);
    }
    let mut group = c.benchmark_group("H1 Heuristic");
    //Only 10 samples because this tests take quite a long time, you can change it if you want
    //but if you make it 100 samples I recommend you to change the amount and buckets variables
    group.sample_size(10);
    group.bench_function("H1 Heuristic Hybrid", |b| {
        b.iter(|| {
            for set in &full_set {
                if starting_set.has_intersection(&set) {
                    starting_set.union(&set);
                }
            }
        })
    });
}

fn h1_heuristic_btreeset(c: &mut Criterion) {
    //Same as h1_heuristic but using a BTreeSet instead of a Hybrid
    let amount = 1000000;
    let buckets = 1000;

    let addresses = create_addresses(amount, 1);
    let mut starting_set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
    let mut full_set = black_box(Vec::<std::collections::BTreeSet<&str>>::new());

    for i in 0..buckets {
        starting_set.insert(&addresses[i * buckets + (buckets - 1)]);
    }

    for i in 0..buckets {
        let mut set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        for j in 0..buckets {
            set.insert(&addresses[i * buckets + j]);
        }
        full_set.push(set);
    }
    let mut group = c.benchmark_group("H1 Heuristic");
    group.sample_size(10);
    group.bench_function("H1 Heuristic BTreeSet", |b| {
        b.iter(|| {
            for set in &full_set {
                if starting_set.intersection(&set).count() > 0 {
                    for i in set {
                        starting_set.insert(i);
                    }
                }
            }
        })
    });
}

criterion_group!(
    benches,
    insertion_benchmark,
    contains_benchmark,
    has_intersection_benchmark,
    union_benchmark,
    heavy_union_benchmark,
    get_iter,
    h1_heuristic,
    h1_heuristic_btreeset
);
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
        let rng_num: f64 = rng.gen();

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
