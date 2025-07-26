use super::*;
use crate::win_info::WinInfo;

pub fn fuzzy_match(input: String, windows: Vec<WinInfo>) -> Option<WinInfo> {
    let input = input.to_lowercase();
    let mut best_match: Option<WinInfo> = None;
    let mut highest_score = 0;

    for window in windows {
        let title = window.title.to_lowercase();
        let exe_name = window.exe_name.to_lowercase();

        let title_score = calc_fuzzy_score(&input, &title);
        let exe_name_score = calc_fuzzy_score(&input, &exe_name);
        let score = title_score.max(exe_name_score);

        if score > highest_score {
            highest_score = score;
            best_match = Some(window);
        }

    };
    best_match
}

pub fn calc_fuzzy_score(pattern: &str, target: &str) -> i32 {
    let mut score = 0;
    let mut pattern_chars = pattern.chars().map(|c| c.to_ascii_lowercase());
    let mut prev_match_index = None;
    let mut pattern_char = pattern_chars.next();

    if pattern_char.is_none() {
        return 0;
    }

    for (i, target_char) in target.chars().map(|c| c.to_ascii_lowercase()).enumerate() {
        if let Some(pc) = pattern_char {
            if target_char == pc {
                if let Some(prev) = prev_match_index {
                    if i == prev + 1 {
                        score += 10;
                    } else {
                        score += 5;
                    }
                } else {
                    score += 5;
                }

                prev_match_index = Some(i);
                pattern_char = pattern_chars.next();

                if pattern_char.is_none() {
                    break;
                }
            } else if prev_match_index.is_some() {
                score -= 1;
            }
        }
    }

    if pattern_char.is_none() {
        score
    } else {
        0
    }
}
