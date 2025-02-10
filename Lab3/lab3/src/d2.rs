#[derive(Debug)] 
struct TreeNode<'a> { 
    data: &'a str, 
    left_child: Option<Box<TreeNode<'a>>>, 
    right_child: Option<Box<TreeNode<'a>>>, 
} 

// question 2 main
fn main() {
    let nodeExample = TreeNode {
        data: "example",
        left_child: None,
        right_child: None,
    };
}