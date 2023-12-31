fn is_vowel(ch: char) -> bool {
    matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u')
}

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        // 确保索引在合理范围内，并转换为 usize
        let left_idx = left as usize;
        let right_idx = (right as usize).min(words.len().saturating_sub(1));

        for word in words.iter().take(right_idx + 1).skip(left_idx) {
            if let Some(first_char) = word.chars().next() {
                if let Some(last_char) = word.chars().last() {
                    if is_vowel(first_char) && is_vowel(last_char) {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
