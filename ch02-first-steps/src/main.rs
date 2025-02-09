/// Safely adds two i32 integers with overflow protection
/// Returns `Some(sum)` if no overflow occurs, `None` otherwise
///
/// # Examples
///
/// ```
/// assert_eq!(add(10, 20), Some(30));
/// assert_eq!(add(i32::MAX, 1), None);
/// ```
fn add(a: i32, b: i32) -> Option<i32> {
    a.checked_add(b)
}

fn main() {
    // Use consistent type annotations and const where possible
    const A: i32 = 10;
    const B: i32 = 20;
    const C: i32 = 30;
    const D: i32 = 30;

    // Handle potential overflows at each step
    let first_sum = match add(A, B) {
        Some(sum) => sum,
        None => {
            eprintln!("Overflow occurred in first addition");
            return;
        }
    };

    let second_sum = match add(C, D) {
        Some(sum) => sum,
        None => {
            eprintln!("Overflow occurred in second addition");
            return;
        }
    };

    match add(first_sum, second_sum) {
        Some(result) => println!("(a + b) + (c + d) = {}", result),
        None => eprintln!("Overflow occurred in final addition"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(10, 20), Some(30));
        assert_eq!(add(i32::MAX, 1), None);
        assert_eq!(add(i32::MIN, -1), None);
    }
}
