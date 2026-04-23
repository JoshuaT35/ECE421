use super::node::Node;
use super::node::*;

pub trait BaseTree<T> {
    type MNode: Node<T>;
    fn get(&self, val: usize) -> &Self::MNode;
    fn get_mut(&self, val: usize) -> &mut Self::MNode;

    fn delete_node(&mut self, index: usize);
    fn create_node(&mut self, val: T) -> usize;

    fn rebalance_ins(&mut self, n: usize);
    fn rebalance_del(&mut self, n: usize, child: usize);

    fn delete(&mut self, val: T) -> bool;
    fn delete_replace(&mut self, n: usize) -> usize;

    fn attach_child(&self, p: usize, c: usize, side: Side);

    fn get_root(&self) -> Option<usize>;
    fn set_root(&mut self, new_root: Option<usize>);
}

pub trait Tree<T: std::fmt::Debug>: BaseTree<T> {
    fn new() -> Self;

    fn is_empty(&self) -> bool {
        return self.get_root().is_none()
    }

    fn contains(&self, val: &T) -> bool {
        let n = self.find(val);
        if n == usize::MAX {
            return false;
        }
        self.get(n).is(val)
    }

    fn insert(&mut self, val: T) {
        if let Some(root) = self.get_root() {
            let n = self.find(&val);
            let node = self.get(n);
    
            let side = if node.lesser(&val) {
                Side::Right
            } else if node.greater(&val) {
                Side::Left
            } else {    
                // Value is equal to node, place duplicates in right subtree
                Side::Right
            };
    
            if let Some(child) = node.get_child(side) {
                // Recursively insert into the child via self
                self.insert_at(child, val);
            } else {
                // Otherwise, create and attach a new node
                let new_node = self.create_node(val);
                self.attach_child(n, new_node, side);
                self.rebalance_ins(new_node);
            }
        } else {
            // If the tree is empty, create and set the root
            let r = Some(self.create_node(val));
            self.set_root(r);
        }
    }
    
    fn insert_at(&mut self, child_node: usize, val: T) {
        let node = self.get(child_node);
        let side = if node.lesser(&val) { Side::Right } else { Side::Left };
    
        if let Some(child) = node.get_child(side) {
            self.insert_at(child, val); // Recurse
        } else {
            let new_node = self.create_node(val);
            self.attach_child(child_node, new_node, side);
            self.rebalance_ins(new_node);
        }
    }
    



    fn rotate(&mut self, side: Side, n: usize) {
        let p = self.get(n).get_parent().expect("Parent rotate");

        if let Some(c) = self.get(n).get_child(side) {
            self.attach_child(p, c, !side);
        } else {
            match !side {
                Side::Left => self.get_mut(p).set_child(None, Side::Left),
                Side::Right => self.get_mut(p).set_child(None, Side::Right),
            }
        }
        if let Some(g) = self.get(p).get_parent() {
            self.get_mut(n).set_parent(Some(g));
            let pside = if self.get(p).is_child(Side::Left) {
                Side::Left
            } else {
                Side::Right
            };
            self.attach_child(g, n, pside);
        } else {
            self.set_root(Some(n));
            self.get_mut(n).set_parent(None);
        }
        self.attach_child(n, p, side);
    }

    fn find(&self, val: &T) -> usize {
        let mut n = self.get_root().unwrap_or(usize::MAX);
        if n == usize::MAX {
            return n;
        }
        loop {
            let node = self.get(n);
            if node.lesser(val) && node.get_child(Side::Right).is_some() {
                n = node.get_child(Side::Right).expect("find n right child");
            } else if node.greater(val) && node.get_child(Side::Left).is_some() {
                n = node.get_child(Side::Left).expect("find n left child");
            } else {
                return n;
            }
        }
    }

    fn get_height(&self) -> usize {
        if let Some(root) = self.get_root() {
            self.get(root).get_height()
        } else {
            0
        }
    }

    fn get_leaf_count(&self) -> usize {
        if let Some(root) = self.get_root() {
            self.get(root).get_leaf_count()
        } else {
            0
        }
    }

    fn inorder_traversal(&self) {
        let mut result = Vec::new();
        if let Some(root) = self.get_root() {
            self.inorder_helper(root, &mut result);
        }
        let result_str: String = result.iter()
            .map(|&value| format!("{:?}", value)) 
            .collect::<Vec<String>>()
            .join(", ");

        println!("{}\n",result_str);
    }

    fn inorder_helper<'a>(&'a self, node_index: usize, result: &mut Vec<&'a T>) {
        let node = self.get(node_index);
        
        // Traverse the left subtree
        if let Some(left_child) = node.get_child(Side::Left) {
            self.inorder_helper(left_child, result);
        }

        // Visit the root node
        result.push(node.get_value());

        // Traverse the right subtree
        if let Some(right_child) = node.get_child(Side::Right) {
            self.inorder_helper(right_child, result);
        }
    }
}