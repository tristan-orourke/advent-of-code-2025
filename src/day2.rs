fn is_doubled_id(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();
    // Number with odd number of digits cannot have the same digits duplicated
    if len % 2 != 0 {
        false
    } else {
        s[0..len / 2] == s[len / 2..]
    }
}

/// Splits a string into substrings of exactly `n` characters.
/// Returns empty iterator if `n` is 0 or greater than string length.
fn chunks_of_n(s: &str, n: usize) -> impl Iterator<Item = &str> + '_ {
    (0..s.len()).step_by(n).map(move |start| {
        let end = (start + n).min(s.len());
        &s[start..end]
    })
}

fn is_n_times_repeated(n: usize, s: &str) -> bool {
    let len = s.len();
    if len % n != 0 {
        false
    } else {
        let chunk_length = len / n;
        let mut chunks = chunks_of_n(s, chunk_length);
        let first = chunks.next().unwrap();
        chunks.all(|chunk| chunk == first)
    }
}

fn is_repeated_id(id: u64) -> bool {
    let s = id.to_string();
    if s.len() < 2 {
        false
    } else {
        (2..=s.len()).any(|n| is_n_times_repeated(n, &s))
    }
}

fn expand_range(s: &str) -> Result<std::ops::RangeInclusive<u64>, Box<dyn std::error::Error>> {
    let (a, b) = match s.split_once("-") {
        Some((a, b)) => (a, b),
        None => return Err("s must be of format x-b, instead was {s}".into()),
    };
    let x: u64 = a.parse()?;
    let y: u64 = b.parse()?;
    Ok(x..=y)
}
// Returns a tuple, first is counting doubled ids, second is counting any duplication
pub fn process_ids_sum_duplicates(s: &str) -> (u64, u64) {
    let invalid_count = s
        .split(",")
        .fold((0, 0), |(a, b), rng| match expand_range(rng) {
            Ok(range) => (
                a + range.clone().filter(|&n| is_doubled_id(n)).sum::<u64>(),
                b + range.filter(|&n| is_repeated_id(n)).sum::<u64>(),
            ),
            Err(e) => panic!("malformed input string, error: {e}"),
        });
    invalid_count
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    use std::fs;

    #[test]
    fn test_is_doubled() {
        assert_eq!(is_doubled_id(1212), true);
        assert_eq!(is_doubled_id(121), false);
        assert_eq!(is_doubled_id(1234), false);
    }

    #[test]
    fn test_expand_range() {
        assert_eq!(expand_range("110-2000").unwrap(), 110..=2000);
        assert!(expand_range("1234").is_err());
        assert!(expand_range("abcd").is_err());
        assert!(expand_range("12-ab").is_err());
        assert!(expand_range("12-20-30").is_err());
    }

    #[test]
    fn process_test_input() {
        let s = fs::read_to_string("./src/data/day2_test.txt").unwrap();
        s.trim().split(",").for_each(|r| {
            expand_range(r)
                .unwrap()
                .filter(|&n| is_doubled_id(n))
                .for_each(|n| println!("{n}"))
        });
        assert_eq!(process_ids_sum_duplicates(s.trim()), (1227775554, 4174379265));
    }
}
