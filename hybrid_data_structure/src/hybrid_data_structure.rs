//Authors: Pedro Leale
//         Dr. Ivan da Silva Sendin

use bloomfilter::Bloom; // bloomfilter = "1.0.9"
use std::collections::BTreeSet;
struct HybridBase<'a> {
    bloom: Bloom<str>,
    set: BTreeSet<&'a str>,
}

pub struct Hybrid <'a>  {
    base: Vec<HybridBase<'a>>,
}

impl <'a> Hybrid <'a>{
    pub fn new(items_count: usize, fp_rate: f64) -> Hybrid <'a> {
        Hybrid {
            base: vec![HybridBase {
                bloom: Bloom::new_for_fp_rate(items_count, fp_rate),
                set: BTreeSet::new(),
            }],
        }
    }

    pub fn insert(&mut self, item: &'a str) {
        //Always insert on the last one
        //Can be changed later
        self.base.last_mut().unwrap().bloom.set(item);
        self.base.last_mut().unwrap().set.insert(item);
    }

    pub fn contains(&self, item: &'a str) -> bool {
        //Start checking from last to first, because most insertions happens on the last position
        //Will only impact if a union has occurred 
        for base in self.base.iter().rev() { 
            if base.bloom.check(item) {
                return base.set.contains(item);
            }
        }
        false
    }

    pub fn get_item(&self, item: &'a str) -> Option<&'a str> {
        for base in self.base.iter().rev(){
            if base.bloom.check(&item) {
                return base.set.get(item).copied();
            }
        }
        None
    }
    
    pub fn iter(&'a self) -> impl Iterator<Item = &'a &'a str> {
        self.base
            .iter()
            .flat_map(|base| base.set.iter())
    }

    pub fn get_iter(&self) -> impl Iterator<Item = &str> + '_ {
        self.base.iter().flat_map(|base| base.set.iter().map(|s| *s))
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty()
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
            .flat_map(move |base| base.set.iter().filter(move |s| other.contains(*s)))
    }


    //I am still testing, between intersection_boolean and has_intersection
    //wich is faster, for my benchmarks intersection_boolean is faster, but I need further testing to prove it

    pub fn intersection_boolean(&'a self, other: &'a Hybrid) -> bool {
        //Returns true if an intersection exists, false if not
        if self.intersection(other).next().is_some() {
            return true;
        }
        false
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

    pub fn union(&mut self, other: &'a mut Hybrid<'a>) -> &mut Hybrid<'a> {
        //Makes "other" part of "self", if other is empty, nothing happens
        //If other's length is too small, it will be inserted in a loop
        //Else everything will be pushed to the list.
        //"too small" will be altered after more benchmarking.
        if other.is_empty() {
            return self;
        }
        if other.len() <= 1000 {
            for item in other.get_iter() {
                self.insert(item);
            }
            return self;
        }
        loop {
            let safe_check = other.base.pop();
            if let Some(_) = safe_check {
                self.base.push(safe_check.unwrap());
            } else {
                break;
            }
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