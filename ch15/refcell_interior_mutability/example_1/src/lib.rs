pub trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max > 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max > 0.9 {
            self.messenger
                .send("Urgent warning: You have used up 90 percent of your quota!");
        } else if percentage_of_max > 0.75 {
            self.messenger
                .send("Warning: You have used up 75 percent of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        pub fn new() -> Self {
            Self {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.push(msg.to_string());
        }
    }

    #[test]
    fn it_sends_an_over_75_warning() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);
        tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
