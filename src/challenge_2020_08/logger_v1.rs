use std::collections::VecDeque;

pub struct Logger {
    queue: VecDeque<(i32, String)>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            queue: VecDeque::default(),
        }
    }

    pub fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if self.queue.is_empty() || timestamp - self.queue[0].0 > 10 {
            self.queue.clear();
            self.queue.push_front((timestamp, message));
            return true;
        }
        if self.queue.iter().any(|(t, m)| timestamp - *t < 10 && *m == message) {
            false
        } else {
            self.queue.push_front((timestamp, message));
            true
        }
    }
}