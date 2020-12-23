/// Stacks
///
/// Our goals with this exercise are...
/// * Implement three stacks with one vec.
/// * Design a stack which has Push, Pop, and Min, all of which operate in O(1).
/// * Implement a Queue using two Stacks.
/// * Sort a stack using at most one additional stack.
///


///
/// TriStack
///
/// > Implement three stacks with one vec.
///
/// Here we implement a stack of `usize` elements.
///
/// We only implement `push` and `pop` interfaces.
///
/// ```
/// # use exercises::stack::{TriStack, StackChoice};
///
/// let mut s = TriStack::new();
///
/// s.push(StackChoice::First, 1);
/// s.push(StackChoice::Second, 3);
/// s.push(StackChoice::Third, 7);
/// s.push(StackChoice::First, 2);
/// s.push(StackChoice::Second, 4);
/// s.push(StackChoice::Third, 8);
///
/// assert_eq!(s.pop(StackChoice::First), Some(2));
/// assert_eq!(s.pop(StackChoice::First), Some(1));
/// assert_eq!(s.pop(StackChoice::First), None);
///
/// assert_eq!(s.pop(StackChoice::Second), Some(4));
/// assert_eq!(s.pop(StackChoice::Second), Some(3));
/// assert_eq!(s.pop(StackChoice::Second), None);
///
/// assert_eq!(s.pop(StackChoice::Third), Some(8));
/// assert_eq!(s.pop(StackChoice::Third), Some(7));
/// assert_eq!(s.pop(StackChoice::Third), None);
/// ```
///
/// API Discussion:
///
/// There are a few API considerations to make.
/// We can either have one `push` and `pop` method, each of which accepts some indicator of which
/// stack to manipulate.
/// We could also have three separate push and pop methods, each of which acts to the user like a
/// separate. To avoid copious code duplication, these would probably wrap a generic push and pop
/// that has most of the complexity.
///
/// I decided that having one generic method that the user interacts with, while not especially
/// clean, it does hint at the implementation, is scalable to more than three stacks and requires
/// me to implement logic that I'd probably need anyway. So it's not the _best_ API, but it is sort
/// of a min-max between clean API and clean implementation.
///
/// Implementation Discussion:
///
/// There's a few ways to implement three stacks in one vec.
///
/// The first I came up with is to store each element in every third position starting at some
/// base index. For example, inserting into stack 2 would populate index 1, 4, 7, ... and inserting
/// into stack 3 would populate index 2, 5, 8, ...
///
/// This is space inefficient and has a worst case scenario where you only use the first stack and
/// your vec is three times too big.
///
/// Next, we could store things tightly with tuples to (stack choice, value). This is more
/// efficient space wise, but makes it hard to find values with a naive linear search. It is a
/// stack, but if you use the second stack once and then only the third stack, to the pop a value
/// off of the first stack would be inefficient.
///
/// Our struct could keep track of the HEAD of each stack, that is part of our TriStack could keep
/// an index of the top of stacks 1, 2, and 3. This might help initially popping values off the
/// stack, but then to re-calculate that value we need to do the searching I was worried about.
///
/// We could do some sort of bucket system where the first N indexes are reserved for stack 1, the
/// next N are for stack 2, etc. This makes searching a little easier and for stacks that are doing
/// a lot of pushing and popping this would probably be optimal. It does take up a lot of space and
/// in the worst case, where only one stack is in use, we have a vec that is 3x too big.
///
/// I'm inclined then to make the decision that we assume the average case where each stack is in
/// roughly the same use. That is to say, we don't code for the worst case where one stack is used
/// way more or less than the others. All three stacks are in use at roughly the same pace.
///
/// Given this assumption the "every N'th index is for stack N" works and is simple to implement.
///
/// Of course if I were to put this in prod I would probably implement a few ideas, benchmark them
/// against real-world cases, and maybe even make the choice of algorithm a run-time decision.
///
pub struct TriStack {
    vec: Vec<Option<usize>>    
}

///
/// Simple Choice struct for selecting which stack to manipulate.
///
#[derive(Copy, Clone)]
pub enum StackChoice {
    First,
    Second,
    Third,
}

impl TriStack {
    pub fn new() -> TriStack {
        TriStack { vec: Vec::new() }
    }

    pub fn push(&mut self, stack: StackChoice, val: usize) {
        let index = match self.find_top_element(stack) {
                Some((pos, _)) => pos+3,
                None => match stack {
                    StackChoice::First  => 0,
                    StackChoice::Second => 1,
                    StackChoice::Third  => 2,
                },
        };

        if index >= self.vec.len() {
            self.vec.reserve(3);
            self.vec.push(None);
            self.vec.push(None);
            self.vec.push(None);
        }

        self.vec[index] = Some(val)
    }

    pub fn pop(&mut self, stack: StackChoice) -> Option<usize> {
        // TODO: I see a lot of duplicated code. Move out to method.
        let (index, element): (Option<usize>, &Option<usize>) = match self.find_top_element(stack) {
            Some((pos, element)) => (Some(pos), element),
            None => (None, &None)
        };

        let val = *element;

        match index {
            Some(idx) => {
                self.vec[idx] = None;
            },
            None => ()
        }

        val
    }

    ///
    /// Private API: find_top_element
    ///
    /// Helper method to find the index and value of the "top" of a given stack.
    ///
    /// Not FDA approved and not intended for human consumption.
    ///
    fn find_top_element(&self, stack: StackChoice) -> Option<(usize, &Option<usize>)> {
        match stack {
            StackChoice::First => {
                self.vec.iter()
                    .enumerate()
                    .filter(|(pos, _)| pos % 3 == 0)
                    .filter(|(_, element)| element.is_some())
                    .last()
            },
            StackChoice::Second => {
                self.vec.iter()
                    .enumerate()
                    .filter(|(pos, _)| pos % 3 == 1)
                    .filter(|(_, element)| element.is_some())
                    .last()
            },
            StackChoice::Third => {
                self.vec.iter()
                    .enumerate()
                    .filter(|(pos, _)| pos % 3 == 2)
                    .filter(|(_, element)| element.is_some())
                    .last()
            },
        }
    }
}

#[test]
fn test_tristack() {
    let mut tstack: TriStack = TriStack::new();

    tstack.push(StackChoice::First, 1);
    tstack.push(StackChoice::Second, 4);
    tstack.push(StackChoice::Third, 7);

    tstack.push(StackChoice::First, 2);
    assert_eq!(tstack.pop(StackChoice::First), Some(2));
    tstack.push(StackChoice::First, 3);

    tstack.push(StackChoice::Second, 5);
    assert_eq!(tstack.pop(StackChoice::Second), Some(5));
    tstack.push(StackChoice::Second, 6);

    tstack.push(StackChoice::Third, 8);
    assert_eq!(tstack.pop(StackChoice::Third), Some(8));
    tstack.push(StackChoice::Third, 9);

    assert_eq!(tstack.pop(StackChoice::Third), Some(9));
    assert_eq!(tstack.pop(StackChoice::Third), Some(7));
    assert_eq!(tstack.pop(StackChoice::Third), None);

    assert_eq!(tstack.pop(StackChoice::Second), Some(6));
    assert_eq!(tstack.pop(StackChoice::Second), Some(4));
    assert_eq!(tstack.pop(StackChoice::Second), None);

    assert_eq!(tstack.pop(StackChoice::First), Some(3));
    assert_eq!(tstack.pop(StackChoice::First), Some(1));
    assert_eq!(tstack.pop(StackChoice::First), None);
}

#[test]
fn test_find_top_element() {
    let mut t = TriStack { vec: vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)] };

    let top_1 = t.find_top_element(StackChoice::First);
    let top_2 = t.find_top_element(StackChoice::Second);
    let top_3 = t.find_top_element(StackChoice::Third);

    assert_eq!(top_1, Some((3, &Some(4))));
    assert_eq!(top_2, Some((4, &Some(5))));
    assert_eq!(top_3, Some((5, &Some(6))));

    t.vec.pop();
    t.vec.pop();

    let top_1 = t.find_top_element(StackChoice::First);
    let top_2 = t.find_top_element(StackChoice::Second);
    let top_3 = t.find_top_element(StackChoice::Third);

    assert_eq!(top_1, Some((3, &Some(4))));
    assert_eq!(top_2, Some((1, &Some(2))));
    assert_eq!(top_3, Some((2, &Some(3))));
}
