mod clock_part;
mod increment;
mod remaining;

pub use clock_part::ClockPart;
pub use increment::Increment;
pub use remaining::Remaining;

use core::fmt;
use core::time::Duration;

use board::Color;

use super::go_key::GoKey;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Clock {
    white: Duration,
    black: Duration,
    white_increment: Duration,
    black_increment: Duration,
}

impl Clock {
    #[must_use]
    pub const fn new(
        white: Duration,
        black: Duration,
        white_increment: Duration,
        black_increment: Duration,
    ) -> Clock {
        Clock {
            white,
            black,
            white_increment,
            black_increment,
        }
    }

    #[must_use]
    pub fn time<P: ClockPart>(&self, side: Color) -> Duration {
        P::read(self, side)
    }

    #[must_use]
    pub fn minus_spent_plus_increment(self, side: Color, spent: Duration) -> Clock {
        match side {
            Color::White => Clock {
                white: self.white.saturating_sub(spent) + self.white_increment,
                ..self
            },
            Color::Black => Clock {
                black: self.black.saturating_sub(spent) + self.black_increment,
                ..self
            },
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {} {} {} {} {} {} {}",
            GoKey::WTime,
            self.white.as_millis(),
            GoKey::BTime,
            self.black.as_millis(),
            GoKey::WInc,
            self.white_increment.as_millis(),
            GoKey::BInc,
            self.black_increment.as_millis()
        )
    }
}

#[cfg(feature = "proptest")]
impl proptest::arbitrary::Arbitrary for Clock {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Clock>;

    fn arbitrary_with((): ()) -> Self::Strategy {
        use proptest::strategy::Strategy;
        const PARTS: usize = 4;
        proptest::arbitrary::any::<[u32; PARTS]>()
            .prop_map(|[white, black, white_increment, black_increment]| {
                Clock::new(
                    Duration::from_millis(u64::from(white)),
                    Duration::from_millis(u64::from(black)),
                    Duration::from_millis(u64::from(white_increment)),
                    Duration::from_millis(u64::from(black_increment)),
                )
            })
            .boxed()
    }
}
