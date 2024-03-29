//Authors: Pedro Leale
//         Dr. Ivan da Silva Sendin

use bloomfilter::Bloom; // bloomfilter = "1.0.9"
use std::collections::BTreeSet;
struct HybridBase<'a> {
    bloom: Bloom<str>,
    bloom_size: usize,
    bloom_fp_rate: f64,
    set: BTreeSet<&'a str>,
}

pub struct Hybrid<'a> {
    base: Vec<HybridBase<'a>>,
}

impl<'a> Hybrid<'a> {
    pub fn new(items_count: usize, fp_rate: f64) -> Hybrid<'a> {
        Hybrid {
            base: vec![HybridBase {
                bloom: Bloom::new_for_fp_rate_with_seed(items_count, fp_rate, &[0u8; 32]),
                bloom_size: items_count,
                bloom_fp_rate: fp_rate,
                set: BTreeSet::new(),
            }],
        }
    }

    pub fn insert(&mut self, item: &'a str) {
        if self.contains(item) {
            return;
        }
        //Always insert on the last one
        if self.base.last().unwrap().bloom_size == self.base.last().unwrap().set.len() {
            //If the last one is full, create a new one
            self.base.push(HybridBase {
                bloom: Bloom::new_for_fp_rate_with_seed(
                    self.base.last().unwrap().bloom_size,
                    self.base.last().unwrap().bloom_fp_rate,
                    &[0u8; 32],
                ),
                bloom_size: self.base.last().unwrap().bloom_size,
                bloom_fp_rate: self.base.last().unwrap().bloom_fp_rate,
                set: BTreeSet::new(),
            });
        }
        self.base.last_mut().unwrap().set.insert(item);
        self.base.last_mut().unwrap().bloom.set(item);
    }

    pub fn contains(&self, item: &'a str) -> bool {
        //Start checking from first to last, because the first ones should have more items
        for base in self.base.iter() {
            if base.bloom.check(item) && base.set.contains(item){
                return true;
            } else {
                continue;
            }
        }
        false
    }

    pub fn get_item(&self, item: &'a str) -> Option<&'a str> {
        for base in self.base.iter().rev() {
            if base.bloom.check(item) {
                return base.set.get(item).copied();
            }
        }
        None
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &'a &'a str> {
        self.base.iter().flat_map(|base| base.set.iter())
    }

    pub fn get_iter(&self) -> impl Iterator<Item = &str> + '_ {
        self.base.iter().flat_map(|base| base.set.iter().copied())
    }

    pub fn is_empty(&self) -> bool {
        for i in self.base.iter() {
            if !i.set.is_empty() {
                return false;
            }
        }
        true
    }

    //Optimize is_subset, is_superset and is_disjoint
    pub fn is_subset(&self, other: &Hybrid) -> bool {
        //Check if all items in self are in other
        for item in self.get_iter() {
            if !other.contains(item) {
                return false;
            }
        }
        true
    }

    pub fn is_superset(&self, other: &Hybrid) -> bool {
        //Check if all items in other are in self
        for item in other.get_iter() {
            if !self.contains(item) {
                return false;
            }
        }
        true
    }

    pub fn is_disjoint(&self, other: &Hybrid) -> bool {
        //Check if there is no intersection between self and other
        for item in self.get_iter() {
            if other.contains(item) {
                return false;
            }
        }
        true
    }

    //Implement intersection, that returns an chained iterator (using methods like flatmap) of all intersections from every base
    pub fn intersection(&'a self, other: &'a Hybrid) -> impl Iterator<Item = &'a &'a str> + '_ {
        self.base
            .iter()
            .flat_map(move |base| base.set.iter().filter(move |s| other.contains(s)))
    }

    pub fn has_intersection(&'a self, other: &'a Hybrid) -> bool {
        //Returns true if an intersection exists, false if not
        for item in self.get_iter() {
            if other.contains(item) {
                return true;
            }
        }
        false
    }

    pub fn union(&mut self, other: &'a Hybrid<'a>) -> &mut Hybrid<'a> {
        if other.is_empty() {
            return self;
        }
        for item in other.get_iter() {
            self.insert(item);
        }
        self
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for base in self.base.iter() {
            len += base.set.len();
        }
        len
    }
}