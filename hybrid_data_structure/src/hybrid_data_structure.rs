use bloomfilter::Bloom; // bloomfilter = "1.0.9"
use std::collections::BTreeSet;
use std::collections::btree_set::SymmetricDifference;
use std::collections::btree_set::Iter;

pub struct Hybrid {
    bloom: Bloom<String>,
    set: BTreeSet<String>,
}

impl <'a> Hybrid {
    pub fn new(items_count: usize, fp_rate: f64) -> Hybrid {
        Hybrid {
            bloom: Bloom::new_for_fp_rate(items_count, fp_rate),
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, item: String) {
        self.bloom.set(&item);
        self.set.insert(item);
    }

    pub fn contains(&self, item: &String) -> bool {
        if self.bloom.check(&item) {
            return self.set.contains(item);
        }
        false
    }

    pub fn get_item(&self, item: String) -> Option<&String> {
        if self.bloom.check(&item) {
            return self.set.get(&item).clone();
        }
        None
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<String> {
        self.set.iter()
    }

    pub fn iterate(&self, iter: fn(&String)) {
        // A function that will iterate throught the set and call the function iter on each item
        for item in self.set.iter() {
            iter(item);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }

    pub fn is_subset(&self, other: &Hybrid) -> bool {
        self.set.is_subset(&other.set)
    }

    pub fn is_superset(&self, other: &Hybrid) -> bool {
        self.set.is_superset(&other.set)
    }

    pub fn is_disjoint(&self, other: &Hybrid) -> bool {
        self.set.is_disjoint(&other.set)
    }

    pub fn symmetric_difference(&'a self, other: &'a Hybrid) ->  SymmetricDifference<'_, String> {
        return self.set.symmetric_difference(&other.set);
    }

    pub fn intersection(&'a self, other: &'a Hybrid) -> std::collections::btree_set::Intersection<'_, String> {
        return self.set.intersection(&other.set);
    }

    pub fn union(&'a self, other: &'a Hybrid) -> std::collections::btree_set::Union<'_, String> {
        return self.set.union(&other.set);
    }

    pub fn hybrid_union(&'a mut self, other: &'a mut Hybrid) -> &Hybrid { //Needs to be optimized
        // If self is smaller, then insert all items from self into "other" and return other, else do the opposite
        if self.len() < other.len() {
            let new_hybrid = other;
            for item in self.iter() {
                new_hybrid.insert(item.to_string());
            }
            return new_hybrid;
        } else {
            let new_hybrid = self;
            for item in other.iter() {
                new_hybrid.insert(item.to_string());
            }
            return new_hybrid;
        }
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn get_set(&self) -> &BTreeSet<String> {
        &self.set
    }

    pub fn get_set_mut(&mut self) -> &mut BTreeSet<String> {
        &mut self.set
    }

    pub fn get_bloom(&self) -> &Bloom<String> {
        &self.bloom
    }

    pub fn get_bloom_mut(&mut self) -> &mut Bloom<String> {
        &mut self.bloom
    }
}