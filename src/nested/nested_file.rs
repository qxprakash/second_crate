#[docif::export]
pub fn nested_example() {
    assert_eq!(3 * 3, 9);
    println!("Running from nested_file in second_crate!");
}