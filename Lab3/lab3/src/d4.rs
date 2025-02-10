#[derive(Debug)]
enum Tree<T: Ord> {
    Node {
        data: T,
        left_child: Box<Tree<T>>,
        right_child: Box<Tree<T>>,
    },
    Empty,
}

fn insert_node<T: std::cmp::Ord>(tree: &mut Tree<T>, new_data: T) {
    match tree {
        Tree::Empty => {
            // make tree ta node
            *tree = Tree::Node{
                data: new_data,
                left_child: Box::new(Tree::Empty),
                right_child: Box::new(Tree::Empty),
            }
        },
        Tree::Node { data: tree_data, left_child: tree_left, right_child: tree_right} => {
            // get the child
            let tree_child = if &new_data < tree_data {
                tree_left
            }
            else {
                tree_right
            };

            // insert into the child
            insert_node(tree_child, new_data);
        }
    }
}

fn main() {
    let mut tree = Tree::Node{
        data: 5,
        left_child: Box::new(Tree::Empty),
        right_child: Box::new(Tree::Empty),
    };

    insert_node(&mut tree, 3);
    insert_node(&mut tree, 2);
    insert_node(&mut tree, 4);
    insert_node(&mut tree, 7);
    insert_node(&mut tree, 6);
    insert_node(&mut tree, 8);

    println!("{:#?}", &tree);
}