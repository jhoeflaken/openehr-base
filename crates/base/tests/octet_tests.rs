#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use base::foundation_types::{Any, Octet};

    #[test]
    fn octet_new_creates_instance() {
        let o = Octet::new(10);
        assert_eq!(o.value, 10);
    }

    #[test]
    fn octet_is_equal_with_octet() {
        let o1 = Octet::new(10);
        let o2 = Octet::new(10);
        assert!(o1.is_equal(&o2));
    }

    #[test]
    fn octet_is_equal_with_u8() {
        let o = Octet::new(10);
        assert!(o == 10);
    }

    #[test]
    fn octet_partial_eq_with_octet() {
        let o1 = Octet::new(10);
        let o2 = Octet::new(10);
        assert!(o1 == o2);
    }

    #[test]
    fn octet_partial_eq_with_u8() {
        let o = Octet::new(10);
        assert!(o == 10);
    }

    #[test]
    fn octet_instance_of() {
        let o = Octet::new(10);
        assert!(o.instance_of("Octet"));
    }

    #[test]
    fn octet_type_of() {
        let o = Octet::new(10);
        assert_eq!(o.type_of(), "Octet");
    }

    #[test]
    fn octet_ord_comparison() {
        let o1 = Octet::new(10);
        let o2 = Octet::new(20);
        assert!(o1 < o2);
    }

    #[test]
    fn octet_partial_ord_comparison() {
        let o1 = Octet::new(10);
        let o2 = Octet::new(20);
        assert_eq!(o1.partial_cmp(&o2).unwrap(), Ordering::Less);
    }

}