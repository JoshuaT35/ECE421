#[derive(Debug)] 
struct TreeNode<'a> { 
    data: &'a str, 
    left_child: Option<Box<TreeNode<'a>>>, 
    right_child: Option<Box<TreeNode<'a>>>, 
} 

impl<'a> TreeNode<'a> {
    pub fn insert_node(&mut self, data: &'a str) {
        // do nothing if the data is the same
        if self.data == data {
            return;
        };

        // get the node to put data into
        let child_node = if data < self.data {
            &mut self.left_child
        }
        else {
            &mut self.right_child
        };
    
        // insert data
        match child_node {
            // None
            None => {
                // create new Node
                *child_node = Some(Box::new(TreeNode {
                    data: data,
                    left_child: None,
                    right_child: None,
                }));
            }
            // node already exists
            Some(child_node) => {
                child_node.insert_node(data);
            }
        }
    }
}

fn main() {
    let mut tree = TreeNode {
        data: "5",
        left_child: None,
        right_child: None,
    };

    tree.insert_node("3");
    tree.insert_node("2");
    tree.insert_node("4");
    tree.insert_node("7");
    tree.insert_node("6");
    tree.insert_node("8");

    println!("{:#?}", &tree);
}
