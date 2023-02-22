use itertools::Itertools;

fn main() {
    let integers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let i_slice = &integers[..];
    let integer_result = i_slice.has_prefix(&[3, 4, 5]);
    dbg!(integer_result);

    let floats = vec![1.0, 4.0, 5.0, 7.0];
    let float_result =
        floats.as_slice().has_prefix(&[4.20, 5.0]);
    dbg!(float_result);
}

trait Prefix<T> {
    fn has_prefix(&self, prefix: &[T]) -> bool;
}

impl<T> Prefix<T> for &[T]
where
    T: PartialEq,
{
    fn has_prefix(&self, prefix: &[T]) -> bool {
        self.iter()
            .positions(|v| *v == prefix[0])
            .find(|&index| {
                let range = index..(index + prefix.len());
                self[range] == *prefix
            })
            .is_some()
    }
}
