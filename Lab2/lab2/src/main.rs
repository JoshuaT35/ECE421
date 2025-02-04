#[derive(Debug)] 
struct TreeNode { 
    data: &str, 
    left_child: Option<TreeNode>, 
    right_child: Option<TreeNode>, 
} 

fn main() {
    let nodeExample = TreeNode {
        data: "example",
        left_child: None,
        right_child: None,
    };
}