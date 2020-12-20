///
/// Shell Sort
///
/// ```
/// # use exercises::shell_sort;
///
/// let input = vec![1,3,5,7,9,8,6,4,2];
/// let output = shell_sort::sort(&input);
/// assert_eq!(vec![1,2,3,4,5,6,7,8,9], output);
/// ```
///
/// Note: we sort elements of type u8 as a placeholder. We could just as easily sort elements of
/// type T where T implements PartialOrd (or something like that), but for simplicity we're using
/// a concrete integer type.
///
pub fn sort(input: &Vec<u8>) -> Vec<u8> {
    todo!()
}

#[test]
fn test_shell_sort() {
    use rand::prelude::*;

    let mut rng = thread_rng();

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);

    let input = (1..256).map(|_| rng.sample(distr)).collect::<Vec<u8>>();

    let output = sort(&input);

    let mut sorted = input.clone();
    sorted.sort();

    assert_eq!(output, sorted);
}
