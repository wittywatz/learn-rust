use cargo::adder;

#[test]
fn integration_test_for_add() {
    assert_eq!(adder::add(5, 7), 12);
}
