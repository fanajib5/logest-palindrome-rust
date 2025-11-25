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
