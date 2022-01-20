pub struct Countdown {
    current: u16,
    starting_from: u16,
}

impl Countdown {
    pub fn new(starting_from: u16) -> Self {
        Self {
            current: 0,
            starting_from,
        }
    }

    pub fn down(&mut self) {
        self.current = self.current.saturating_sub(1);
    }

    pub fn off(&self) -> bool {
        self.current == 0
    }

    pub fn on(&self) -> bool {
        !self.off()
    }

    pub fn restart(&mut self) {
        self.current = self.starting_from;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_always_off_if_starting_from_0() {
        let mut countdown = Countdown::new(0);
        assert!(countdown.off());
        countdown.down();
        assert!(countdown.off());
    }

    #[test]
    fn is_off_when_new() {
        let countdown = new_countdown_from_2();
        assert!(!countdown.on());
        assert!(countdown.off());
    }

    #[test]
    fn is_on_after_restart() {
        let mut countdown = new_countdown_from_2();
        countdown.restart();
        assert!(countdown.on());
        assert!(!countdown.off());
    }

    #[test]
    fn is_on_after_1_down() {
        let mut countdown = new_countdown_from_2();
        countdown.restart();
        down_n_times(&mut countdown, 1);
        assert!(countdown.on());
    }

    #[test]
    fn is_off_after_2_down() {
        let mut countdown = new_countdown_from_2();
        countdown.restart();
        assert!(!countdown.off());
        down_n_times(&mut countdown, 2);
        assert!(countdown.off());
    }

    #[test]
    fn stays_off_after_3_down() {
        let mut countdown = new_countdown_from_2();
        countdown.restart();
        assert!(!countdown.off());
        down_n_times(&mut countdown, 3);
        assert!(countdown.off());
    }

    fn down_n_times(countdown: &mut Countdown, n: usize) {
        for _ in 0..n {
            countdown.down();
        }
    }

    fn new_countdown_from_2() -> Countdown {
        Countdown::new(2)
    }
}
