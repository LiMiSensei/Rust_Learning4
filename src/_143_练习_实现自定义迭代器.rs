enum Priorit {
    Low,
    Medium,
    Hight,
}

struct Task {
    name: String,
    priority: Priorit,
}

struct PriorityIterator<'a> {
    tasks: &'a [Task],
    current_priority: Priorit,
    next_index: usize,
}
impl<'a> PriorityIterator<'a> {
    fn new(tasks: &'a [Task]) -> Self {
        PriorityIterator {
            tasks,
            current_priority: Priorit::Medium,
            next_index: 0,
        }
    }
}
impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
#[allow(dead_code)]
pub fn main() {
    let mut number = ["One".to_string(), "Two".to_string(), "Three".to_string()];

    let iterate_by_immutable_borrow = number[..].iter();

    for i in iterate_by_immutable_borrow {
        println!("{}", i);
    }

    let a = [1, 2, 3, 4];
    let ret = a.iter().find(|&x| *x < 0);
}
