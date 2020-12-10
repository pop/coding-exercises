use exercises;
use text_io::read;

fn main() {
    println!("Creating a binary tree");
    println!("Please enter an integer");
    let input: isize = read!();
    let mut the_tree = exercises::btree::BTree::new(input);
    println!("Press Ctrl+C to quit modifying tree");
    loop {
        println!("{:#?}", the_tree);
        println!("Depth: {} | Balanced: {}", the_tree.depth(), the_tree.balanced());
        println!("Please enter an integer to add to the tree");
        let input: isize = read!();
        the_tree.insert(input);
    }
}
