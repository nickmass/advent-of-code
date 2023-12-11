use crate::HashSet;

pub fn part_one(input: &str) -> usize {
    solve::<4>(input.trim())
}

pub fn part_two(input: &str) -> usize {
    solve::<14>(input.trim())
}

fn solve<const N: usize>(input: &str) -> usize {
    let mut tokens = HashSet::<u8>::with_capacity(N);
    for (idx, window) in input.as_bytes().windows(N).enumerate() {
        tokens.extend(window);
        if tokens.len() == N {
            return idx + N;
        }
        tokens.clear();
    }

    0
}

#[test]
fn test() {
    let test_cases = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    for (input, one, two) in test_cases {
        assert_eq!(one, part_one(input), "input: {}", input);
        assert_eq!(two, part_two(input), "input: {}", input);
    }
}
