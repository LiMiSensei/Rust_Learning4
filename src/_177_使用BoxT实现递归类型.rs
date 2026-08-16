struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

#[allow(dead_code)]
pub fn main() {
    let node = Node {
        data: 1,
        next: Some(Box::new(Node {
            data: 1,
            next: Some(Box::new(Node {
                data: 1,
                next: Some(Box::new(Node {
                    data: 1,
                    next: Some(Box::new(Node {
                        data: 1,
                        next: Some(Box::new(Node {
                            data: 1,
                            next: Some(Box::new(Node {
                                data: 1,
                                next: None,
                            })),
                        })),
                    })),
                })),
            })),
        })),
    };
}
