// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

pub const RUNE_ALGIZ: &str = "ᛉ";
pub const RUNE_OTHALA: &str = "ᛟ";
pub const RUNE_THURISAZ: &str = "ᚦ";

pub struct NonSpinnerRune {
    pub rune: &'static str,
    pub fallback: &'static str,
}

pub struct SpinnerRune {
    pub rune: &'static str,
    pub fallback: &'static str,
}

/// These 8 runes visually spell "NECRONUX" using Elder Futhark characters.
/// Fallbacks are the ASCII letters of the app name.
pub const SPINNER_RUNE_TICKS_WITH_FALLBACK: [SpinnerRune; 8] = [
    SpinnerRune {
        rune: "ᚾ",
        fallback: "+",
    },
    SpinnerRune {
        rune: "ᛖ",
        fallback: "#",
    },
    SpinnerRune {
        rune: "ᚲ",
        fallback: "<",
    },
    SpinnerRune {
        rune: "ᚱ",
        fallback: "&",
    },
    SpinnerRune {
        rune: SUCCESS_RUNE_WITH_FALLBACK.rune,
        fallback: SUCCESS_RUNE_WITH_FALLBACK.fallback,
    },
    SpinnerRune {
        rune: "ᚾ",
        fallback: "+",
    },
    SpinnerRune {
        rune: "ᚢ",
        fallback: "^",
    },
    SpinnerRune {
        rune: PROGRESS_RUNE_WITH_FALLBACK.rune,
        fallback: PROGRESS_RUNE_WITH_FALLBACK.fallback,
    },
];

pub const PROGRESS_RUNE_WITH_FALLBACK: NonSpinnerRune = NonSpinnerRune {
    rune: RUNE_ALGIZ,
    fallback: "~",
};

pub const SUCCESS_RUNE_WITH_FALLBACK: NonSpinnerRune = NonSpinnerRune {
    rune: RUNE_OTHALA,
    fallback: "$",
};
pub const WARNING_RUNE_WITH_FALLBACK: NonSpinnerRune = NonSpinnerRune {
    rune: RUNE_THURISAZ,
    fallback: "!",
};

pub const REGULAR_RUNE_WITH_FALLBACK: NonSpinnerRune = NonSpinnerRune {
    rune: RUNE_ALGIZ,
    fallback: "-",
};
