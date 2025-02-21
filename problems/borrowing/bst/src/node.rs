#![forbid(unsafe_code)]

use std::borrow::Borrow;
pub struct Node<K, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    height: i32,
    balance_factor: i32,
    len: usize,
}

impl<'a, K: Ord, V> Node<K, V> {

    pub fn new(key: K, value: V,
               left: Option<Box<Node<K, V>>>,
               right: Option<Box<Node<K, V>>>) -> Node<K, V> {
        let (height, balance_factor, len) = Node::calculate_height_bf_len(&left, &right);
        Self {
            key: key,
            value: value,
            left: left,
            right: right,
            height: height,
            balance_factor: balance_factor,
            len: len,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn calculate_height_bf_len(left: &Option<Box<Node<K, V>>>, right: &Option<Box<Node<K, V>>>) -> (i32, i32, usize) {
        match (left, right) {
            (Some(left), Some(right)) =>
                (std::cmp::max(left.height, right.height) + 1, right.height - left.height, left.len + right.len + 1),
            (Some(left), None) =>
                (left.height + 1, 0 - left.height, left.len + 1),
            (None, Some(right)) =>
                (right.height + 1, right.height - 0, right.len + 1),
            (None, None) =>
                (1, 0, 1),
        }
    }

    fn update_height_bf_len(node: &mut Box<Node<K, V>>) {
        (node.height, node.balance_factor, node.len) = Node::calculate_height_bf_len(&node.left, &node.right);
    }

    fn rotate_left_move_const(node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if let Some(rnode) = node.right {

            let lnode = Box::<Node<K, V>>::new(Node::new(node.key, node.value, node.left, rnode.left));

            let resnode = Box::<Node<K, V>>::new(Node::new(rnode.key, rnode.value, Some(lnode), rnode.right));
            return resnode;
        }
        panic!("Rotate left (move) can't be performed!");
    }

    fn rotate_left(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if let Some(mut rnode) = node.right {
            node.right = rnode.left;
            Node::update_height_bf_len(&mut node);
            rnode.left = Some(node);
            Node::update_height_bf_len(&mut rnode);
            return rnode
        }
        panic!("Rotate left can't be performed!");
    }

    fn rotate_right_move_const(node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if let Some(lnode) = node.left {

            let rnode = Box::<Node<K, V>>::new(Node::new(node.key, node.value, lnode.right, node.right));

            let resnode = Box::<Node<K, V>>::new(Node::new(lnode.key, lnode.value, lnode.left, Some(rnode)));
            return resnode
        }
        panic!("Rotate right (move) can't be performed!");
    }

    fn rotate_right(mut node: Box<Node<K, V>>) -> Box<Node<K, V>>  {
        if let Some(mut lnode) = node.left {
            node.left = lnode.right;
            Node::update_height_bf_len(&mut node);
            lnode.right = Some(node);
            Node::update_height_bf_len(&mut lnode);
            return lnode
        }
        panic!("Rotate right can't be performed!");
    }

    fn balance_move_const(node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        match &node.balance_factor {
            -2 => {
                let new_node = match &node.left {
                    Some(lrefnode) if lrefnode.balance_factor > 0 =>
                        Box::new(Node::new(node.key, node.value,
                            Some(Node::rotate_left_move_const(node.left.unwrap())),
                            node.right)),
                    _ => node,
                };
                Node::rotate_right_move_const(new_node)
            },
            -1 | 0 | 1 => node,
            2 => {
                let new_node = match &node.right {
                    Some(rrefnode) if rrefnode.balance_factor < 0 =>
                        Box::new(Node::new(node.key, node.value,
                            node.left,
                            Some(Node::rotate_right_move_const(node.right.unwrap())))),
                    _ => node,
                };
                Node::rotate_left_move_const(new_node)
            },
            _ => panic!("Tree unbalanced more than 2!"),
        }
    }

    fn balance(mut node: Box<Node<K, V>>) -> Box<Node<K, V>> {
        match &node.balance_factor {
            -2 => {
                node.left = match &node.left {
                    Some(lrefnode) if lrefnode.balance_factor > 0 =>
                        Some(Node::rotate_left(node.left.unwrap())),
                    _ => node.left,
                };
                Node::rotate_right(node)
            },
            -1 | 0 | 1 => node,
            2 => {
                node.right = match &node.right {
                    Some(rrefnode) if rrefnode.balance_factor < 0 =>
                        Some(Node::rotate_right(node.right.unwrap())),
                    _ => node.right,
                };
                Node::rotate_left(node)
            },
            bf => panic!("Tree unbalanced more than 2! bf = {}, height = {}, left = {}, right = {}",
                               bf, node.height, node.left.is_some(), node.right.is_some()),
        }
    }

    pub fn get_key_value<Q: ?Sized>(node: &'a Option<Box<Node<K, V>>>, key: &Q) -> Option<(&'a K, &'a V)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        if let Some(node) = node.as_ref() {
            return match key {
                key if key < (&node.key).borrow() => Node::get_key_value(&node.left, key),
                key if key > (&node.key).borrow() => Node::get_key_value(&node.right, key),
                _ => Some((&node.key, &node.value)),
            }
        }
        None
    }

    pub fn nth_key_value(node: &'a Option<Box<Node<K, V>>>, k: usize) -> Option<(&'a K, &'a V)> {
        if let Some(node) = node.as_ref() {
            return match k {
                k if k >= node.len => None, // k too big
                _ => {
                    match (node.left.as_ref(), node.right.as_ref()) {
                        (Some(lnode), Some(_rnode)) if k == lnode.len => Some((&node.key, &node.value)),
                        (Some(lnode), Some(_rnode)) if k < lnode.len => Node::nth_key_value(&node.left, k),
                        (Some(lnode), Some(_rnode)) if k > lnode.len => Node::nth_key_value(&node.right, k - 1 - lnode.len),
                        (Some(lnode), None) if k == lnode.len => Some((&node.key, &node.value)),
                        (Some(lnode), None) if k < lnode.len => Node::nth_key_value(&node.left, k),
                        (None, Some(_rnode)) if k == 0 => Some((&node.key, &node.value)),
                        (None, Some(_rnode)) if k > 0 => Node::nth_key_value(&node.right, k - 1),
                        (None, None) if k == 0 => Some((&node.key, &node.value)),
                        _ => panic!("Nth algo error"),
                    }
                },
            }
        }
        None
    }

    pub fn insert(node: Option<Box<Node<K, V>>>, key: K, value: V) -> (Box<Node<K, V>>, Option<V>) {
        if let Some(mut node) = node {
            let val: Option<V>;
            match key {
                key if key < node.key => {
                    let (ln, v) = Node::insert(node.left, key, value);
                    (node.left, val) = (Some(ln), v);
                    Node::update_height_bf_len(&mut node);
                    node = Node::balance(node);
                },
                key if key > node.key => {
                    let (rn, v) = Node::insert(node.right, key, value);
                    (node.right, val) = (Some(rn), v);
                    Node::update_height_bf_len(&mut node);
                    node = Node::balance(node);
                },
                key if key == node.key => {
                    val = Some(node.value);
                    node.value = value;
                },
                _ => panic!("Must be unreachable!"),
            }
            return (node, val)
        }
        (Box::new(Node::new(key, value, None, None)), None)
    }

    fn take_max(mut node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Box<Node<K, V>>) {
        if let Some(rnode) = node.right.take() {
            return match Node::take_max(rnode) {
                (None, max_node) => {
                    Node::update_height_bf_len(&mut node);
                    (Some(Node::balance(node)), max_node)
                }
                (Some(new_rnode), max_node) => {
                    node.right = Some(new_rnode);
                    Node::update_height_bf_len(&mut node);
                    (Some(Node::balance(node)), max_node)
                }
            }
        }
        (node.left.take(), node)
    }

    fn take_min(mut node: Box<Node<K, V>>) -> (Option<Box<Node<K, V>>>, Box<Node<K, V>>) {
        if let Some(lnode) = node.left.take() {
            return match Node::take_min(lnode) {
                (None, min_node) => {
                    Node::update_height_bf_len(&mut node);
                    (Some(Node::balance(node)), min_node)
                }
                (Some(new_lnode), min_node) => {
                    node.left = Some(new_lnode);
                    Node::update_height_bf_len(&mut node);
                    (Some(Node::balance(node)), min_node)
                }
            }
        }
        (node.right.take(), node)
    }

    pub fn remove_entry<Q: ?Sized>(mut node: Box<Node<K, V>>, key: &Q) -> (Option<Box<Node<K, V>>>, Option<(K, V)>)
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match key {
            key if key < (&node.key).borrow() => {
                match node.left.take() {
                    Some(lnode) => {
                        let (new_lnode, ret) = Node::remove_entry(lnode, key);
                        node.left = new_lnode;
                        Node::update_height_bf_len(&mut node);
                        (Some(Node::balance(node)), ret)
                    },
                    None => (Some(node), None),
                }
            },
            key if key > (&node.key).borrow() => {
                match node.right.take() {
                    Some(rnode) => {
                        let (new_rnode, ret) = Node::remove_entry(rnode, key);
                        node.right = new_rnode;
                        Node::update_height_bf_len(&mut node);
                        (Some(Node::balance(node)), ret)
                    },
                    None => (Some(node), None),
                }
            },
            key if key == (&node.key).borrow() => {
                let ret = Some((node.key, node.value));
                match (node.left, node.right) {
                    (Some(lnode), rnode) if node.balance_factor < 0 => {
                        let (new_lnode, mut new_node) = Node::take_max(lnode);
                        new_node.left = new_lnode;
                        new_node.right = rnode;
                        Node::update_height_bf_len(&mut new_node);
                        (Some(Node::balance(new_node)), ret)
                    },
                    (lnode, Some(rnode)) if node.balance_factor >= 0 => {
                        let (new_rnode, mut new_node) = Node::take_min(rnode);
                        new_node.right = new_rnode;
                        new_node.left = lnode;
                        Node::update_height_bf_len(&mut new_node);
                        (Some(Node::balance(new_node)), ret)
                    },
                    (None, None) => (None, ret),
                    _ => panic!("balance factor is inconsistent with left/right!"),
                }
            },
            _ => panic!("Unreachable!"),
        }
    }

}
