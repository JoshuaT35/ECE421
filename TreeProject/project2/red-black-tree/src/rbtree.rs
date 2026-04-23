use std::cell::RefCell;
use std::rc::Rc;

use super::tree::BaseTree;
use super::tree::Tree;

use::std::io::{self, Write};
use::std::fmt::{Debug, Display};
use ptree::{TreeItem, Style};
use colored::Colorize;
use std::borrow::Cow;

use super::node::Node;
use super::node::*;

#[derive(Debug, Clone)]
pub struct ColorNode<T> {
    pub value: Option<T>,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    pub color: Color,
    data: Rc<RefCell<Vec<ColorNode<T>>>>,
}

impl<T> ColorNode<T>
where
    T: std::fmt::Debug,
    T: std::cmp::PartialOrd,
{
    fn new(val: T, selfptr: usize, data: Rc<RefCell<Vec<ColorNode<T>>>>) -> Self {
        Self {
            value: Some(val),
            ptr: selfptr,
            parent: None,
            lchild: None,
            rchild: None,
            color: Color::Black,
            data: data,
        }
    }

    fn is_red(&self) -> bool {
        match self.color {
            Color::Red => true,
            Color::Black => false,
        }
    }

    // Nil nodes are black children too
    fn is_child_black(&self, side: Side) -> bool {
        let child = self.get_child(side);
        if child.is_some() && self.get(child.unwrap()).is_red() {
            false
        } else {
            true
        }
    }

    // this will panic of called on root node
    fn is_parent_black(&self) -> bool {
        let p = self.parent.unwrap();
        !self.get(p).is_red()
    }

    // Nil nodes are black children too
    fn is_sibling_black(&self) -> bool {
        let sib = self.get_sibling();
        if sib.is_some() && self.get(sib.unwrap()).is_red() {
            false
        } else {
            true
        }
    }
}

impl<T: std::fmt::Debug + std::cmp::PartialOrd> Node<T> for ColorNode<T> {
    fn is(&self, val: &T) -> bool {
        match &self.value {
            Some(v) => v == val,
            None => false,
        }
    }

    fn greater(&self, val: &T) -> bool {
        match &self.value {
            Some(v) => v > val,
            None => false,
        }
    }

    fn lesser(&self, val: &T) -> bool {
        match &self.value {
            Some(v) => v < val,
            None => false,
        }
    }

    fn get_value(&self) -> &T {
        self.value.as_ref().expect("Value should not be None")
    }
    /*
    Currently uses raw pointers, need to support safe pointers (Rc and RefCell)
    */
    fn get(&self, ptr: usize) -> &ColorNode<T> {
        unsafe { &(*self.data.as_ptr())[ptr] }
    }

    fn get_mut(&self, ptr: usize) -> &mut ColorNode<T> {
        unsafe { &mut (*self.data.as_ptr())[ptr] }
    }

    fn get_child(&self, side: Side) -> Option<usize> {
        match side {
            Side::Left => self.lchild,
            Side::Right => self.rchild,
        }
    }

    fn set_child(&mut self, child: Option<usize>, side: Side) {
        match side {
            Side::Left => self.lchild = child,
            Side::Right => self.rchild = child,
        };
        if let Some(child) = child {
            self.get_mut(child).parent = Some(self.location());
        }
    }

    fn set_parent(&mut self, p: Option<usize>) {
        self.parent = p;
    }

    fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    fn location(&self) -> usize {
        self.ptr
    }
}

#[derive(Debug, Clone)]
pub struct RBTree<T> {
    root: Option<usize>,
    data: Rc<RefCell<Vec<ColorNode<T>>>>,
    free: Vec<usize>,
}

impl<T> Tree<T> for RBTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            root: None,
            data: Rc::new(RefCell::new(Vec::new())),
            free: Vec::new(),
        }
    }
}

const TREE_END: usize = 0xFFFFFFFF;
impl<T> BaseTree<T> for RBTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    type MNode = ColorNode<T>;
    /*
    CURRRENTLY USES UNSAFE POINTERS, NEEDS TO BE CHANGED
    */
    fn get(&self, val: usize) -> &Self::MNode {
        unsafe { &(*self.data.as_ptr())[val] }
    }

    fn get_mut(&self, val: usize) -> &mut Self::MNode {
        unsafe { &mut (*self.data.as_ptr())[val] }
    }

    fn get_root(&self) -> Option<usize> {
        self.root
    }

    fn set_root(&mut self, new_root: Option<usize>) {
        self.root = new_root
    }

    fn attach_child(&self, p: usize, c: usize, side: Side) {
        self.get_mut(p).set_child(Some(c), side)
    }

    fn rebalance_ins(&mut self, n: usize) {
        self.get_mut(n).color = Color::Red; // Inserted node is red by default
        if let Some(p) = self.get(n).parent {
            if self.get(p).is_red() { // Parent is red
                if let Some(u) = self.get(n).get_uncle() {
                    if self.get(u).is_red() { // Case 1: Uncle is red
                        self.get_mut(p).color = Color::Black; // Recolor parent and uncle
                        self.get_mut(u).color = Color::Black;
                        let gp = self.get(p).get_parent().expect("No grandparent");
                        self.get_mut(gp).color = Color::Red; // Recolor grandparent
                        self.rebalance_ins(gp); // Recurse up
                    } else { // Case 2: Uncle is black (or doesn't exist)
                        self.do_ins_hard_case(n);
                    }
                } else { // Case 3 (uncle is null)
                    self.do_ins_hard_case(n);
                }
            }
        }
        self.get_mut(self.root.unwrap()).color = Color::Black; // Ensure root is black
    }

    fn rebalance_del(&mut self, n: usize, child: usize) {
        if self.get(n).ptr == TREE_END || self.get(child).ptr == TREE_END {
            self.fix_del_color_long();
        } else {
            self.fix_del_color(n, child)
        }
    }

    fn delete(&mut self, val: T) -> bool {
        if !self.contains(&val) {
            return false;
        } else {
            let n = self.find(&val);
            let del = self.delete_replace(n);
            self.rebalance_del(del, n);
            self.delete_node(del);
            return true;
        }
    }

    fn delete_replace(&mut self, n: usize) -> usize {
        self.get_mut(n).ptr = TREE_END;
        n
    }

    fn create_node(&mut self, val: T) -> usize {
        let loc = RefCell::<Vec<ColorNode<T>>>::borrow(&self.data).len();
        self.data.borrow_mut().push(ColorNode::new(val, loc, self.data.clone()));
        loc
    }

    fn delete_node(&mut self, index: usize) {
        self.free.push(index);
    }
}

impl<T> RBTree<T>
where
    T: PartialOrd,
    T: PartialEq,
    T: std::fmt::Debug,
{
    // child is the new node in the location, n is being deleted
    fn fix_del_color(&mut self, n: usize, child: usize) {
        if !self.is_empty() {
            if !self.get(n).is_red() {
                if self.get(child).is_red() {
                    self.get_mut(child).color = Color::Black;
                } else {
                    self.delete_case_1(child);
                }
            }
        }
    }

    // sets a node to black if it exists
    fn set_maybe_black(&mut self, no: Option<usize>) {
        if let Some(n) = no {
            self.get_mut(n).color = Color::Black;
        }
    }

    fn delete_case_1(&mut self, n: usize) {
        let s = self.get(n).get_sibling();
        if self.get(n).is_sibling_black() {
            let p = self.get(n).parent.expect("D2 P");
            self.set_maybe_black(s);
            self.get_mut(p).color = Color::Red;
            self.rotate(self.get(n).side(), p);
        }
        self.delete_case_2(n);
    }

    fn delete_case_2(&mut self, n: usize) {
        let s = self.get(n).get_sibling().expect("D3 S");
        let p = self.get(n).parent.expect("D3 P");
        if self.get(n).is_parent_black()
            && !self.get(s).is_red()
            && self.get(s).is_child_black(Side::Left)
            && self.get(s).is_child_black(Side::Right)
        {
            self.delete_case_1(p);
        } else {
            self.delete_case_3(p);
        }
    }

    fn delete_case_3(&mut self, n: usize) {
        let node = self.get(n);
        let s = node.get_sibling().expect("D4 S");
        let p = node.parent.expect("D4 P");

        if !node.is_parent_black()
            && node.is_sibling_black()
            && self.get(s).is_child_black(Side::Left)
            && self.get(s).is_child_black(Side::Right)
        {
            self.get_mut(s).color = Color::Red;
            self.get_mut(p).color = Color::Black;
        } else {
            self.delete_case_4(n)
        }
    }

    fn delete_case_4(&mut self, n: usize) {
        let s = self.get(n).get_sibling().expect("D5 S");
        if !self.get(s).is_red() {
            if self.get(n).is_child(Side::Left)
                && self.get(s).is_child_black(Side::Right)
                && !self.get(s).is_child_black(Side::Left)
            {
                let scl = self.get(s).get_child(Side::Left);
                self.get_mut(s).color = Color::Red;
                self.set_maybe_black(scl);
                self.rotate(Side::Right, s);
            } else if self.get(n).is_child(Side::Right)
                && self.get(s).is_child_black(Side::Left)
                && !self.get(s).is_child_black(Side::Right)
            {
                let scr = self.get(s).get_child(Side::Right);
                self.get_mut(s).color = Color::Red;
                self.set_maybe_black(scr);
                self.rotate(Side::Left, s);
            }
        }
        self.delete_case_5(n)
    }

    fn delete_case_5(&mut self, n: usize) {
        let s = self.get(n).get_sibling().expect("D6 S");
        let p = self.get(n).parent.expect("D6 P");
        let pc = self.get(p).color;
        self.get_mut(s).color = pc;
        self.get_mut(p).color = Color::Black;

        if self.get(n).is_child(Side::Left) {
            let scr = self.get(s).get_child(Side::Right);
            self.set_maybe_black(scr);
            self.rotate(Side::Left, p);
        } else {
            let scl = self.get(s).get_child(Side::Left);
            self.set_maybe_black(scl);
            self.rotate(Side::Right, p);
        }
    }

    fn fix_del_color_long(&mut self) {
        let mut t = RBTree::new();
        let mut v = self.data.borrow_mut().pop();
        while v.is_some() {
            let n = v.unwrap();
            if n.ptr != TREE_END {
                t.insert(n.value.expect("Failed to insert value"));
            }
            v = self.data.borrow_mut().pop();
        }

        *self = t;
    }

    fn do_ins_hard_case(&mut self, nn: usize) {
        let mut n = nn;
        let mut p = self.get(n).parent.unwrap();
    
        // Left-Right or Right-Left cases
        if self.get(p).is_child(Side::Left) && self.get(n).is_child(Side::Right) {
            self.rotate(Side::Left, n); // Left-Right case
            n = self.get(n).get_child(Side::Left).unwrap();
        }
    
        p = self.get(n).parent.unwrap();
        if self.get(p).is_child(Side::Right) && self.get(n).is_child(Side::Left) {
            self.rotate(Side::Right, n); // Right-Left case
            n = self.get(n).get_child(Side::Right).unwrap();
        }
    
        self.do_ins_hard_case2(n); // After rotation, handle case2
    }

    fn do_ins_hard_case2(&mut self, n: usize) {
        let p = self.get(n).parent.unwrap();
        let g = self.get(p).get_parent().unwrap();
    
        self.get_mut(p).color = Color::Black; // Parent becomes black
        self.get_mut(g).color = Color::Red; // Grandparent becomes red
    
        // Rotate grandparent to maintain balance
        if self.get(p).is_child(Side::Right) {
            self.rotate(Side::Left, p); // Rotate left if parent is right child
        } else if self.get(p).is_child(Side::Left) {
            self.rotate(Side::Right, p); // Rotate right if parent is left child
        }
    }
}

impl<T: Clone + Debug + Display> TreeItem for ColorNode<T> {
    type Child = Self;
    fn write_self<W: io::Write>(&self, f: &mut W, _style: &Style) -> io::Result<()> {
        // Match on the color to print appropriately
        match self.color {
            Color::Black => {
                match self.value.clone() {
                    Some(val) => write!(f, "({})", val.to_string().bright_black()), 
                    None => Ok(()),
                }
            }
            Color::Red => {
                match &self.value {
                    Some(val) => write!(f, "({})", val.to_string().red()), 
                    None => Ok(()),
                }
            }
        }
    }

    fn children(&self) -> Cow<[Self::Child]> {
        let mut vec: Vec<ColorNode<T>> = Vec::new();
    
        if self.lchild.is_some() {
            let lchild_index = self.lchild.expect("Left child should exist");
            let lchild_node = RefCell::<Vec<ColorNode<T>>>::borrow(&self.data)[lchild_index].clone();
            vec.push(lchild_node.clone());            
        }
        if self.rchild.is_some() {
            let rchild_index = self.rchild.expect("Left child should exist");
            let rchild_node = RefCell::<Vec<ColorNode<T>>>::borrow(&self.data)[rchild_index].clone();
            vec.push(rchild_node.clone());
            
        }
        Cow::from(vec)
    }

}
impl<T: Clone + Debug + Display> TreeItem for RBTree<T> {
    type Child = ColorNode<T>;
    
    fn write_self<W: Write>(&self, f: &mut W, _style: &Style) -> io::Result<()> {
        write!(f, "{}", "Root".blue())
    }
    fn children(&self) -> Cow<[Self::Child]> {
        match &self.root {
            Some(root) => {
                let root_index = root.clone();
                let root_node = RefCell::<Vec<ColorNode<T>>>::borrow(&self.data)[root_index].clone();
                Cow::from(vec![root_node.clone()])
            },
            None => {
                Cow::from(vec![])
            }
        }
    }
}
