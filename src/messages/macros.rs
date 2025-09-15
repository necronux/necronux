// ==-----------------------------------------------------------== //
// SPDX-FileCopyrightText: © 2025 Nayan Patil <nayantsg@proton.me>
//
// SPDX-License-Identifier: GPL-3.0-or-later
// ==-----------------------------------------------------------== //

macro_rules! new_line {
    ($w:expr) => {
        writeln!($w)
    };
}

macro_rules! json_obj {
    ( $( $k:expr => $v:expr ),* $(,)? ) => {{
        let mut map = serde_json::Map::new();
        $( map.insert($k.into(), $v.into()); )*
        map
    }};
}

// ----- STDERR (no JSON fields) -----

macro_rules! task_msg {
    (
        $theme:expr,
        $w:expr,
        wants_spinner: $wants_spinner:expr,
        has_steps: $has_steps:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stderr_msg(
            $w,
            $crate::messages::MsgKind::Task {
                wants_spinner: $wants_spinner,
                has_steps: $has_steps,
            },
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ]
        )
    };
}

macro_rules! step_msg {
    (
        $theme:expr,
        $w:expr,
        wants_spinner: $wants_spinner:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stderr_msg(
            $w,
            $crate::messages::MsgKind::Step {
                wants_spinner: $wants_spinner,
            },
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ]
        )
    };
}

macro_rules! warning_msg {
    (
        $theme:expr,
        $w:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stderr_msg(
            $w,
            $crate::messages::MsgKind::Warning,
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ]
        )
    };
}

macro_rules! regular_msg {
    (
        $theme:expr,
        $w:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stderr_msg(
            $w,
            $crate::messages::MsgKind::Regular,
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ]
        )
    };
}

// ----- STDOUT (optional JSON fields) -----

macro_rules! result_success_msg {
    (
        $theme:expr,
        $w:expr,
        $json_fields:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stdout_msg(
            $w,
            $crate::messages::MsgKind::StdOutSuccess,
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ],
            $json_fields
        )
    };
}

macro_rules! result_regular_msg {
    (
        $theme:expr,
        $w:expr,
        $json_fields:expr,
        $( ($text:expr, $style:expr) ),+ $(,)?
    ) => {
        $theme.new_stdout_msg(
            $w,
            $crate::messages::MsgKind::StdOutRegular,
            &[
                $( $crate::messages::MsgFragment::new($text, $style) ),+
            ],
            $json_fields
        )
    };
}
