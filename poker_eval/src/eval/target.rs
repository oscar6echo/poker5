//! Expected hand stats for 5 and 7 cards

#[cfg(feature = "serde")]
use serde::Serialize;

/// Hand stats container for a hand type, eg. "high-card", "one-pair", etc.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct HandStats {
    /// Number of hands
    pub nb_hand: u32,
    /// Minimum rank
    pub min_rank: u32,
    /// Maximum rank
    pub max_rank: u32,
    /// Number of occurrences
    pub nb_occur: u32,
}

/// Expected hand stats for 5 cards
pub const STATS_FIVE: [(&str, HandStats); 9] = [
    (
        "high-card",
        HandStats {
            nb_hand: 1277,
            min_rank: 0,
            max_rank: 1276,
            nb_occur: 1302540,
        },
    ),
    (
        "one-pair",
        HandStats {
            nb_hand: 2860,
            min_rank: 1277,
            max_rank: 4136,
            nb_occur: 1098240,
        },
    ),
    (
        "two-pairs",
        HandStats {
            nb_hand: 858,
            min_rank: 4137,
            max_rank: 4994,
            nb_occur: 123552,
        },
    ),
    (
        "three-of-a-kind",
        HandStats {
            nb_hand: 858,
            min_rank: 4995,
            max_rank: 5852,
            nb_occur: 54912,
        },
    ),
    (
        "straight",
        HandStats {
            nb_hand: 10,
            min_rank: 5853,
            max_rank: 5862,
            nb_occur: 10200,
        },
    ),
    (
        "flush",
        HandStats {
            nb_hand: 1277,
            min_rank: 5863,
            max_rank: 7139,
            nb_occur: 5108,
        },
    ),
    (
        "full-house",
        HandStats {
            nb_hand: 156,
            min_rank: 7140,
            max_rank: 7295,
            nb_occur: 3744,
        },
    ),
    (
        "four-of-a-kind",
        HandStats {
            nb_hand: 156,
            min_rank: 7296,
            max_rank: 7451,
            nb_occur: 624,
        },
    ),
    (
        "straight-flush",
        HandStats {
            nb_hand: 10,
            min_rank: 7452,
            max_rank: 7461,
            nb_occur: 40,
        },
    ),
];

/// Expected hand stats for 7 cards
pub const STATS_SEVEN: [(&str, HandStats); 9] = [
    (
        "high-card",
        HandStats {
            nb_hand: 407,
            min_rank: 48,
            max_rank: 1276,
            nb_occur: 23294460,
        },
    ),
    (
        "one-pair",
        HandStats {
            nb_hand: 1470,
            min_rank: 1295,
            max_rank: 4136,
            nb_occur: 58627800,
        },
    ),
    (
        "two-pairs",
        HandStats {
            nb_hand: 763,
            min_rank: 4140,
            max_rank: 4994,
            nb_occur: 31433400,
        },
    ),
    (
        "three-of-a-kind",
        HandStats {
            nb_hand: 575,
            min_rank: 5003,
            max_rank: 5852,
            nb_occur: 6461620,
        },
    ),
    (
        "straight",
        HandStats {
            nb_hand: 10,
            min_rank: 5853,
            max_rank: 5862,
            nb_occur: 6180020,
        },
    ),
    (
        "flush",
        HandStats {
            nb_hand: 1277,
            min_rank: 5863,
            max_rank: 7139,
            nb_occur: 4047644,
        },
    ),
    (
        "full-house",
        HandStats {
            nb_hand: 156,
            min_rank: 7140,
            max_rank: 7295,
            nb_occur: 3473184,
        },
    ),
    (
        "four-of-a-kind",
        HandStats {
            nb_hand: 156,
            min_rank: 7296,
            max_rank: 7451,
            nb_occur: 224848,
        },
    ),
    (
        "straight-flush",
        HandStats {
            nb_hand: 10,
            min_rank: 7452,
            max_rank: 7461,
            nb_occur: 41584,
        },
    ),
];

#[cfg(test)]
mod tests {

    use super::HandStats;
    use crate::util::is_normal;

    #[test]
    fn check_hand_stats_normal() {
        is_normal::<HandStats>();
    }
}
