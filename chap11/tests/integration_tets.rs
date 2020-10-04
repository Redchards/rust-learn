use chap11;

// Each file in the 'tests' directory is compiled as it's own separate crate!

#[test]
fn it_adds_two() {
    assert_eq!(4, chap11::add_two(2));
}