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
    pub number: u16,
    cycles: u16,
}

impl Default for Ticker {
    fn default() -> Ticker {
        Self::new(0)
    }
}

impl Ticker {
    pub fn should(&self, frequency: Frequency) -> bool {
        // `number` is initialized to 0, so this will return true for all `frequency` when `self` is first initialized.
        self.number % u16::from(frequency) == 0
    }

    pub fn tick(&mut self) {
        self.number = if self.number == MAX_NUMBER {
            self.cycles += 1;
            1
        } else {
            self.number + 1
        };
    }

    pub fn ticks(self) -> u32 {
        // Will overflow after ~2.75 years at 1 tick every 20ms.
        u32::from(self.cycles) * u32::from(MAX_NUMBER) + u32::from(self.number)
    }

    fn new(number: u16) -> Self {
        Self { cycles: 0, number }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_0() {
        let ticker = Ticker::default();

        assert_eq!(ticker.number, 0);
    }

    #[test]
    fn should_is_true_for_any_frequencies_at_start() {
        let ticker = Ticker::default();

        assert_eq!(ticker.should(Frequency::One), true);
        assert_eq!(ticker.should(Frequency::Five), true);
        assert_eq!(ticker.should(Frequency::Ten), true);
    }

    #[test]
    fn increments_by_1() {
        let mut ticker = Ticker::default();

        ticker.tick();
        assert_eq!(ticker.number, 1);
        ticker.tick();
        assert_eq!(ticker.number, 2);
    }

    #[test]
    fn restarts_at_1_after_2048() {
        let mut ticker = Ticker::new(2047);

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
        let mut ticker = Ticker::default();
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
