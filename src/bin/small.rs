fn main() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = values
        .windows(4)
        .find(|window| window == &[3, 4, 5, 6])
        .is_some();
    dbg!(result);
}
