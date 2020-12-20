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
/// We are ignoring a lot of easy performance gains, for example:
///
/// * Sorting `input` in place. We duplicate it and return a whole new vector. This is memory
///   inefficient at the very least.
///
pub fn sort(input: &Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = input.clone();

    // Initialize `gap` to be length of the input array.
    // This immediately gets cut to ~1/3 for faster comparison.
    // We cannot initialize it to 1/3 outside of the loop because of the `while` comparison.
    let mut gap = output.len();

    while gap != 1 {
        // We start with a gap of len/3 + 1
        // Tutorials/psuedocode complicate this step...
        // I'm still not sure why we cannot statically start with this gap.
        // And that makes me nervous.
        gap = gap / 3 + 1;

        println!("DEBUG gap: {}", gap);

        for index in gap..output.len() {
            println!("DEBUG {:?}", output);
            // `a` is the left-most value in our tuple comparison
            let a = index-gap;
            // `b` is our right-most value in our tuple comparison
            let b = index;
            println!("DEBUG comparing output[{}] ({}) > output[{}] ({})", a, output[a], b, output[b]);
            // If the left value is bigger, swap them
            if output[a] > output[b] {
                println!("DEBUG Swapping output[{}] {} and output[{}] {}", a, output[a], b, output[b]);
                output.swap(a, b);
            }
        }
        println!("DEBUG After iteration 1: {:?}", output);
    }

    output
}

#[test]
fn test_shell_sort() {
    use rand::prelude::*;

    let mut rng = thread_rng();

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);

    let input = (1..10).map(|_| rng.sample(distr)).collect::<Vec<u8>>();
    println!("DEBUG input: {:?}", input);

    let output = sort(&input);
    println!("DEBUG input: {:?}", output);

    let mut sorted = input.clone();
    sorted.sort();

    assert_eq!(output, sorted);
}
