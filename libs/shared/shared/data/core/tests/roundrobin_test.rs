use shared_shared_data_core::roundrobin::RoundRobin;

#[test]
fn test_next_value_cycles_through_values() {
    let mut rr = RoundRobin::new(vec![1, 2, 3]);
    assert_eq!(*rr.next_value(), 1);
    assert_eq!(*rr.next_value(), 2);
    assert_eq!(*rr.next_value(), 3);
    assert_eq!(*rr.next_value(), 1); // Should cycle back
}

#[test]
fn test_next_value_with_strings() {
    let mut rr = RoundRobin::new(vec!["a".to_string(), "b".to_string()]);
    assert_eq!(rr.next_value(), "a");
    assert_eq!(rr.next_value(), "b");
    assert_eq!(rr.next_value(), "a");
}

#[test]
#[should_panic(expected = "Cannot call next_value on an empty RoundRobin generator.")]
fn test_next_value_empty_panics() {
    let mut rr: RoundRobin<i32> = RoundRobin::new(vec![]);
    rr.next_value();
}

#[test]
fn test_replace_values_resets_index() {
    let mut rr = RoundRobin::new(vec![1, 2, 3]);
    rr.next_value(); // index = 1
    rr.replace_values(vec![10, 20]);
    assert_eq!(*rr.next_value(), 10);
    assert_eq!(*rr.next_value(), 20);
}

#[test]
fn test_replace_values_with_empty_then_panic() {
    let result = std::panic::catch_unwind(|| {
        let mut rr = RoundRobin::new(vec![1, 2]);
        assert_eq!(*rr.next_value(), 1);
        rr.replace_values(vec![]);
        let _v = *rr.next_value(); // should panic
    });
    assert!(result.is_err());
}
