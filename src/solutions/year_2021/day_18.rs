#![allow(dead_code, unused_variables)]

pub fn part_one(input: &str) -> i64 {
    let result: Option<Expression> =
        input
            .trim()
            .lines()
            .map(Expression::new)
            .fold(None, |accum, next| {
                // next.print();
                if let Some(mut accum) = accum {
                    accum.add(next);
                    accum.reduce();
                    Some(accum)
                } else {
                    Some(next)
                }
            });

    result
        .map(|r| {
            // r.print();
            r.magnitude()
        })
        .unwrap_or_default()
}

pub fn part_two(input: &str) -> u64 {
    let _lines = input.trim().lines();

    0
}

struct Expression {
    nodes: Vec<Node>,
    start: usize,
}

impl Expression {
    fn new(line: &str) -> Self {
        let mut nodes = Vec::new();

        let tokens = line.bytes().filter_map(|t| Token::try_from(t).ok());
        let mut parse_stack = Vec::new();

        for token in tokens {
            let node = match token {
                Token::Close => match parse_stack.pop().zip(parse_stack.pop()) {
                    Some((two, one)) => {
                        parse_stack.push(nodes.len());
                        Node::Pair(one, two)
                    }
                    _ => unreachable!(),
                },
                Token::Number(n) => {
                    parse_stack.push(nodes.len());
                    Node::Literal(n)
                }
            };

            nodes.push(node)
        }

        let start = nodes.len() - 1;

        Expression { nodes, start }
    }

    fn print(&self) {
        print_node(&self.nodes, self.start);
        println!();
    }

    fn magnitude(&self) -> i64 {
        0
    }

    fn add(&mut self, other: Self) {
        let len = self.nodes.len();
        self.nodes.reserve(other.nodes.len());
        for node in other.nodes {
            let node = match node {
                Node::Literal(_) => node,
                Node::Pair(a, b) => Node::Pair(a + len, b + len),
            };

            self.nodes.push(node);
        }
        let other_start = other.start + len;
        let pair = Node::Pair(self.start, other_start);

        self.start = self.nodes.len();
        self.nodes.push(pair);
    }

    fn reduce(&mut self) {
        let explode = false;
        let split = false;
        loop {
            if explode {
                continue;
            }
            if split {
                continue;
            }

            break;
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Explode {
    Searching,
    Done,
}

fn explode_node(
    nodes: &mut Vec<Node>,
    node: usize,
    depth: usize,
    state: (Option<Explode>, Option<Explode>),
) -> (Option<Explode>, Option<Explode>) {
    match nodes[node] {
        Node::Literal(_) => return (None, None),
        Node::Pair(one, two) => match (nodes[one], nodes[two]) {
            (Node::Literal(_), Node::Literal(_)) => todo!(),
            (Node::Literal(_), Node::Pair(_, _)) => todo!(),
            (Node::Pair(_, _), Node::Literal(_)) => todo!(),
            (Node::Pair(_, _), Node::Pair(_, _)) => todo!(),
        },
    }
}

fn print_node(nodes: &[Node], node: usize) {
    match nodes[node] {
        Node::Literal(n) => eprint!("{}", n),
        Node::Pair(a, b) => {
            eprint!("[");
            print_node(nodes, a);
            eprint!(",");
            print_node(nodes, b);
            eprint!("]");
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Node {
    Literal(i64),
    Pair(usize, usize),
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Close,
    Number(i64),
}

struct TokenParseErr;

impl TryFrom<u8> for Token {
    type Error = TokenParseErr;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let val = match value {
            b']' => Token::Close,
            b'0'..=b'9' => Token::Number((value - b'0') as i64),
            _ => return Err(TokenParseErr),
        };

        Ok(val)
    }
}

#[test]
fn test() {
    let input = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

    assert_eq!(4140, part_one(input));
    assert_eq!(0, part_two(input));
}
