use std::collections::BinaryHeap;

pub fn part_one(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut error = 0;
    let mut stack = Vec::new();
    'line: for line in lines {
        stack.clear();
        for chunk in line.chars().filter_map(|c| Chunk::try_from(c).ok()) {
            match chunk {
                Chunk::Open(kind) => stack.push(kind),
                Chunk::Close(closer) => match stack.pop() {
                    Some(opener) if opener != closer => {
                        error += closer.invalid_score();
                        continue 'line;
                    }
                    _ => (),
                },
            }
        }
    }

    error
}

pub fn part_two(input: &str) -> u64 {
    let lines = input.trim().lines();

    let mut scores = BinaryHeap::new();
    let mut stack = Vec::new();
    'line: for line in lines {
        stack.clear();
        for chunk in line.chars().filter_map(|c| Chunk::try_from(c).ok()) {
            match chunk {
                Chunk::Open(kind) => stack.push(kind),
                Chunk::Close(closer) => match stack.pop() {
                    Some(opener) if opener != closer => {
                        continue 'line;
                    }
                    _ => (),
                },
            }
        }

        let mut score = 0;
        while let Some(stacker) = stack.pop() {
            score *= 5;
            score += stacker.closing_score();
        }

        scores.push(score);
    }

    let mid_score = scores.len() / 2;
    let mut n = 0;
    while let Some(next) = scores.pop() {
        if n == mid_score {
            return next;
        }
        n += 1;
    }

    0
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum ChunkType {
    Round,
    Square,
    Curly,
    Angled,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Chunk {
    Open(ChunkType),
    Close(ChunkType),
}

struct ChunkParseError;

impl TryFrom<char> for Chunk {
    type Error = ChunkParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value {
            '(' => Chunk::Open(ChunkType::Round),
            ')' => Chunk::Close(ChunkType::Round),
            '[' => Chunk::Open(ChunkType::Square),
            ']' => Chunk::Close(ChunkType::Square),
            '{' => Chunk::Open(ChunkType::Curly),
            '}' => Chunk::Close(ChunkType::Curly),
            '<' => Chunk::Open(ChunkType::Angled),
            '>' => Chunk::Close(ChunkType::Angled),
            _ => return Err(ChunkParseError),
        };

        Ok(res)
    }
}

impl ChunkType {
    fn invalid_score(self) -> u64 {
        match self {
            ChunkType::Round => 3,
            ChunkType::Square => 57,
            ChunkType::Curly => 1197,
            ChunkType::Angled => 25137,
        }
    }

    fn closing_score(self) -> u64 {
        match self {
            ChunkType::Round => 1,
            ChunkType::Square => 2,
            ChunkType::Curly => 3,
            ChunkType::Angled => 4,
        }
    }
}

#[test]
fn test() {
    let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    assert_eq!(26397, part_one(input));
    assert_eq!(288957, part_two(input));
}
