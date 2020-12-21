///
/// Insertion Sort
///
/// ```
/// # use exercises::insertion_sort;
///
/// let mut input = vec![1,3,5,7,9,8,6,4,2];
/// insertion_sort::sort(&mut input);
/// assert_eq!(vec![1,2,3,4,5,6,7,8,9], input);
/// ```
///
/// This implementation is a bit verbose and does not make performance or language ergenomic
/// improvements. Explicit and verbose for the sake of understanding what the algorithm is doing.
///
pub fn sort(input: &mut Vec<u8>) {
    // Work our way from the start of the list to the end
    for outer in 0..input.len() {
        // Initialize variables for source and destination
        // Our plan is for each value in the vec,
        // determine where it should go and then move it there
        let (mut to, from) = (outer, outer);

        // Note that if the inner loop finds that the current element is appropriately placed, that
        // is to say it is the highest valued element up to this point, it should not (and does
        // not) move.

        // From our current pointer work back to the start of the list
        for inner in (0..outer).rev() {
            // If our current value is greater than the one we're looking at
            // We have to iterate another step
            if input[inner] > input[outer] {
                to = inner;
            // If the [inner] value is smaller, we can stop looping
            } else {
                break;
            }
        }

        // If we found a new index to put our value at
        if to != from {
            // Remove it from it's current location
            let val = input.remove(from);

            // Insert it at the new location
            input.insert(to, val);
        }
    }
}

#[test]
fn test_insertion_sort() {
    use rand::prelude::*;

    let mut rng = thread_rng();

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);

    let mut input = (1..1024).map(|_| rng.sample(distr)).collect::<Vec<u8>>();

    let mut sorted = input.clone();
    sorted.sort();

    sort(&mut input);

    assert_eq!(input, sorted);
}
