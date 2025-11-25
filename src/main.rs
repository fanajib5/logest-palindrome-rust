fn main() {
    println!("Hello, world!");
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
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

fn expand_around_center(chars: &Vec<char>, mut left: isize, mut right: isize) -> (usize, usize) {
    let n = chars.len() as isize;

    while left >= 0 && right < n && chars[left as usize] == chars[right as usize] {
        left -= 1;
        right += 1;
    }

    // hasil palindrome adalah (left+1, right-1)
    ((left+1) as usize, (right-1) as usize)
}