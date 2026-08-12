use crate::{
    bots::{
        Bot, idabp_new::idabp_new, idabp_old::idabp_old, parse_bot, random_mover::random_mover,
        uses_time_limit,
    },
    heuristics::{Heuristic, parse_heuristic},
};
use std::{ops::Not, ptr::fn_addr_eq, time::Duration};

#[derive(Debug, Clone)]
pub enum PlayerKind {
    Human,
    Bot { bot: Bot, heuristic: Heuristic },
}

#[derive(Debug, Clone)]
pub struct Player {
    pub kind: PlayerKind,
    pub time_limit: Option<Duration>,
}

pub const DEFAULT_TIME_LIMIT: Duration = Duration::from_millis(500);

impl Player {
    pub const RANDOM: Self = Self {
        kind: PlayerKind::Bot { bot: random_mover, heuristic: Heuristic::ZERO },
        time_limit: None,
    };
    pub const MANUAL: Self = Self {
        kind: PlayerKind::Bot { bot: idabp_new, heuristic: Heuristic::MANUAL },
        time_limit: None,
    };

    fn new() -> Self {
        Self {
            kind: PlayerKind::Bot { bot: idabp_new, heuristic: Heuristic::new() },
            time_limit: None,
        }
    }

    fn old() -> Self {
        Self {
            kind: PlayerKind::Bot { bot: idabp_old, heuristic: Heuristic::old() },
            time_limit: None,
        }
    }

    #[inline]
    #[must_use]
    pub const fn is_human(&self) -> bool {
        matches!(self.kind, PlayerKind::Human)
    }

    #[inline]
    #[must_use]
    pub const fn is_bot(&self) -> bool {
        matches!(self.kind, PlayerKind::Bot { .. })
    }
}

impl PartialEq for PlayerKind {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Bot {
                    bot: l_bot,
                    heuristic: Heuristic { fun: l_heuristic, coeffs: l_coeffs },
                },
                Self::Bot {
                    bot: r_bot,
                    heuristic: Heuristic { fun: r_heuristic, coeffs: r_coeffs },
                },
            ) => {
                fn_addr_eq(*l_bot, *r_bot)
                    && fn_addr_eq(*l_heuristic, *r_heuristic)
                    && l_coeffs == r_coeffs
            }
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialEq for Player {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.time_limit == other.time_limit && self.kind == other.kind
    }
}

#[expect(clippy::fallible_impl_from)]
impl From<&str> for Player {
    #[inline]
    fn from(v: &str) -> Self {
        match v {
            "human" => Self { kind: PlayerKind::Human, time_limit: None },
            "random" => Self::RANDOM,
            "manual" => Self::MANUAL,
            "old" => Self::old(),
            "new" => Self::new(),
            _ => {
                let mut words = v.split(':');
                let kind_arg = words.next().expect(&format!("Invalid arg: {v}"));
                let heuristic_arg = words.next().expect(&format!("Invalid arg: {v}"));
                let time_limit_arg = words.next();
                assert!(words.next().is_none(), "Invalid arg: {v}");

                let heuristic = parse_heuristic(heuristic_arg).unwrap();
                let kind = if kind_arg == "human" {
                    PlayerKind::Human
                } else {
                    PlayerKind::Bot { bot: parse_bot(kind_arg).unwrap(), heuristic }
                };
                let time_limit = time_limit_arg.map(|limit| {
                    Duration::from_millis(
                        limit.parse().expect(&format!("Invalid time_limit: `{limit}`")),
                    )
                });
                if let Some(limit) = time_limit {
                    let millis = limit.as_millis();
                    if millis == 0 {
                        panic!("Invalid time_limit: `{millis}` must be > 0");
                    }
                    if let PlayerKind::Bot { bot, .. } = &kind
                        && uses_time_limit(bot)
                    {
                        panic!("Invalid time_limit: `{millis}` must be > 0");
                    }
                }

                Self { kind, time_limit }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerColor {
    Black,
    White,
}

impl Not for PlayerColor {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
