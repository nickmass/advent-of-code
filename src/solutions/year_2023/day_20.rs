use std::collections::VecDeque;

use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let mut modules = ModuleNetwork::new(input.trim());

    for _ in 0..1000 {
        modules.tick();
    }

    modules.low_count * modules.high_count
}

pub fn part_two(input: &str) -> u64 {
    let mut modules = ModuleNetwork::new(input.trim());
    modules.time_to_pulse("rx")
}

struct ModuleNetwork<'a> {
    modules: HashMap<Target<'a>, Module<'a>>,
    targets: Vec<Target<'a>>,
    sequence: VecDeque<(Target<'a>, Target<'a>, Pulse)>,
    low_count: usize,
    high_count: usize,
    ticks: u64,
}

impl<'a> ModuleNetwork<'a> {
    fn new(input: &'a str) -> Self {
        let mut modules = HashMap::new();
        let mut target_pairs = Vec::new();
        let mut all_targets = Vec::new();

        for line in input.lines() {
            let (kind, targets) = line.split_once(" -> ").unwrap();
            let targets = targets.split(", ").map(Target);

            let name = kind.trim_start_matches(['%', '&']);
            let name = Target(name);
            let targ_start = all_targets.len();
            for target in targets {
                all_targets.push(target);
                target_pairs.push((name, target));
            }
            let target_id = TargetsId(targ_start, all_targets.len());

            let module = match &kind[0..1] {
                "b" => Module::Broadcast(target_id),
                "%" => Module::FlipFlop(false, target_id),
                "&" => Module::Conjunction(Vec::new(), target_id),
                _ => unreachable!(),
            };

            modules.insert(name, module);
        }

        for (src, dst) in target_pairs {
            if let Some(Module::Conjunction(map, _)) = modules.get_mut(&dst) {
                map.push((src, Pulse::Low, 0));
            }
        }

        Self {
            modules,
            targets: all_targets,
            sequence: VecDeque::new(),
            low_count: 0,
            high_count: 0,
            ticks: 0,
        }
    }

    fn tick(&mut self) {
        self.ticks += 1;
        self.sequence.clear();
        self.sequence
            .push_back((Target("user"), Target("broadcaster"), Pulse::Low));

        while let Some((src, dst, pulse)) = self.sequence.pop_front() {
            match pulse {
                Pulse::High => self.high_count += 1,
                Pulse::Low => self.low_count += 1,
            }

            if let Some(module) = self.modules.get_mut(&dst) {
                self.sequence
                    .extend(module.tick(self.ticks, src, pulse).into_iter().flat_map(
                        |(targets, pulse)| {
                            self.targets[targets.0..targets.1]
                                .into_iter()
                                .copied()
                                .map(move |next_dst| (dst, next_dst, pulse))
                        },
                    ));
            }
        }
    }

    fn time_to_pulse(&mut self, target: &str) -> u64 {
        let target = Target(target);

        let mut watching = None;

        for (name, m) in self.modules.iter() {
            if let Module::Conjunction(_, targets) = m {
                for &t in self.targets[targets.0..targets.1].iter() {
                    if t == target {
                        watching = Some(*name);
                        break;
                    }
                }
            }
        }

        let watching = watching.unwrap();
        let mut child_cycles = HashMap::new();

        'outer: loop {
            self.tick();
            if let Some(Module::Conjunction(children, _)) = self.modules.get(&watching) {
                for &(child, _state, tick) in children.iter() {
                    if tick != 0 && !child_cycles.contains_key(&child) {
                        child_cycles.insert(child, tick);

                        if child_cycles.len() == children.len() {
                            break 'outer;
                        }
                    }
                }
            }
        }

        lcm_iter(child_cycles.into_values())
    }
}

#[derive(Debug, Copy, Clone)]
struct TargetsId(usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Target<'a>(&'a str);

#[derive(Debug)]
enum Module<'a> {
    Broadcast(TargetsId),
    FlipFlop(bool, TargetsId),
    Conjunction(Vec<(Target<'a>, Pulse, u64)>, TargetsId),
}

impl<'a> Module<'a> {
    fn tick(&mut self, tick: u64, src: Target<'a>, pulse: Pulse) -> Option<(TargetsId, Pulse)> {
        match self {
            Module::Broadcast(targets) => Some((*targets, pulse)),
            Module::FlipFlop(state, targets) if pulse == Pulse::Low => {
                *state = !*state;

                if *state {
                    Some((*targets, Pulse::High))
                } else {
                    Some((*targets, Pulse::Low))
                }
            }
            Module::Conjunction(map, targets) => {
                let mut all_high = true;
                for (t, p, tk) in map.iter_mut() {
                    if *t == src {
                        *p = pulse;
                        if pulse == Pulse::High {
                            *tk = tick;
                        }
                    }
                    all_high &= *p == Pulse::High;
                }

                if all_high {
                    Some((*targets, Pulse::Low))
                } else {
                    Some((*targets, Pulse::High))
                }
            }
            _ => None,
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut n = a.min(b);
    while n > 1 {
        if a % n == 0 && b % n == 0 {
            return n;
        }
        n -= 1;
    }

    return 1;
}

fn lcm_iter<I: IntoIterator<Item = u64>>(iter: I) -> u64 {
    iter.into_iter().fold(1, |acc, n| (acc * n) / gcd(acc, n))
}

#[test]
fn test() {
    let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

    assert_eq!(32000000, part_one(input));

    let input = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

    assert_eq!(11687500, part_one(input));
}
