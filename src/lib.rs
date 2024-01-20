struct Node {
    lower: isize,
    upper: isize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(lower: isize, upper: isize, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            lower, upper,
            left,
            right,
        }
    }
}

pub struct Diet {
    head: Option<Box<Node>>,
    len: usize,
}

impl Diet {
    pub fn new() -> Self {
        Diet {
            head: None,
            len: 0,
        }
    }

    fn recur_contains(&self, num: isize, node: &Option<Box<Node>>) -> bool {
        match node {
            None => false,
            Some(node) => {
                if node.lower <= num && num <= node.upper {
                    true
                } else if num < node.lower {
                    self.recur_contains(num, &node.left)
                } else {
                    self.recur_contains(num, &node.right)
                }
            }
        }
    }

    pub fn contains(&self, num: isize) -> bool {
        self.recur_contains(num, &self.head)
    }

    fn split_max(mut node: Box<Node>) -> (isize, isize, Option<Box<Node>>) {
        match node.right {
            None => {
                let left = node.left.take();
                (node.lower, node.upper, left)
            },
            Some(right) => {
                let (x, y, new_right) = Self::split_max(right);
                node.right = new_right;
                (x, y, Some(node))
            }
        }
    }

    fn join_left(mut node: Box<Node>) -> Box<Node> {
        match node.left.take() {
            None => node,
            Some(left) => {
                let (x_right, y_right, new_left) = Self::split_max(left);
                if y_right + 1 == node.lower {
                    Box::new(Node::new(x_right, node.upper, new_left, node.right))
                } else {
                    node
                }
            }
        }
    }

    fn split_min(mut node: Box<Node>) -> (isize, isize, Option<Box<Node>>) {
        match node.left {
            None => {
                let right = node.right.take();
                (node.lower, node.upper, right)
            },
            Some(left) => {
                let (x, y, new_left) = Self::split_min(left);
                node.left = new_left;
                (x, y, Some(node))
            }
        }
    }

    fn join_right(mut node: Box<Node>) -> Box<Node> {
        match node.right.take() {
            None => node,
            Some(right) => {
                let (x_left, y_left, new_right) = Self::split_min(right);
                if x_left - 1 == node.upper {
                    Box::new(Node::new(node.lower, y_left, node.left, new_right))
                } else {
                    node
                }
            }
        }
    }

    fn recur_insert(num: isize, node: &mut Option<Box<Node>>) -> bool {
        match node {
            None => {
                *node = Some(Box::new(Node::new(num, num, None, None)));
                true
            },
            Some(inner) => {
                if num < inner.lower {
                    if num + 1 == inner.lower {
                        let left = inner.left.take();
                        let right = inner.right.take();
                        let tmp_node = Box::new(Node::new(num, inner.upper, left, right));
                        *node = Some(Self::join_left(tmp_node));
                        true
                    } else {
                        Self::recur_insert(num, &mut inner.left)
                    }
                } else if num > inner.upper {
                    if num - 1 == inner.upper {
                        let left = inner.left.take();
                        let right = inner.right.take();
                        let tmp_node = Box::new(Node::new(inner.lower, num, left, right));
                        *node = Some(Self::join_right(tmp_node));
                        true
                        
                    } else {
                        Self::recur_insert(num, &mut inner.right)
                    }
                } else {
                    // num in [upper, lower]
                    false
                }
            }
        }
    }

    pub fn len(&self) -> usize {
       self.len 
    }

    pub fn insert(&mut self, num: isize) -> bool {
        let result = Self::recur_insert(num, &mut self.head);
        if result {
            self.len += 1;
        }
        result
    }
    
    fn merge(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>>{
        match (left, right) {
            (left, None) => left,
            (None, right) => right,
            (Some(left), Some(right)) => {
                let (x, y, left) = Self::split_max(left);
                Some(Box::new(Node::new(x, y, left, Some(right))))

            }
        }
    }

    fn recur_remove(num: isize, node: &mut Option<Box<Node>>) -> bool {
        match node {
            None => false,
            Some(ref mut inner) => {
                if num < inner.lower {
                    Self::recur_remove(num, &mut inner.left)
                } else if num > inner.upper {
                    Self::recur_remove(num, &mut inner.right)
                } else {
                    if num == inner.lower {
                        if inner.lower == inner.upper {
                            let left = inner.left.take();
                            let right = inner.right.take();
                            *node = Self::merge(left, right);
                        } else {
                            // [x, y] -> [x + 1, y]
                            inner.lower += 1;
                        }
                    } else if num == inner.upper {
                        // [x, y] -> [x, y - 1]
                        inner.upper -= 1;
                    } else {
                        let left = inner.left.take();
                        let right = inner.right.take();
                        let upper_node = Some(Box::new(Node::new(num + 1, inner.upper, None, right)));
                        *node = Some(Box::new(Node::new(inner.lower, num - 1, left, upper_node)));
                    }
                    return true;
                } 
            }
        }
    }
    
    pub fn remove(&mut self, num: isize) -> bool {
        let result = Self::recur_remove(num, &mut self.head);
        if result {
            self.len -= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_set() -> Diet {
        let mut set = Diet::new();
        for i in 7..=10 {
            assert!(set.insert(i));
        }
        for i in 7..=10 {
            assert!(set.contains(i));
        }

        for i in 13..=14 {
            assert!(set.insert(i));
            assert!(set.contains(i));
        }

        assert!(set.insert(-1));
        assert!(set.contains(-1));

        assert!(set.insert(1));
        assert!(set.contains(1));

        assert!(set.insert(5));
        assert!(set.contains(5));

        assert!(set.insert(3));
        assert!(set.contains(3));
        
        assert!(set.insert(6));
        assert!(!set.insert(6));
        assert!(!set.insert(3));

        assert_eq!(set.len(), 11);
        set
    }

    #[test]
    fn insert_and_delete() {
        let mut set = create_set();

        assert!(set.remove(13));
        assert!(!set.contains(13));

        assert!(set.remove(14));
        assert!(!set.contains(14));

        assert_eq!(set.len(), 9);

        assert!(set.remove(-1));
        assert!(!set.contains(-1));

        assert!(!set.remove(-2));
        assert_eq!(set.len(), 8);
    }
}
