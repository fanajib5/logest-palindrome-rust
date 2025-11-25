fn main() {
    println!("{}", longest_palindrome("abccccdd")); // Output: "dcccd"
    println!("{}", longest_palindrome("a"));        // Output: "a"
    println!("{}", longest_palindrome("aAbBABba")); // Output: "bBABb"
    println!("{}", longest_palindrome("bananas"));  // Output: "anana"
    println!("{}", longest_palindrome("kasurinirusak")); // Output: "kasurinirusak"
    println!("{}", longest_palindrome("ababbbab")); // Output: "babbbab"
    println!("{}", longest_palindrome("zzbananaxxbaananaayyaaananaaads")); // Output: "aaananaaa"
    println!("{}", longest_palindrome("xvabaxvxxbananayybbananabbvvzkkzaaananaaazkkz")); // Output: "zkkzaaananaaazkkz"
}

fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let chars_len = chars.len();

    if chars_len == 0 {
      return String::new();
    }

    if is_palindrome(s) {
      return s.to_string();
    }

    let mut best_left = 0;
    let mut best_right = 0;

    for i in 0..chars_len {
        // palindrome ganjil (center = i)
        let (l_odd, r_odd) = expand_around_center(&chars, i as isize, i as isize);

        // palindrome genap (center= i, i+1)
        let (l_even, r_even) = expand_around_center(&chars, i as isize, (i + 1) as isize);

        // memilih mana yang lebih panjang
        let (new_left, new_right) = if (r_odd - l_odd + 1) > (r_even - l_even + 1) {
            (l_odd, r_odd)
        } else {
            (l_even, r_even)
        };

        // update best_left, best_right
        let new_len = new_right - new_left + 1;
        let best_len = best_right - best_left + 1;

        if new_len > best_len {
            best_left = new_left;
            best_right = new_right;
        }
    }

    chars[best_left as usize..=best_right as usize].iter().collect()
}


fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();

    if chars.is_empty() {
        return false;
    }

    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
      if chars[left] != chars[right] {
        return false;
      }

      left += 1;
      right -= 1;
    }

    true
}

fn expand_around_center(chars: &[char], mut left: isize, mut right: isize) -> (isize, isize) {
    let n = chars.len() as isize;

    while left >= 0 && right < n && chars[left as usize] == chars[right as usize] {
        left -= 1;
        right += 1;
    }

    // hasil palindrome adalah (left+1, right-1)
    (left+1, right-1)
}