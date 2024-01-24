
pub mod avl;

#[derive(Debug)]
pub struct AVLNode {
    depth: usize,
    cnt : usize,
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
    parent: Option<Box<AVLNode>>,
}

pub impl AVLNode {
    fn new() -> AVLNode {
        AVLNode {
            depth: 1,
            cnt: 1,
            left: None,
            right: None,
            parent: None,
        }
    }

    // fn rotate_left(&mut self) {}

    // fn rotate_right(&mut self) {}

    fn insert(&mut self, element: usize) {
        if self.cnt == 0 {
            self.cnt = 1;
            return;
        }
        if element < self.cnt {
            if let Some(ref mut left) = self.left {
                left.insert(element);
            } else {
                let mut left = AVLNode::new();
                left.insert(element);
                self.left = Some(Box::new(left));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(element);
            } else {
                let mut right = AVLNode::new();
                right.insert(element);
                self.right = Some(Box::new(right));
            }
        }
    }

    fn contains(&self, element: &usize) -> bool {
        if self.cnt == 0 {
            return false;
        }
        if element < self.cnt {
            if let Some(ref left) = self.left {
                left.contains(element)
            } else {
                false
            }
        } else if element > self.cnt {
            if let Some(ref right) = self.right {
                right.contains(element)
            } else {
                false
            }
        } else {
            true
        }
    }
}
