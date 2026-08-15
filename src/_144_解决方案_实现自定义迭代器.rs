#[derive(PartialEq, Eq, Clone, Debug)]
enum Priorit {
    Low,
    Medium,
    Hight,
}

struct Task {
    name: String,
    priority: Priorit,
}

impl Task {
    fn new(name: &str, priority: Priorit) -> Self {
        Task {
            name: name.to_string(),
            priority,
        }
    }
}

struct PriorityIterator<'a> {
    tasks: &'a [Task],
    current_priority: Priorit,
    index: usize,
}
impl<'a> PriorityIterator<'a> {
    fn new(tasks: &'a [Task], priority: Priorit) -> Self {
        PriorityIterator {
            tasks,
            current_priority: priority,
            index: 0,
        }
    }
}
impl<'a> Iterator for PriorityIterator<'a> {
    type Item = &'a Task;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(pos) = self.tasks[self.index..]
                .iter()
                .position(|task| task.priority == self.current_priority)
            {
                //execute_task(&all_tasks[index - 1]);
                self.index += pos + 1;
                return Some(&self.tasks[self.index - 1]);
            } else {
                self.current_priority = match self.current_priority {
                    Priorit::Hight => Priorit::Medium,
                    Priorit::Medium => Priorit::Low,
                    Priorit::Low => return None,
                };
                self.index = 0
            }
        }
    }
}

fn execute_task(p0: &Task) {
    todo!()
}
// fn execute_tasks_by_ppriority(all_tasks: &Vec<Task>, start_priority: Priorit) {
//     let mut current_priority = start_priority;
//     let mut index = 0;
//     if let Some(pos) = all_tasks[index..]
//         .iter()
//         .position(|task| task.priority == start_priority)
//     {
//         execute_task(&all_tasks[index - 1]);
//         index += pos + 1;
//     } else {
//         current_priority = match current_priority {
//             Priorit::Hight => Priorit::Medium,
//             Priorit::Medium => Priorit::Low,
//             Priorit::Low => break,
//         };
//         index = 0
//     }
// }

#[allow(dead_code)]
pub fn main() {
    let all_tasks = vec![
        Task::new("Laundry", Priorit::Low),
        Task::new("Emails", Priorit::Hight),
        Task::new("Homworks", Priorit::Medium),
        Task::new("Cleaning", Priorit::Medium),
        Task::new("Taxes", Priorit::Hight),
    ];

    let pririty = PriorityIterator::new(&all_tasks, Priorit::Hight);
    println!("Tasks by priority");

    //execute_tasks_by_ppriority(&all_tasks, Priorit::Hight);
    let priority_tasks = PriorityIterator::new(&all_tasks, Priorit::Low);
}
