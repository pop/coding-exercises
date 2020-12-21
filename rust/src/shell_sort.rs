///
/// Shell Sort
///
/// ```
/// # use exercises::shell_sort;
///
/// let mut input = vec![1,3,5,7,9,8,6,4,2];
/// shell_sort::sort(&mut input);
/// assert_eq!(vec![1,2,3,4,5,6,7,8,9], input);
/// ```
///
/// Note: we sort elements of type u8 as a placeholder. We could just as easily sort elements of
/// type T where T implements PartialOrd (or something like that), but for simplicity we're using
/// a concrete integer type.
///
/// Following thes resources:
/// * https://www.tutorialspoint.com/data_structures_algorithms/shell_sort_algorithm.htm
/// * https://www.toptal.com/developers/sorting-algorithms/shell-sort
///
/// The algorithm goes like this:
/// Step 1 − Initialize the value of h
/// Step 2 − Divide the list into smaller sub-list of equal interval h
/// Step 3 − Sort these sub-lists using insertion sort
/// Step 3 − Repeat until complete list is sorted
///
/// Wikipedia has a good high-level description:
///
/// > The method starts by sorting pairs of elements far apart from each other, then progressively
/// > reducing the gap between elements to be compared. By starting with far apart elements, it can
/// > move some out-of-place elements into position faster than a simple nearest neighbor exchange.
///
pub fn sort(input: &mut Vec<u8>) {
    // Initialize `gap` to be length of the input array.
    // This immediately gets cut to ~1/3 for faster comparison.
    // We cannot initialize it to 1/3 outside of the loop because of the `while` comparison.
    let mut gap = input.len();

    while gap != 1 {
        // We start with a gap of len/3 + 1
        // Tutorials/psuedocode complicate this step...
        // I'm still not sure why we cannot statically start with this gap.
        // And that makes me nervous.
        gap = std::cmp::max(gap/2, 1);

        for index in gap..input.len() {
            // `a` is the left-most value in our tuple comparison
            let a = index-gap;
            // `b` is our right-most value in our tuple comparison
            let b = index;

            // If the left value is bigger, swap them
            if input[a] > input[b] {
                input.swap(a, b);
            }
        }
    }

    crate::insertion_sort::sort(input);
}

#[test]
fn test_shell_sort() {
    use rand::prelude::*;

    let mut rng = thread_rng();

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);

    let mut input = (1..10).map(|_| rng.sample(distr)).collect::<Vec<u8>>();

    let mut sorted = input.clone();
    sorted.sort();

    sort(&mut input);

    assert_eq!(input, sorted);
}
