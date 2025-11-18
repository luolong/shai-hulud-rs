/// Collection of Indicatif spinner styles generated from
/// [simeg/bash-cli-spinners](https://raw.githubusercontent.com/simeg/bash-cli-spinners/refs/heads/main/spinners.json)
#[allow(dead_code)]
pub(crate) mod spinners {
    use indicatif::ProgressStyle;

    pub fn dots() -> ProgressStyle {
        dots_with_final(" ")
    }

    pub fn dots_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠋",
            "⠙",
            "⠹",
            "⠸",
            "⠼",
            "⠴",
            "⠦",
            "⠧",
            "⠇",
            "⠏",
            &final_tick,
        ])
    }

    pub fn dots2() -> ProgressStyle {
        dots2_with_final(" ")
    }

    pub fn dots2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⣾",
            "⣽",
            "⣻",
            "⢿",
            "⡿",
            "⣟",
            "⣯",
            "⣷",
            &final_tick,
        ])
    }

    pub fn dots3() -> ProgressStyle {
        dots3_with_final(" ")
    }

    pub fn dots3_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠋",
            "⠙",
            "⠚",
            "⠞",
            "⠖",
            "⠦",
            "⠴",
            "⠲",
            "⠳",
            "⠓",
            &final_tick,
        ])
    }

    pub fn dots4() -> ProgressStyle {
        dots4_with_final(" ")
    }

    pub fn dots4_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠄",
            "⠆",
            "⠇",
            "⠋",
            "⠙",
            "⠸",
            "⠰",
            "⠠",
            "⠰",
            "⠸",
            "⠙",
            "⠋",
            "⠇",
            "⠆",
            &final_tick,
        ])
    }

    pub fn dots5() -> ProgressStyle {
        dots5_with_final(" ")
    }

    pub fn dots5_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠋",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            &final_tick,
        ])
    }

    pub fn dots6() -> ProgressStyle {
        dots6_with_final(" ")
    }

    pub fn dots6_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠴",
            "⠲",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠚",
            "⠙",
            "⠉",
            "⠁",
            &final_tick,
        ])
    }

    pub fn dots7() -> ProgressStyle {
        dots7_with_final(" ")
    }

    pub fn dots7_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠈",
            "⠉",
            "⠋",
            "⠓",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠖",
            "⠦",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈",
            &final_tick,
        ])
    }

    pub fn dots8() -> ProgressStyle {
        dots8_with_final(" ")
    }

    pub fn dots8_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠁",
            "⠁",
            "⠉",
            "⠙",
            "⠚",
            "⠒",
            "⠂",
            "⠂",
            "⠒",
            "⠲",
            "⠴",
            "⠤",
            "⠄",
            "⠄",
            "⠤",
            "⠠",
            "⠠",
            "⠤",
            "⠦",
            "⠖",
            "⠒",
            "⠐",
            "⠐",
            "⠒",
            "⠓",
            "⠋",
            "⠉",
            "⠈",
            "⠈",
            &final_tick,
        ])
    }

    pub fn dots9() -> ProgressStyle {
        dots9_with_final(" ")
    }

    pub fn dots9_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⢹",
            "⢺",
            "⢼",
            "⣸",
            "⣇",
            "⡧",
            "⡗",
            "⡏",
            &final_tick,
        ])
    }

    pub fn dots10() -> ProgressStyle {
        dots10_with_final(" ")
    }

    pub fn dots10_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⢄",
            "⢂",
            "⢁",
            "⡁",
            "⡈",
            "⡐",
            "⡠",
            &final_tick,
        ])
    }

    pub fn dots11() -> ProgressStyle {
        dots11_with_final(" ")
    }

    pub fn dots11_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠁",
            "⠂",
            "⠄",
            "⡀",
            "⢀",
            "⠠",
            "⠐",
            "⠈",
            &final_tick,
        ])
    }

    pub fn dots12() -> ProgressStyle {
        dots12_with_final("  ")
    }

    pub fn dots12_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⢀⠀",
            "⡀⠀",
            "⠄⠀",
            "⢂⠀",
            "⡂⠀",
            "⠅⠀",
            "⢃⠀",
            "⡃⠀",
            "⠍⠀",
            "⢋⠀",
            "⡋⠀",
            "⠍⠁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⢈⠩",
            "⡀⢙",
            "⠄⡙",
            "⢂⠩",
            "⡂⢘",
            "⠅⡘",
            "⢃⠨",
            "⡃⢐",
            "⠍⡐",
            "⢋⠠",
            "⡋⢀",
            "⠍⡁",
            "⢋⠁",
            "⡋⠁",
            "⠍⠉",
            "⠋⠉",
            "⠋⠉",
            "⠉⠙",
            "⠉⠙",
            "⠉⠩",
            "⠈⢙",
            "⠈⡙",
            "⠈⠩",
            "⠀⢙",
            "⠀⡙",
            "⠀⠩",
            "⠀⢘",
            "⠀⡘",
            "⠀⠨",
            "⠀⢐",
            "⠀⡐",
            "⠀⠠",
            "⠀⢀",
            "⠀⡀",
            &final_tick,
        ])
    }

    pub fn dots13() -> ProgressStyle {
        dots13_with_final(" ")
    }

    pub fn dots13_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⣼",
            "⣹",
            "⢻",
            "⠿",
            "⡟",
            "⣏",
            "⣧",
            "⣶",
            &final_tick,
        ])
    }

    pub fn dots14() -> ProgressStyle {
        dots14_with_final("  ")
    }

    pub fn dots14_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠉⠉",
            "⠈⠙",
            "⠀⠹",
            "⠀⢸",
            "⠀⣰",
            "⢀⣠",
            "⣀⣀",
            "⣄⡀",
            "⣆⠀",
            "⡇⠀",
            "⠏⠀",
            "⠋⠁",
            &final_tick,
        ])
    }

    pub fn dots8_bit() -> ProgressStyle {
        dots8_bit_with_final(" ")
    }

    pub fn dots8_bit_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠀",
            "⠁",
            "⠂",
            "⠃",
            "⠄",
            "⠅",
            "⠆",
            "⠇",
            "⡀",
            "⡁",
            "⡂",
            "⡃",
            "⡄",
            "⡅",
            "⡆",
            "⡇",
            "⠈",
            "⠉",
            "⠊",
            "⠋",
            "⠌",
            "⠍",
            "⠎",
            "⠏",
            "⡈",
            "⡉",
            "⡊",
            "⡋",
            "⡌",
            "⡍",
            "⡎",
            "⡏",
            "⠐",
            "⠑",
            "⠒",
            "⠓",
            "⠔",
            "⠕",
            "⠖",
            "⠗",
            "⡐",
            "⡑",
            "⡒",
            "⡓",
            "⡔",
            "⡕",
            "⡖",
            "⡗",
            "⠘",
            "⠙",
            "⠚",
            "⠛",
            "⠜",
            "⠝",
            "⠞",
            "⠟",
            "⡘",
            "⡙",
            "⡚",
            "⡛",
            "⡜",
            "⡝",
            "⡞",
            "⡟",
            "⠠",
            "⠡",
            "⠢",
            "⠣",
            "⠤",
            "⠥",
            "⠦",
            "⠧",
            "⡠",
            "⡡",
            "⡢",
            "⡣",
            "⡤",
            "⡥",
            "⡦",
            "⡧",
            "⠨",
            "⠩",
            "⠪",
            "⠫",
            "⠬",
            "⠭",
            "⠮",
            "⠯",
            "⡨",
            "⡩",
            "⡪",
            "⡫",
            "⡬",
            "⡭",
            "⡮",
            "⡯",
            "⠰",
            "⠱",
            "⠲",
            "⠳",
            "⠴",
            "⠵",
            "⠶",
            "⠷",
            "⡰",
            "⡱",
            "⡲",
            "⡳",
            "⡴",
            "⡵",
            "⡶",
            "⡷",
            "⠸",
            "⠹",
            "⠺",
            "⠻",
            "⠼",
            "⠽",
            "⠾",
            "⠿",
            "⡸",
            "⡹",
            "⡺",
            "⡻",
            "⡼",
            "⡽",
            "⡾",
            "⡿",
            "⢀",
            "⢁",
            "⢂",
            "⢃",
            "⢄",
            "⢅",
            "⢆",
            "⢇",
            "⣀",
            "⣁",
            "⣂",
            "⣃",
            "⣄",
            "⣅",
            "⣆",
            "⣇",
            "⢈",
            "⢉",
            "⢊",
            "⢋",
            "⢌",
            "⢍",
            "⢎",
            "⢏",
            "⣈",
            "⣉",
            "⣊",
            "⣋",
            "⣌",
            "⣍",
            "⣎",
            "⣏",
            "⢐",
            "⢑",
            "⢒",
            "⢓",
            "⢔",
            "⢕",
            "⢖",
            "⢗",
            "⣐",
            "⣑",
            "⣒",
            "⣓",
            "⣔",
            "⣕",
            "⣖",
            "⣗",
            "⢘",
            "⢙",
            "⢚",
            "⢛",
            "⢜",
            "⢝",
            "⢞",
            "⢟",
            "⣘",
            "⣙",
            "⣚",
            "⣛",
            "⣜",
            "⣝",
            "⣞",
            "⣟",
            "⢠",
            "⢡",
            "⢢",
            "⢣",
            "⢤",
            "⢥",
            "⢦",
            "⢧",
            "⣠",
            "⣡",
            "⣢",
            "⣣",
            "⣤",
            "⣥",
            "⣦",
            "⣧",
            "⢨",
            "⢩",
            "⢪",
            "⢫",
            "⢬",
            "⢭",
            "⢮",
            "⢯",
            "⣨",
            "⣩",
            "⣪",
            "⣫",
            "⣬",
            "⣭",
            "⣮",
            "⣯",
            "⢰",
            "⢱",
            "⢲",
            "⢳",
            "⢴",
            "⢵",
            "⢶",
            "⢷",
            "⣰",
            "⣱",
            "⣲",
            "⣳",
            "⣴",
            "⣵",
            "⣶",
            "⣷",
            "⢸",
            "⢹",
            "⢺",
            "⢻",
            "⢼",
            "⢽",
            "⢾",
            "⢿",
            "⣸",
            "⣹",
            "⣺",
            "⣻",
            "⣼",
            "⣽",
            "⣾",
            "⣿",
            &final_tick,
        ])
    }

    pub fn dots_circle() -> ProgressStyle {
        dots_circle_with_final("  ")
    }

    pub fn dots_circle_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⢎ ",
            "⠎⠁",
            "⠊⠑",
            "⠈⠱",
            " ⡱",
            "⢀⡰",
            "⢄⡠",
            "⢆⡀",
            &final_tick,
        ])
    }

    pub fn sand() -> ProgressStyle {
        sand_with_final(" ")
    }

    pub fn sand_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⠁",
            "⠂",
            "⠄",
            "⡀",
            "⡈",
            "⡐",
            "⡠",
            "⣀",
            "⣁",
            "⣂",
            "⣄",
            "⣌",
            "⣔",
            "⣤",
            "⣥",
            "⣦",
            "⣮",
            "⣶",
            "⣷",
            "⣿",
            "⡿",
            "⠿",
            "⢟",
            "⠟",
            "⡛",
            "⠛",
            "⠫",
            "⢋",
            "⠋",
            "⠍",
            "⡉",
            "⠉",
            "⠑",
            "⠡",
            "⢁",
            &final_tick,
        ])
    }

    pub fn line() -> ProgressStyle {
        line_with_final(" ")
    }

    pub fn line_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["-", "\\", "|", "/", &final_tick])
    }

    pub fn line2() -> ProgressStyle {
        line2_with_final(" ")
    }

    pub fn line2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["⠂", "-", "–", "—", "–", "-", &final_tick])
    }

    pub fn pipe() -> ProgressStyle {
        pipe_with_final(" ")
    }

    pub fn pipe_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "┤",
            "┘",
            "┴",
            "└",
            "├",
            "┌",
            "┬",
            "┐",
            &final_tick,
        ])
    }

    pub fn simple_dots() -> ProgressStyle {
        simple_dots_with_final("   ")
    }

    pub fn simple_dots_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[".  ", ".. ", "...", "   ", &final_tick])
    }

    pub fn simple_dots_scrolling() -> ProgressStyle {
        simple_dots_scrolling_with_final("   ")
    }

    pub fn simple_dots_scrolling_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            ".  ",
            ".. ",
            "...",
            " ..",
            "  .",
            "   ",
            &final_tick,
        ])
    }

    pub fn star() -> ProgressStyle {
        star_with_final(" ")
    }

    pub fn star_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["✶", "✸", "✹", "✺", "✹", "✷", &final_tick])
    }

    pub fn star2() -> ProgressStyle {
        star2_with_final(" ")
    }

    pub fn star2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["+", "x", "*", &final_tick])
    }

    pub fn flip() -> ProgressStyle {
        flip_with_final(" ")
    }

    pub fn flip_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "_",
            "_",
            "_",
            "-",
            "`",
            "`",
            "'",
            "´",
            "-",
            "_",
            "_",
            "_",
            &final_tick,
        ])
    }

    pub fn hamburger() -> ProgressStyle {
        hamburger_with_final(" ")
    }

    pub fn hamburger_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["☱", "☲", "☴", &final_tick])
    }

    pub fn grow_vertical() -> ProgressStyle {
        grow_vertical_with_final(" ")
    }

    pub fn grow_vertical_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▁",
            "▃",
            "▄",
            "▅",
            "▆",
            "▇",
            "▆",
            "▅",
            "▄",
            "▃",
            &final_tick,
        ])
    }

    pub fn grow_horizontal() -> ProgressStyle {
        grow_horizontal_with_final(" ")
    }

    pub fn grow_horizontal_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▏",
            "▎",
            "▍",
            "▌",
            "▋",
            "▊",
            "▉",
            "▊",
            "▋",
            "▌",
            "▍",
            "▎",
            &final_tick,
        ])
    }

    pub fn balloon() -> ProgressStyle {
        balloon_with_final(" ")
    }

    pub fn balloon_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            " ",
            ".",
            "o",
            "O",
            "@",
            "*",
            " ",
            &final_tick,
        ])
    }

    pub fn balloon2() -> ProgressStyle {
        balloon2_with_final(" ")
    }

    pub fn balloon2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            ".",
            "o",
            "O",
            "°",
            "O",
            "o",
            ".",
            &final_tick,
        ])
    }

    pub fn noise() -> ProgressStyle {
        noise_with_final(" ")
    }

    pub fn noise_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["▓", "▒", "░", &final_tick])
    }

    pub fn bounce() -> ProgressStyle {
        bounce_with_final(" ")
    }

    pub fn bounce_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["⠁", "⠂", "⠄", "⠂", &final_tick])
    }

    pub fn box_bounce() -> ProgressStyle {
        box_bounce_with_final(" ")
    }

    pub fn box_bounce_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["▖", "▘", "▝", "▗", &final_tick])
    }

    pub fn box_bounce2() -> ProgressStyle {
        box_bounce2_with_final(" ")
    }

    pub fn box_bounce2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["▌", "▀", "▐", "▄", &final_tick])
    }

    pub fn triangle() -> ProgressStyle {
        triangle_with_final(" ")
    }

    pub fn triangle_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◢", "◣", "◤", "◥", &final_tick])
    }

    pub fn binary() -> ProgressStyle {
        binary_with_final("      ")
    }

    pub fn binary_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "010010",
            "001100",
            "100101",
            "111010",
            "111101",
            "010111",
            "101011",
            "111000",
            "110011",
            "110101",
            &final_tick,
        ])
    }

    pub fn arc() -> ProgressStyle {
        arc_with_final(" ")
    }

    pub fn arc_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◜", "◠", "◝", "◞", "◡", "◟", &final_tick])
    }

    pub fn circle() -> ProgressStyle {
        circle_with_final(" ")
    }

    pub fn circle_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◡", "⊙", "◠", &final_tick])
    }

    pub fn square_corners() -> ProgressStyle {
        square_corners_with_final(" ")
    }

    pub fn square_corners_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◰", "◳", "◲", "◱", &final_tick])
    }

    pub fn circle_quarters() -> ProgressStyle {
        circle_quarters_with_final(" ")
    }

    pub fn circle_quarters_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◴", "◷", "◶", "◵", &final_tick])
    }

    pub fn circle_halves() -> ProgressStyle {
        circle_halves_with_final(" ")
    }

    pub fn circle_halves_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◐", "◓", "◑", "◒", &final_tick])
    }

    pub fn squish() -> ProgressStyle {
        squish_with_final(" ")
    }

    pub fn squish_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["╫", "╪", &final_tick])
    }

    pub fn toggle() -> ProgressStyle {
        toggle_with_final(" ")
    }

    pub fn toggle_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["⊶", "⊷", &final_tick])
    }

    pub fn toggle2() -> ProgressStyle {
        toggle2_with_final(" ")
    }

    pub fn toggle2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["▫", "▪", &final_tick])
    }

    pub fn toggle3() -> ProgressStyle {
        toggle3_with_final(" ")
    }

    pub fn toggle3_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["□", "■", &final_tick])
    }

    pub fn toggle4() -> ProgressStyle {
        toggle4_with_final(" ")
    }

    pub fn toggle4_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["■", "□", "▪", "▫", &final_tick])
    }

    pub fn toggle5() -> ProgressStyle {
        toggle5_with_final(" ")
    }

    pub fn toggle5_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["▮", "▯", &final_tick])
    }

    pub fn toggle6() -> ProgressStyle {
        toggle6_with_final(" ")
    }

    pub fn toggle6_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["ဝ", "၀", &final_tick])
    }

    pub fn toggle7() -> ProgressStyle {
        toggle7_with_final(" ")
    }

    pub fn toggle7_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["⦾", "⦿", &final_tick])
    }

    pub fn toggle8() -> ProgressStyle {
        toggle8_with_final(" ")
    }

    pub fn toggle8_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◍", "◌", &final_tick])
    }

    pub fn toggle9() -> ProgressStyle {
        toggle9_with_final(" ")
    }

    pub fn toggle9_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["◉", "◎", &final_tick])
    }

    pub fn toggle10() -> ProgressStyle {
        toggle10_with_final(" ")
    }

    pub fn toggle10_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["㊂", "㊀", "㊁", &final_tick])
    }

    pub fn toggle11() -> ProgressStyle {
        toggle11_with_final(" ")
    }

    pub fn toggle11_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["⧇", "⧆", &final_tick])
    }

    pub fn toggle12() -> ProgressStyle {
        toggle12_with_final(" ")
    }

    pub fn toggle12_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["☗", "☖", &final_tick])
    }

    pub fn toggle13() -> ProgressStyle {
        toggle13_with_final(" ")
    }

    pub fn toggle13_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["=", "*", "-", &final_tick])
    }

    pub fn arrow() -> ProgressStyle {
        arrow_with_final(" ")
    }

    pub fn arrow_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "←",
            "↖",
            "↑",
            "↗",
            "→",
            "↘",
            "↓",
            "↙",
            &final_tick,
        ])
    }

    pub fn arrow2() -> ProgressStyle {
        arrow2_with_final("   ")
    }

    pub fn arrow2_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "⬆️ ",
            "↗️ ",
            "➡️ ",
            "↘️ ",
            "⬇️ ",
            "↙️ ",
            "⬅️ ",
            "↖️ ",
            &final_tick,
        ])
    }

    pub fn arrow3() -> ProgressStyle {
        arrow3_with_final("     ")
    }

    pub fn arrow3_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▹▹▹▹▹",
            "▸▹▹▹▹",
            "▹▸▹▹▹",
            "▹▹▸▹▹",
            "▹▹▹▸▹",
            "▹▹▹▹▸",
            &final_tick,
        ])
    }

    pub fn bouncing_bar() -> ProgressStyle {
        bouncing_bar_with_final("      ")
    }

    pub fn bouncing_bar_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "[    ]",
            "[=   ]",
            "[==  ]",
            "[=== ]",
            "[====]",
            "[ ===]",
            "[  ==]",
            "[   =]",
            "[    ]",
            "[   =]",
            "[  ==]",
            "[ ===]",
            "[====]",
            "[=== ]",
            "[==  ]",
            "[=   ]",
            &final_tick,
        ])
    }

    pub fn bouncing_ball() -> ProgressStyle {
        bouncing_ball_with_final("        ")
    }

    pub fn bouncing_ball_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "( ●    )",
            "(  ●   )",
            "(   ●  )",
            "(    ● )",
            "(     ●)",
            "(    ● )",
            "(   ●  )",
            "(  ●   )",
            "( ●    )",
            "(●     )",
            &final_tick,
        ])
    }

    pub fn smiley() -> ProgressStyle {
        smiley_with_final("  ")
    }

    pub fn smiley_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["😄 ", "😝 ", &final_tick])
    }

    pub fn monkey() -> ProgressStyle {
        monkey_with_final("  ")
    }

    pub fn monkey_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["🙈 ", "🙈 ", "🙉 ", "🙊 ", &final_tick])
    }

    pub fn hearts() -> ProgressStyle {
        hearts_with_final("   ")
    }

    pub fn hearts_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "💛 ",
            "💙 ",
            "💜 ",
            "💚 ",
            "❤️ ",
            &final_tick,
        ])
    }

    pub fn clock() -> ProgressStyle {
        clock_with_final("  ")
    }

    pub fn clock_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🕛 ",
            "🕐 ",
            "🕑 ",
            "🕒 ",
            "🕓 ",
            "🕔 ",
            "🕕 ",
            "🕖 ",
            "🕗 ",
            "🕘 ",
            "🕙 ",
            "🕚 ",
            &final_tick,
        ])
    }

    pub fn earth() -> ProgressStyle {
        earth_with_final("  ")
    }

    pub fn earth_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["🌍 ", "🌎 ", "🌏 ", &final_tick])
    }

    pub fn material() -> ProgressStyle {
        material_with_final("                    ")
    }

    pub fn material_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "███████▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "██████████▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "█████████████▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁██████████████▁▁▁▁",
            "▁▁▁██████████████▁▁▁",
            "▁▁▁▁█████████████▁▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁██████████████▁▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁██████████████▁",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁██████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁█████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁████████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁███████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁██████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁████████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁██████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "█▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "██▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "███▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "████▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "█████▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "██████▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "████████▁▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "█████████▁▁▁▁▁▁▁▁▁▁▁",
            "███████████▁▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "████████████▁▁▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "██████████████▁▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁██████████████▁▁▁▁▁",
            "▁▁▁█████████████▁▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁████████████▁▁▁",
            "▁▁▁▁▁▁███████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁█████████▁▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁█████████▁▁",
            "▁▁▁▁▁▁▁▁▁▁█████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁████████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁███████▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁███████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁████",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁███",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁██",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁█",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            "▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁",
            &final_tick,
        ])
    }

    pub fn moon() -> ProgressStyle {
        moon_with_final("  ")
    }

    pub fn moon_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🌑 ",
            "🌒 ",
            "🌓 ",
            "🌔 ",
            "🌕 ",
            "🌖 ",
            "🌗 ",
            "🌘 ",
            &final_tick,
        ])
    }

    pub fn runner() -> ProgressStyle {
        runner_with_final("  ")
    }

    pub fn runner_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["🚶 ", "🏃 ", &final_tick])
    }

    pub fn pong() -> ProgressStyle {
        pong_with_final("          ")
    }

    pub fn pong_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▐⠂       ▌",
            "▐⠈       ▌",
            "▐ ⠂      ▌",
            "▐ ⠠      ▌",
            "▐  ⡀     ▌",
            "▐  ⠠     ▌",
            "▐   ⠂    ▌",
            "▐   ⠈    ▌",
            "▐    ⠂   ▌",
            "▐    ⠠   ▌",
            "▐     ⡀  ▌",
            "▐     ⠠  ▌",
            "▐      ⠂ ▌",
            "▐      ⠈ ▌",
            "▐       ⠂▌",
            "▐       ⠠▌",
            "▐       ⡀▌",
            "▐      ⠠ ▌",
            "▐      ⠂ ▌",
            "▐     ⠈  ▌",
            "▐     ⠂  ▌",
            "▐    ⠠   ▌",
            "▐    ⡀   ▌",
            "▐   ⠠    ▌",
            "▐   ⠂    ▌",
            "▐  ⠈     ▌",
            "▐  ⠂     ▌",
            "▐ ⠠      ▌",
            "▐ ⡀      ▌",
            "▐⠠       ▌",
            &final_tick,
        ])
    }

    pub fn shark() -> ProgressStyle {
        shark_with_final("                ")
    }

    pub fn shark_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▐|\\____________▌",
            "▐_|\\___________▌",
            "▐__|\\__________▌",
            "▐___|\\_________▌",
            "▐____|\\________▌",
            "▐_____|\\_______▌",
            "▐______|\\______▌",
            "▐_______|\\_____▌",
            "▐________|\\____▌",
            "▐_________|\\___▌",
            "▐__________|\\__▌",
            "▐___________|\\_▌",
            "▐____________|\\▌",
            "▐____________/|▌",
            "▐___________/|_▌",
            "▐__________/|__▌",
            "▐_________/|___▌",
            "▐________/|____▌",
            "▐_______/|_____▌",
            "▐______/|______▌",
            "▐_____/|_______▌",
            "▐____/|________▌",
            "▐___/|_________▌",
            "▐__/|__________▌",
            "▐_/|___________▌",
            "▐/|____________▌",
            &final_tick,
        ])
    }

    pub fn dqpb() -> ProgressStyle {
        dqpb_with_final(" ")
    }

    pub fn dqpb_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["d", "q", "p", "b", &final_tick])
    }

    pub fn weather() -> ProgressStyle {
        weather_with_final("   ")
    }

    pub fn weather_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "☀️ ",
            "☀️ ",
            "☀️ ",
            "🌤 ",
            "⛅️ ",
            "🌥 ",
            "☁️ ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "🌨 ",
            "🌧 ",
            "🌨 ",
            "☁️ ",
            "🌥 ",
            "⛅️ ",
            "🌤 ",
            "☀️ ",
            "☀️ ",
            &final_tick,
        ])
    }

    pub fn christmas() -> ProgressStyle {
        christmas_with_final(" ")
    }

    pub fn christmas_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["🌲", "🎄", &final_tick])
    }

    pub fn grenade() -> ProgressStyle {
        grenade_with_final("   ")
    }

    pub fn grenade_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "،  ",
            "′  ",
            " ´ ",
            " ‾ ",
            "  ⸌",
            "  ⸊",
            "  |",
            "  ⁎",
            "  ⁕",
            " ෴ ",
            "  ⁓",
            "   ",
            "   ",
            "   ",
            &final_tick,
        ])
    }

    pub fn point() -> ProgressStyle {
        point_with_final("   ")
    }

    pub fn point_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "∙∙∙",
            "●∙∙",
            "∙●∙",
            "∙∙●",
            "∙∙∙",
            &final_tick,
        ])
    }

    pub fn layer() -> ProgressStyle {
        layer_with_final(" ")
    }

    pub fn layer_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["-", "=", "≡", &final_tick])
    }

    pub fn beta_wave() -> ProgressStyle {
        beta_wave_with_final("       ")
    }

    pub fn beta_wave_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "ρββββββ",
            "βρβββββ",
            "ββρββββ",
            "βββρβββ",
            "ββββρββ",
            "βββββρβ",
            "ββββββρ",
            &final_tick,
        ])
    }

    pub fn finger_dance() -> ProgressStyle {
        finger_dance_with_final("  ")
    }

    pub fn finger_dance_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🤘 ",
            "🤟 ",
            "🖖 ",
            "✋ ",
            "🤚 ",
            "👆 ",
            &final_tick,
        ])
    }

    pub fn fist_bump() -> ProgressStyle {
        fist_bump_with_final("       ")
    }

    pub fn fist_bump_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🤜　　　　🤛 ",
            "🤜　　　　🤛 ",
            "🤜　　　　🤛 ",
            "　🤜　　🤛　 ",
            "　　🤜🤛　　 ",
            "　🤜✨🤛　　 ",
            "🤜　✨　🤛　 ",
            &final_tick,
        ])
    }

    pub fn soccer_header() -> ProgressStyle {
        soccer_header_with_final("             ")
    }

    pub fn soccer_header_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            " 🧑⚽️       🧑 ",
            "🧑  ⚽️      🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑      ⚽️  🧑 ",
            "🧑       ⚽️🧑  ",
            "🧑      ⚽️  🧑 ",
            "🧑     ⚽️   🧑 ",
            "🧑    ⚽️    🧑 ",
            "🧑   ⚽️     🧑 ",
            "🧑  ⚽️      🧑 ",
            &final_tick,
        ])
    }

    pub fn mindblown() -> ProgressStyle {
        mindblown_with_final("  ")
    }

    pub fn mindblown_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "😐 ",
            "😐 ",
            "😮 ",
            "😮 ",
            "😦 ",
            "😦 ",
            "😧 ",
            "😧 ",
            "🤯 ",
            "💥 ",
            "✨ ",
            "　 ",
            "　 ",
            "　 ",
            &final_tick,
        ])
    }

    pub fn speaker() -> ProgressStyle {
        speaker_with_final("  ")
    }

    pub fn speaker_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&["🔈 ", "🔉 ", "🔊 ", "🔉 ", &final_tick])
    }

    pub fn orange_pulse() -> ProgressStyle {
        orange_pulse_with_final("  ")
    }

    pub fn orange_pulse_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 ",
            &final_tick,
        ])
    }

    pub fn blue_pulse() -> ProgressStyle {
        blue_pulse_with_final("  ")
    }

    pub fn blue_pulse_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 ",
            &final_tick,
        ])
    }

    pub fn orange_blue_pulse() -> ProgressStyle {
        orange_blue_pulse_with_final("  ")
    }

    pub fn orange_blue_pulse_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🔸 ",
            "🔶 ",
            "🟠 ",
            "🟠 ",
            "🔶 ",
            "🔹 ",
            "🔷 ",
            "🔵 ",
            "🔵 ",
            "🔷 ",
            &final_tick,
        ])
    }

    pub fn time_travel() -> ProgressStyle {
        time_travel_with_final("  ")
    }

    pub fn time_travel_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "🕛 ",
            "🕚 ",
            "🕙 ",
            "🕘 ",
            "🕗 ",
            "🕖 ",
            "🕕 ",
            "🕔 ",
            "🕓 ",
            "🕒 ",
            "🕑 ",
            "🕐 ",
            &final_tick,
        ])
    }

    pub fn aesthetic() -> ProgressStyle {
        aesthetic_with_final("       ")
    }

    pub fn aesthetic_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            "▰▱▱▱▱▱▱",
            "▰▰▱▱▱▱▱",
            "▰▰▰▱▱▱▱",
            "▰▰▰▰▱▱▱",
            "▰▰▰▰▰▱▱",
            "▰▰▰▰▰▰▱",
            "▰▰▰▰▰▰▰",
            "▰▱▱▱▱▱▱",
            &final_tick,
        ])
    }

    pub fn dwarf_fortress() -> ProgressStyle {
        dwarf_fortress_with_final("            ")
    }

    pub fn dwarf_fortress_with_final(final_tick: &str) -> ProgressStyle {
        ProgressStyle::default_spinner().tick_strings(&[
            " ██████£££  ",
            "☺██████£££  ",
            "☺██████£££  ",
            "☺▓█████£££  ",
            "☺▓█████£££  ",
            "☺▒█████£££  ",
            "☺▒█████£££  ",
            "☺░█████£££  ",
            "☺░█████£££  ",
            "☺ █████£££  ",
            " ☺█████£££  ",
            " ☺█████£££  ",
            " ☺▓████£££  ",
            " ☺▓████£££  ",
            " ☺▒████£££  ",
            " ☺▒████£££  ",
            " ☺░████£££  ",
            " ☺░████£££  ",
            " ☺ ████£££  ",
            "  ☺████£££  ",
            "  ☺████£££  ",
            "  ☺▓███£££  ",
            "  ☺▓███£££  ",
            "  ☺▒███£££  ",
            "  ☺▒███£££  ",
            "  ☺░███£££  ",
            "  ☺░███£££  ",
            "  ☺ ███£££  ",
            "   ☺███£££  ",
            "   ☺███£££  ",
            "   ☺▓██£££  ",
            "   ☺▓██£££  ",
            "   ☺▒██£££  ",
            "   ☺▒██£££  ",
            "   ☺░██£££  ",
            "   ☺░██£££  ",
            "   ☺ ██£££  ",
            "    ☺██£££  ",
            "    ☺██£££  ",
            "    ☺▓█£££  ",
            "    ☺▓█£££  ",
            "    ☺▒█£££  ",
            "    ☺▒█£££  ",
            "    ☺░█£££  ",
            "    ☺░█£££  ",
            "    ☺ █£££  ",
            "     ☺█£££  ",
            "     ☺█£££  ",
            "     ☺▓£££  ",
            "     ☺▓£££  ",
            "     ☺▒£££  ",
            "     ☺▒£££  ",
            "     ☺░£££  ",
            "     ☺░£££  ",
            "     ☺ £££  ",
            "      ☺£££  ",
            "      ☺£££  ",
            "      ☺▓££  ",
            "      ☺▓££  ",
            "      ☺▒££  ",
            "      ☺▒££  ",
            "      ☺░££  ",
            "      ☺░££  ",
            "      ☺ ££  ",
            "       ☺££  ",
            "       ☺££  ",
            "       ☺▓£  ",
            "       ☺▓£  ",
            "       ☺▒£  ",
            "       ☺▒£  ",
            "       ☺░£  ",
            "       ☺░£  ",
            "       ☺ £  ",
            "        ☺£  ",
            "        ☺£  ",
            "        ☺▓  ",
            "        ☺▓  ",
            "        ☺▒  ",
            "        ☺▒  ",
            "        ☺░  ",
            "        ☺░  ",
            "        ☺   ",
            "        ☺  &",
            "        ☺ ☼&",
            "       ☺ ☼ &",
            "       ☺☼  &",
            "      ☺☼  & ",
            "      ‼   & ",
            "     ☺   &  ",
            "    ‼    &  ",
            "   ☺    &   ",
            "  ‼     &   ",
            " ☺     &    ",
            "‼      &    ",
            "      &     ",
            "      &     ",
            "     &   ░  ",
            "     &   ▒  ",
            "    &    ▓  ",
            "    &    £  ",
            "   &    ░£  ",
            "   &    ▒£  ",
            "  &     ▓£  ",
            "  &     ££  ",
            " &     ░££  ",
            " &     ▒££  ",
            "&      ▓££  ",
            "&      £££  ",
            "      ░£££  ",
            "      ▒£££  ",
            "      ▓£££  ",
            "      █£££  ",
            "     ░█£££  ",
            "     ▒█£££  ",
            "     ▓█£££  ",
            "     ██£££  ",
            "    ░██£££  ",
            "    ▒██£££  ",
            "    ▓██£££  ",
            "    ███£££  ",
            "   ░███£££  ",
            "   ▒███£££  ",
            "   ▓███£££  ",
            "   ████£££  ",
            "  ░████£££  ",
            "  ▒████£££  ",
            "  ▓████£££  ",
            "  █████£££  ",
            " ░█████£££  ",
            " ▒█████£££  ",
            " ▓█████£££  ",
            " ██████£££  ",
            " ██████£££  ",
            &final_tick,
        ])
    }
}
