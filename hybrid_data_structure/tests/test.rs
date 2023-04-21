#[cfg(test)]
mod tests {
    use hybrid_data_structure::Hybrid;



    #[test]
    fn is_empty(){
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
    fn get_item(){
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");
        assert!(hybrid.get_item("test") == Some(&"test") && hybrid.get_item("test2") == Some(&"test2") && hybrid.get_item("test3") == None);
    }

    #[test]
    fn has_intersection(){
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
    fn union(){
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test2");
        hybrid2.insert("test3");

        hybrid.union(&hybrid2);

        assert!(hybrid.contains("test3") && hybrid.contains("test2") && hybrid.contains("test3") && (hybrid.len() == 3));
    }

    #[test]
    fn is_superset(){
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
    fn is_subset(){
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
    fn is_disjoint(){
        let mut hybrid = Hybrid::new(10, 0.01);
        hybrid.insert("test");
        hybrid.insert("test2");

        let mut hybrid2 = Hybrid::new(10, 0.01);
        hybrid2.insert("test3");
        hybrid2.insert("test4");

        assert!(hybrid.is_disjoint(&hybrid2) && hybrid2.is_disjoint(&hybrid));
    }
}