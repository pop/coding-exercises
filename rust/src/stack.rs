/// Stacks
///
/// Our goals with this exercise are...
/// * Implement three stacks with one vec. [TriStack]
/// * Design a stack which has Push, Pop, and Min, all of which operate in O(1). [ConstStack]
/// * Implement a Queue using two Stacks. [TwoStackQueue]
/// * Sort a stack using at most one additional stack. [StackSort]
///
/// NOTE: All of these implementations operate on `usize` elements.
/// They could just as easily operate on an arbitrary T, but we operate on usize for simplicity.
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
/// NOTE: I start with an API discussion to nail down our interface so I can write tests
/// immediately. Once the API is stable-ish we can write tests and start our Test driven
/// development implementation.
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

        self.vec[index] = Some(val);
    }

    pub fn pop(&mut self, stack: StackChoice) -> Option<usize> {
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

///
/// ConstStack
///
/// > Implement a stack with push, pop, and min, all of which operate in O(1).
///
/// Note that I was tempted to implement this with slices and arrays instead of Vecs.
/// I was hoping to do that to (A) limit the helper methods I could use and (B) hopefully get some
/// memory gains.
/// This ended up being tricky and I gave up on it because of either ownership problems, which are
/// manageable, and growing memory ownership problems, which I'm still not comfortable with. Plus
/// by the time I got something working I lost all of the memory benefits of Vec (I assume) are in
/// the stdlib.
///
/// Here's the problem with using slices instead of vecs.
///
/// Let's say we're in `push` and we want to add an element to our stack:
///
/// ```compile_fail
/// pub fn push(&mut self, val: usize) {
///     self.list = ...? // What goes here?
/// }
/// ```
///
/// You might think we can concat our current slice with a new slice of just our val.
/// That looks like this and it suffers from temporary ownership -- the memory allocated for the
/// new `self.list` is restricted to the local scope so it can't be assigned to `self.list`.
///
/// ```compile_fail
/// pub fn push(&mut self, val: usize) {
///     let new: [usize; 1] = [val]; // freed after function scope
///     self.list = &[self.list, new[..]].concat();
/// }
/// ```
///
/// Great you're thinking. If scope is your issue, just use a `Box`.
/// Ah, but a Box needs to know the size of it's arguments and a slice by definition does not know
/// it's size at compile time.
///
/// ```compile_fail
/// pub fn push(&mut self, val: usize) {
///     let new_val: [usize; 1] = [val]; // "all function arguments must have a statically known size"
///     let new_list = [self.list, Box::new(new_val[..])].concat();
/// }
/// ```
///
/// No matter how hard we try here, (A) Box requires static sized inputs and (B) we cannot
/// pass pointers to the Box because any memory we allocate in `push` is freed, therefore we
/// cannot use Box<Slice> nor Box<&Slice> for this solution.
///
/// Which leaves us with a few other options.
/// We can either use a Vec, which is definitely the easy way out (it even has `push` and `pop`
/// built in) or we can use some other structure like a linked list.
///
/// To make this exercise challenging we're doing our own bookkeeping for `min` value and `top`
/// value (indexes in the array) so we can do things in constant time.
///
/// Now we could make this whole structure a linked list where we have a structure that stores it's
/// value, it's min, and it's next value in a Option<Box>.
///
/// Push and Pop are as trivial as working on a linked list, easier even because we're just working
/// on the tail of the list.
///
/// The downside is of course that we are working heavily on the heap which has poorer performance
/// compared with the stack, but I'm sure L1-L3 cache will be enough to compensate for that...
/// maybe... (TODO: Benchmarking).
///
/// So I ended up going with a linked-list approach to this problem.
/// This ends up being similar to [A Bad
/// Stack](https://rust-unofficial.github.io/too-many-lists/first.html) from "Learn Rust With
/// Entirely Too Many Linked Lists" -- which I only realized after getting half way through my
/// implementation... 😅
///
/// NOTE: There are plenty of short-cuts I could have used for this exercise, namely using a
/// LinkedList stdlib data type or a Vec, but I _chose_ to do this on harder mode to learn
/// something about Rust. I'm still wading my way through Rust to get down to the deep memory
/// management bits. The first step in mastering something like Rust is to avoid going the easy way
/// out at every turn and start doing things the hard way; then when you come up for air you'll
/// have much better mastery over the easy stuff. That's the plan at least...
///
#[derive(Debug, Clone)]
pub struct ConstStack {
    val: Option<usize>,     // Value at top of stack
    min: Option<usize>,     // Minimum value of stack
    next: Option<Box<ConstStack>>, // Next value in stack
}

impl<'a> ConstStack {
    pub fn new() -> ConstStack {
        ConstStack {
            val: None,
            min: None,
            next: None,
        }
    }

    ///
    /// Push a new value onto the stack.
    ///
    /// We use a linked-list so this involves updating the "top" node and pushing current values to
    /// the 'next' node.
    ///
    pub fn push(&mut self, val: usize) {
        // Create copy of self and push that under self into linked list
        // We do this before modifying the head of the stack because we are about to over-write our
        // self.* values.
        self.next = match self.val {
            Some(_) => {
                let mut push = ConstStack::new();
                push.val = self.val;
                push.min = self.min;
                push.next = self.next.take();
                Some(Box::from(push))
            },
            None => None
        };

        // Update self values to reflect the new stack state
        self.min = match self.min {
            Some(cur) if cur > val => Some(val),
            None                   => Some(val),
            _                      => self.min ,
        };

        self.val = Some(val);
   }

    pub fn pop(&mut self) -> Option<usize> {
        let val = self.val;

        match &mut self.next {
            Some(pull) => {
                self.val = pull.val;
                self.min = pull.min;
                self.next = pull.next.take();
            },
            None => {
                self.val = None;
                self.min = None;
                self.next = None;
            },
        }

        val
    }

    pub fn min(&self) -> Option<usize> {
        self.min
    }
}

#[test]
fn test_const_stack() {
    // TODO: Need benchmarks to verify O(1) complexity
    let mut o1 = ConstStack::new();
    o1.push(2);
    o1.push(1);
    o1.push(3);

    assert_eq!(o1.min(), Some(1));

    assert_eq!(o1.pop(), Some(3));

    assert_eq!(o1.pop(), Some(1));

    assert_eq!(o1.min(), Some(2));

    assert_eq!(o1.pop(), Some(2));

    assert_eq!(o1.pop(), None);

    assert_eq!(o1.min(), None);

    o1.push(100);

    assert_eq!(o1.min(), Some(100));

    o1.push(50);
    o1.push(101);
    o1.push(102);
    o1.push(103);

    assert_eq!(o1.min(), Some(50));
}

/// 
/// TwoStackQueue
///
/// > Implement a Queue using two Stacks.
///
/// API Discussion:
///
/// We want a queue, first in first out, so we want an interface like this:
///
/// ```
/// # use exercises::stack::TwoStackQueue;
///
/// let mut q = TwoStackQueue::new();
///
/// q.push(1);
/// q.push(2);
/// q.push(3);
///
/// assert_eq!(q.pop(), Some(1));
/// assert_eq!(q.pop(), Some(2));
/// assert_eq!(q.pop(), Some(3));
/// assert_eq!(q.pop(), None);
/// ```
///
/// Should be easy enough.
///
/// Implementation Discussion:
///
/// Using only two stacks, we could use one of the above stacks but let's not.
/// We'll just use two Vecs and implement a Push and Pop method.
///
/// So how do we use two Stacks to create a queue? Let's brainstorm:
///
/// * Let's say we have two stacks (A) and (B).
///
/// * If we just store everything the user `push`-es onto the queue in (A).
///   (A) has all user input, the question becomes how do we pop elements off a queue-fashion?
///
/// * When the user `pop`-s something off the queue we can reverse (A) onto (B), pop desired
/// element off, and then un-reverse the stack onto the original stack.
///
/// * This is computationally inefficient, but the problem didn't specify any space of
/// computational complexity restrictions, so it is a valid solution.
///
/// Here's some pseudocode for the above solution:
///
/// ```ignore
/// fn push(&mut self, val: usize) {
///     push val onto self.a_stack
/// }
///
/// fn pop(&mut self) -> usize {
///     reverse self.a_stack into self.b_stack
///
///     pop element off of self.b_stack
///
///     reverse self.b_stack onto self.a_stack
///
///     return popped element
/// }
/// ```
///
/// Note that this is computationally complex. We are pushing and popping a _lot_ and for large
/// stacks this can become a burden. The question didn't ask for an _efficient_ algorithm, it asked
/// for an algorithm. The complexity of this implementation's `pop` is roughly O(n) where `n` is
/// the size of the queue, and `push` is O(1) or constant time. Technically `push` is O(2n), but we
/// collapse the 2 because we don't care about constant multipliers just weather this is linear,
/// sub-linear, or exponential.
///
pub struct TwoStackQueue {
    a: Vec<usize>,
    b: Vec<usize>,
}

impl TwoStackQueue {
    pub fn new() -> TwoStackQueue {
        TwoStackQueue {
            a: Vec::new(),
            b: Vec::new(),
        }
    }

    pub fn push(&mut self, val: usize) {
        // Push is dead simple as we are leveraging `Vec`-s use as a stack.
        self.a.push(val);
    }

    pub fn pop(&mut self) -> Option<usize> {
        // Drain the A stack onto B
        while let Some(element) = self.a.pop() {
            self.b.push(element);
        }

        // Pop the first-in element from B
        // We get the safety that our `pop` will always return `None` on an empty stack because Vec
        // always returns None on an empty stack or Some for all other cases.
        let ret = self.b.pop();

        // Drain B back onto A, giving us almost the original stack
        while let Some(element) = self.b.pop() {
            self.a.push(element);
        }

        ret
    }
}

#[test]
fn test_two_stack_queue() {
    let mut q = TwoStackQueue::new();

    assert_eq!(q.pop(), None);

    q.push(1);
    q.push(2);
    q.push(1);
    q.push(3);
    q.push(2);
    q.push(3);

    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), Some(3));
    assert_eq!(q.pop(), Some(2));
    assert_eq!(q.pop(), Some(3));
    assert_eq!(q.pop(), None);
    assert_eq!(q.pop(), None);

    q.push(4);
    q.push(5);

    assert_eq!(q.pop(), Some(4));
    assert_eq!(q.pop(), Some(5));
    assert_eq!(q.pop(), None);
}

/// 
/// SortStack
///
/// > Sort a stack using at most one additional stack.
/// > The stacks support push, pop, peek, and isEmpty.
///
/// For this we are goig to implement a simple stack that wraps `Vec` and includes a `sort` method.
/// That `sort` will be where all the interesting solution stuff happens, so we're going to focus
/// on that part of our implementation.
///
/// Knowing nothing about sorting a stack with a second stack, I'm going to need to brainstorm a
/// bit:
///
/// * We have two stacks (duh).
/// * We can only compare the top of these two stacks.
/// * I think we need at least one temporary variable, in addition to the two stacks -- maybe that
///   was obvious...?
/// * With only two stacks all we can do is "pour" values from on stack to the other.
/// * (I did some research and that assumption about needing a temp variable is correct)
/// * Given that we can and should use a helper variable, the algorithm becomes pretty easy:
///     * Designate a temporary stack as the "sorted" stack
///     * Take a value off of the main stack
///     * Save that value to a variable
///     * Compare it to the top of the temp stack
///     * If it is greater (or less, however we are sorting) pop the temp stack onto the main stack
///     * Compare again and pop/push until the temp value is less (or greater, however we are
///       sorting)
///     * Pop values off of the main stack onto the temp stack as long as temp stack stays sorted.
///
/// Look at the docs for the `StackSort::sort()` method for more information about the
/// implementation.
///
pub struct StackSort {
    list: Vec<usize>,
    temp: Vec<usize>,
}

impl StackSort {
    pub fn new() -> StackSort {
        StackSort {
            list: Vec::new(),
            temp: Vec::new(),
        }
    }

    pub fn push(&mut self, val: usize) {
        self.list.push(val);
    }

    fn push_temp(&mut self, val: usize) {
        self.temp.push(val);
    }

    pub fn pop(&mut self) -> Option<usize> {
        self.list.pop()
    }

    fn pop_temp(&mut self) -> Option<usize> {
        self.temp.pop()
    }

    pub fn peek(&self) -> Option<usize> {
        match self.list.last() {
            Some(&v) => Some(v),
            None => None,
        }
    }

    fn peek_temp(&self) -> Option<usize> {
        match self.temp.last() {
            Some(&v) => Some(v),
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    ///
    /// The jist of our algorithm is that we have a `temp` stack that will be sorted in ascending
    /// order (lower values on top).
    ///
    /// We also have our `list` which is our input and where we eventually dump our `temp` onto so
    /// we have a finalized `sorted` list which is in descending (higher values on top) order.
    ///
    /// 1. We pop a value off of `list` into a temporary variable. This is what we are trying to
    ///    sort.
    /// 2. Then we pop values off of `temp` (which should always be sorted) until we find where our
    ///    temp value should go.
    /// 3. Pop elements off of `list` onto `temp` until it stops being sorted.
    ///
    /// NOTE: This function is _long_. It could be shorter, by splitting out some functionality
    /// into smaller methods, but I feel it wouldn't read as well. Of course when collaborating
    /// with others on a codebase I follow the team's direction, but for this I figured "I can read
    /// this cover to cover and it makes sense. Splitting this out into smaller methods would
    /// obscure that legibility".
    ///
    /// Also note that this sorting algorithm is not computationally efficient. It is constantly
    /// pushing and popping things from one stack to the other, but given the constraints of the
    /// problem I think it's roughly as complex as it needs to be. As for space complexity, it uses
    /// roughly constant space _total_ between both stacks and a few variables for bookkeeping.
    ///
    pub fn sort(&mut self) {
        // While !self.is_empty()
        //     If self.peek() < self.temp.peek()
        //         self.list.pop() -> self.temp.push(val)
        //     Else
        //         val = self.pop()
        //         While val > self.temp.peek()
        //             self.temp.pop() -> self.push()
        //         self.temp.push(val)
        //  Reverse self.temp into self.list
        while !self.is_empty() {
            // Determine if our stack has elements to sort, it should at this point
            match self.peek() {
                // The stack contains _something_ so we process that value
                Some(top) => {
                    // Compare the top of our stack with the top of our temp stack
                    match self.peek_temp() {
                        // The temp stack is already populated, so we do some comparisons
                        // NOTE: the temp stack should _always_ be sorted
                        Some(tmp) => {
                            // We should be able to unwrap this safely as we aleady verified this
                            // is a Some(value) not a None when we called `peek()`.
                            let val = self.pop().unwrap();

                            // If the top value on our stack is less than the temp stack's top, we
                            // transfer the value from the stack to the temp stack
                            if val < tmp {
                                self.push_temp(val);
                            // Our current value should not go onto the sorted stack, so we have to
                            // do shuffling of bits
                            } else {
                                // First, pop values off of `temp` onto `list` until we find a spot
                                // that our value fits, or we hit bedrock (an empty stack).
                                while Some(val) >= self.peek_temp() {
                                    match self.pop_temp() {
                                        Some(t) => self.push(t),
                                        None => break,
                                    }
                                }

                                // Push our value onto the stack
                                self.push_temp(val);

                                // Now pop values off of `list` onto `temp` as long as temp stays
                                // sorted (that's not the literal logic we use, but that's
                                // effectively what we're doing here).
                                while Some(val) <= self.peek() {
                                    match self.pop() {
                                        Some(t) => self.push_temp(t),
                                        None => break,
                                    }
                                }
                            }
                        },
                        // The temp stack is empty, so our `top` value must be the lowest value
                        // we've seen this far
                        None => {
                            self.pop();
                            self.push_temp(top);
                        },
                    }
                },
                // Our stack is empty, this should have ended the `while`, so this code path is
                // probably never hit, but we cover this `match` case for correctness.
                None => {
                    break;
                },
            }
        }

        // Reverse the `temp` stack back onto `list` so we end up with a descending ordered stack
        while let Some(val) = self.temp.pop() {
            self.list.push(val);
        }
    }
}

#[test]
fn test_stack_sort() {
    let mut s = StackSort::new();

    // Check our methods work with an empty stack
    assert_eq!(s.is_empty(), true);
    assert_eq!(s.peek(), None);
    assert_eq!(s.pop(), None);

    s.push(1);

    // Check is_empty and peek work
    assert_eq!(s.is_empty(), false);
    assert_eq!(s.peek(), Some(1));

    // Populate the stack with _unsorted_ elements
    s.push(2);
    s.push(6);
    s.push(4);
    s.push(3);
    s.push(5);

    // Double check the stack has not been tampered with
    assert_eq!(s.peek(), Some(5));

    // Make the call to sort the stack
    s.sort();

    // Double check the stack has been modified
    assert_eq!(s.peek(), Some(6));

    // Assert the list was sorted by popping all elements from the stack
    // We exect it to be sorted in descending order
    assert_eq!(s.pop(), Some(6));
    assert_eq!(s.pop(), Some(5));
    assert_eq!(s.pop(), Some(4));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}
