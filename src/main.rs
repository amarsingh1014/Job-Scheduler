use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum TaskState {
    Ready,
    Running,
    Waiting,
    Completed,
}

#[derive(Debug, Eq)]
struct Task {
    id: usize,
    burst_time: usize,
    priority: usize,
    state: TaskState,
}

impl Task {
    fn new(id: usize, burst_time: usize, priority: usize) -> Task {
        Task {
            id,
            burst_time,
            priority,
            state: TaskState::Ready,
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority comes first
        other.priority.cmp(&self.priority)
            .then_with(|| self.burst_time.cmp(&other.burst_time)) // For same priority, shorter job first
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Scheduler {
    tasks: BinaryHeap<Task>,
    time_quantum: usize,
}

impl Scheduler {
    fn new(time_quantum: usize) -> Scheduler {
        Scheduler {
            tasks: BinaryHeap::new(),
            time_quantum,
        }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop() {
            if task.state == TaskState::Completed {
                continue;
            }
            
            task.state = TaskState::Running;
            if task.burst_time <= self.time_quantum {
                println!("Running task {} for {} units", task.id, task.burst_time);
                task.burst_time = 0;
                task.state = TaskState::Completed;
            } else {
                println!("Running task {} for {} units", task.id, self.time_quantum);
                task.burst_time -= self.time_quantum;
                task.state = TaskState::Ready;
            }

            if task.state != TaskState::Completed {
                task.state = TaskState::Waiting;
                self.tasks.push(task);
            }
        }
    }
}

fn main() {
    let mut scheduler = Scheduler::new(5); 

    scheduler.add_task(Task::new(0, 7, 2));
    scheduler.add_task(Task::new(1, 3, 1));
    scheduler.add_task(Task::new(2, 9, 3));
    scheduler.add_task(Task::new(3, 5, 2));

    scheduler.run();
}
