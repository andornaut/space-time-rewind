use std::time::{Duration, Instant};

pub trait TickHandler {
    fn handle_tick(&mut self, ticker: &Ticker);
}

#[derive(Copy, Clone, Debug)]
pub enum Frequency {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
}

impl From<Frequency> for u16 {
    fn from(frequency: Frequency) -> u16 {
        match frequency {
            Frequency::One => 1,
            Frequency::Two => 2,
            Frequency::Three => 4,
            Frequency::Four => 8,
            Frequency::Five => 16,
            Frequency::Six => 32,
            Frequency::Seven => 64,
            Frequency::Eight => 128,
            Frequency::Nine => 256,
            Frequency::Ten => 512,
            Frequency::Eleven => 1024,
            Frequency::Twelve => 2048,
        }
    }
}

const MAX_NUMBER: u16 = 2048;

pub struct Ticker {
    cycles: u16,
    number: u16,
    last_tick: Option<Instant>,
    tick_rate: Duration,
}

impl Ticker {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            cycles: 0,
            number: 0,
            last_tick: None,
            tick_rate,
        }
    }

    pub fn maybe_tick(&mut self) -> bool {
        let last_tick = match self.last_tick {
            Some(tick) => tick,
            None => self.reset_last_tick(),
        };
        let should_tick = last_tick.elapsed() >= self.tick_rate;
        if should_tick {
            self.reset_last_tick();
            self.tick();
        }
        return should_tick;
    }

    pub fn remaining_timeout(&self) -> Duration {
        let elapsed = self.last_tick.unwrap_or_else(|| Instant::now()).elapsed();
        self.tick_rate
            .checked_sub(elapsed)
            .unwrap_or_else(|| Duration::from_secs(0))
    }

    pub fn restart(&mut self) {
        self.cycles = 0;
        self.last_tick = None;
        self.number = 0;
    }

    pub fn should(&self, frequency: Frequency) -> bool {
        // `number` is initialized to 0, so this will return true for all `frequency` when `self` is first initialized.
        self.number % u16::from(frequency) == 0
    }

    pub fn tick(&mut self) {
        self.number = if self.number == MAX_NUMBER {
            self.cycles += 1;
            1 // Restart at 1, so that `number==0` is only true on the first tick.
        } else {
            self.number + 1
        };
    }

    pub fn number(&self) -> u32 {
        // Will overflow after ~2.75 years at 1 tick every 20ms.
        u32::from(self.cycles) * u32::from(MAX_NUMBER) + u32::from(self.number)
    }

    fn reset_last_tick(&mut self) -> Instant {
        let now = Instant::now();
        self.last_tick = Some(now);
        now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_0() {
        let ticker = Ticker::new(Duration::from_secs(1));

        assert_eq!(ticker.number, 0);
    }

    #[test]
    fn should_is_true_for_any_frequencies_at_start() {
        let ticker = Ticker::new(Duration::from_secs(1));

        assert_eq!(ticker.should(Frequency::One), true);
        assert_eq!(ticker.should(Frequency::Five), true);
        assert_eq!(ticker.should(Frequency::Ten), true);
    }

    #[test]
    fn increments_by_1() {
        let mut ticker = Ticker::new(Duration::from_secs(1));

        ticker.tick();
        assert_eq!(ticker.number, 1);
        ticker.tick();
        assert_eq!(ticker.number, 2);
    }

    #[test]
    fn restarts_at_1_after_2048() {
        let mut ticker = Ticker::new(Duration::from_secs(1));
        ticker.number = 2047;

        ticker.tick();
        assert_eq!(ticker.number, 2048);
        ticker.tick();
        assert_eq!(ticker.number, 1);
    }

    #[test]
    fn tick_speed_one_fires_every_time() {
        assert_eq!(count_in_1_cycle(Frequency::One), MAX_NUMBER);
    }

    #[test]
    fn tick_speed_two_fires_half_the_time() {
        assert_eq!(count_in_1_cycle(Frequency::Two), MAX_NUMBER / 2);
    }

    #[test]
    fn tick_speed_twelve_fires_once() {
        assert_eq!(count_in_1_cycle(Frequency::Twelve), 1);
    }

    fn count_in_1_cycle(frequency: Frequency) -> u16 {
        let mut ticker = Ticker::new(Duration::from_secs(1));
        let mut count = 0;
        for _ in 0..MAX_NUMBER {
            if ticker.should(frequency) {
                count += 1;
            }
            ticker.tick();
        }
        count
    }
}
