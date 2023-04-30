#[cfg(test)]
mod tests {
    use hybrid_data_structure::Hybrid;
    use rand::prelude::*;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn is_empty() {
        let mut hybrid = Hybrid::new(10, 0.01);
        assert!(hybrid.is_empty());
        hybrid.insert("test");
        assert!(!hybrid.is_empty());
    }

    #[test]
    fn insertion_and_contains() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");
        assert!(hybrid.contains("test") && hybrid.contains("test2") && !hybrid.contains("test3"));
    }

    #[test]
    fn get_item() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");
        assert!(
            hybrid.get_item("test") == Some(&"test")
                && hybrid.get_item("test2") == Some(&"test2")
                && hybrid.get_item("test3") == None
        );
    }

    #[test]
    fn has_intersection() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test2");

        let mut hybrid3 = Hybrid::new(10, 0.01);
        hybrid3.insert("test3");

        assert!(hybrid.has_intersection(&hybrid2) && !hybrid.has_intersection(&hybrid3));
    }

    #[test]
    fn union() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test2");
        hybrid2.insert("test3");

        hybrid.union(&hybrid2);

        assert!(
            hybrid.contains("test3")
                && hybrid.contains("test2")
                && hybrid.contains("test3")
                && (hybrid.len() == 3)
        );
    }

    #[test]
    fn is_superset() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");
        hybrid.insert("test3");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test2");
        hybrid2.insert("test3");

        assert!(hybrid.is_superset(&hybrid2) && !hybrid2.is_superset(&hybrid));
    }

    #[test]
    fn is_subset() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test");
        hybrid2.insert("test2");
        hybrid2.insert("test3");

        assert!(hybrid.is_subset(&hybrid2) && !hybrid2.is_subset(&hybrid));
    }

    #[test]
    fn is_disjoint() {
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test3");
        hybrid2.insert("test4");

        assert!(hybrid.is_disjoint(&hybrid2) && hybrid2.is_disjoint(&hybrid));
    }

    #[test]
    fn btreeset_union() {
        let amount = 10000;
        let slices = 100;
        let addresses = create_addresses(amount, 1);
        let mut starting_set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
        let mut full_set = Vec::<std::collections::BTreeSet<&str>>::new();

        for i in 0..slices {
            starting_set.insert(&addresses[i * slices + (slices - 1)]);
        }

        for i in 0..slices {
            let mut set: std::collections::BTreeSet<&str> = std::collections::BTreeSet::new();
            for j in 0..slices {
                set.insert(&addresses[i * slices + j]);
            }
            full_set.push(set);
        }

        for set in &full_set {
            if starting_set.intersection(&set).count() > 0 {
                for i in set {
                    starting_set.insert(i);
                }
            }
        }

        assert_eq!(starting_set.len(), amount);
    }

    const BASE58_ALPHABET: &'static str =
        &"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

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
}
