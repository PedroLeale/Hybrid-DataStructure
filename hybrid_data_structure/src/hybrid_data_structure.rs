use bloomfilter::Bloom; // bloomfilter = "1.0.9"
use std::collections::BTreeSet;
use std::collections::btree_set::SymmetricDifference;

pub struct Hybrid <'a> {
    bloom: Bloom<&'a str>,
    set: BTreeSet<&'a str>,
}

impl <'a> Hybrid <'a> {
    pub fn new(items_count: usize, fp_rate: f64) -> Hybrid <'a> {
        Hybrid {
            bloom: Bloom::new_for_fp_rate(items_count, fp_rate),
            set: BTreeSet::new(),
        }
    }

    pub fn insert(&mut self, item: &'a str) {
        self.bloom.set(&item);
        self.set.insert(item);
    }

    pub fn contains(&self, item: &'a str) -> bool {
        if self.bloom.check(&item) {
            return self.set.contains(item);
        }
        false
    }

    pub fn get_item(&self, item: &'a str) -> Option<&'a str> {
        if self.bloom.check(&item) {
            return self.set.get(&item).copied();
        }
        None
    }

    pub fn iter(&self) -> std::collections::btree_set::Iter<&str> {
        self.set.iter()
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

    pub fn symmetric_difference(&self, other: &'a Hybrid) ->  SymmetricDifference<'_, &str> {
        return self.set.symmetric_difference(&other.set);
    }

    pub fn intersection(&self, other: &'a Hybrid) -> std::collections::btree_set::Intersection<'_, &str> {
        return self.set.intersection(&other.set);
    }

    pub fn union(&self, other: &'a Hybrid) -> std::collections::btree_set::Union<'_, &str> {
        //Work this one out to make it smarter
        return self.set.union(&other.set);
    }

    pub fn hybrid_union(&'a mut self, other: &'a mut Hybrid<'a>) -> &Hybrid {
        if self.len() < other.len() {
            let new_hybrid = other;
            for item in self.iter() {
                new_hybrid.insert(item);
            }
            return new_hybrid;
        } else {
            let new_hybrid = self;
            for item in other.iter() {
                new_hybrid.insert(item);
            }
            return new_hybrid;
        }
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn get_set(&self) -> &BTreeSet<&'a str> {
        &self.set
    }

    pub fn get_set_mut(&mut self) -> &mut BTreeSet<&'a str> {
        &mut self.set
    }

    pub fn get_bloom(&self) -> &Bloom<&'a str> {
        &self.bloom
    }

    pub fn get_bloom_mut(&mut self) -> &mut Bloom<&'a str> {
        &mut self.bloom
    }
}
