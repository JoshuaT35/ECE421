use std::cmp::max;
use std::ops::Not;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Side;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

#[derive(Debug)]
pub struct DepthNode<T> {
    pub value: T,
    pub ptr: usize,
    pub parent: Option<usize>,
    pub lchild: Option<usize>,
    pub rchild: Option<usize>,
    pub height: usize,
}

pub trait Node<T> {
    // Base methods
    fn get_value(&self) -> &T;
    fn get(&self, i: usize) -> &Self;
    fn get_mut(&self, i: usize) -> &mut Self;
    fn location(&self) -> usize;
    fn get_parent(&self) -> Option<usize>;
    fn set_parent(&mut self, p: Option<usize>);
    fn get_child(&self, side: Side) -> Option<usize>;
    fn set_child(&mut self, child: Option<usize>, side: Side);
    fn is(&self, val: &T) -> bool;
    fn greater(&self, val: &T) -> bool;
    fn lesser(&self, val: &T) -> bool;

    fn get_height(&self) -> usize {
        let f = |c| Some(1 + self.get(c).get_height());
        max(
            self.get_child(Side::Left).and_then(f).unwrap_or(1),
            self.get_child(Side::Right).and_then(f).unwrap_or(1),
        )
    }

    fn get_depth(&self) -> usize {
        let f = |c| Some(1 + self.get(c).get_depth());
        self.get_parent().and_then(f).unwrap_or(0)
    }

    fn get_leaf_count(&self) -> usize {
        let f = |c| Some(self.get(c).get_leaf_count());
        let val = self.get_child(Side::Left).and_then(f).unwrap_or(0) + self.get_child(Side::Right).and_then(f).unwrap_or(0);
        if val == 0 {
            1
        } else {
            val
        }
    }

    fn find_min(&self) -> usize {
        if let Some(l) = self.get_child(Side::Left) {
            self.get(l).find_min()
        } else {
            self.location()
        }
    }

    fn side(&self) -> Side {
        if self.is_child(Side::Left) {
            Side::Left
        } else {
            Side::Right
        }
    }

    fn get_sibling(&self) -> Option<usize> {
        if let Some(p) = self.get_parent() {
            let parent = self.get(p);
            if self.is_child(Side::Left) {
                parent.get_child(Side::Right)
            } else if self.is_child(Side::Right) {
                parent.get_child(Side::Left)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_uncle(&self) -> Option<usize> {
        self.get_parent()
            .and_then(|p| Some(self.get(p)))
            .and_then(|p| p.get_sibling())
    }

    fn is_child(&self, side: Side) -> bool {
        if let Some(p) = self.get_parent() {
            let parent = self.get(p);
            parent.get_child(side).is_some() && parent.get_child(side).unwrap() == self.location()
        } else {
            false
        }
    }
}