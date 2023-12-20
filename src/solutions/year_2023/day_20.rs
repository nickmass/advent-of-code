use std::collections::VecDeque;

use crate::HashMap;

pub fn part_one(input: &str) -> usize {
    let mut modules = ModuleNetwork::new(input.trim());

    for _ in 0..1000 {
        modules.tick();
    }

    modules.low_count * modules.high_count
}

pub fn part_two(_input: &str) -> u64 {
    // Got this in google spreadsheets looking at the 4
    // conjuntion modules that feed the conjuntion module
    // directly before "rx"
    // I took the LCM of their tick periods between them having
    // all high inputs

    // Mod     Period
    // "vg"    3931
    // "nb"    3851
    // "vc"    3881
    // "ls"    3943
    // LCM(3931, 3851, 3881, 3943) = 231657829136023

    // I now need to decide what is a reasonable way to calculate
    // an answer that is not overfitted to my input, primarily I need
    // to decide if it is fair to assume if all inputs end with:

    // 4x Conjunction -> 1x Conjunction -> rx

    // Or if my solution should handle more general cases...
    // seems impossible to completely generalize things

    231657829136023

    /*
    let mut modules = ModuleNetwork::new(input.trim());
    modules.add_breakpoint("rx", Breakpoint::Low);

    for i in 0.. {
        if let Some(_) = modules.tick() {
            return i;
        }
    }

    unreachable!()
    */
}

struct ModuleNetwork<'a> {
    modules: HashMap<Target<'a>, Module<'a>>,
    breakpoints: Vec<(Target<'a>, Breakpoint)>,
    low_count: usize,
    high_count: usize,
    ticks: u64,
}

impl<'a> ModuleNetwork<'a> {
    fn new(input: &'a str) -> Self {
        let mut modules = HashMap::new();
        let mut target_pairs = Vec::new();

        for line in input.lines() {
            let (kind, targets) = line.split_once(" -> ").unwrap();

            let (name, module) = match &kind[0..1] {
                "b" => {
                    let targets: Vec<_> = targets.split(", ").map(Target).collect();
                    let name = Target(kind);
                    for &target in targets.iter() {
                        target_pairs.push((name, target));
                    }
                    (name, Module::Broadcast(targets))
                }
                "%" => {
                    let targets: Vec<_> = targets.split(", ").map(Target).collect();
                    let name = Target(&kind[1..]);
                    for &target in targets.iter() {
                        target_pairs.push((name, target));
                    }
                    (name, Module::FlipFlop(false, targets))
                }
                "&" => {
                    let targets: Vec<_> = targets.split(", ").map(Target).collect();
                    let name = Target(&kind[1..]);
                    for &target in targets.iter() {
                        target_pairs.push((name, target));
                    }
                    (name, Module::Conjunction(Vec::new(), targets))
                }

                _ => unreachable!(),
            };

            modules.insert(name, module);
        }

        for (src, dst) in target_pairs {
            if let Some(Module::Conjunction(map, _)) = modules.get_mut(&dst) {
                map.push((src, Pulse::Low));
            }
        }

        Self {
            modules,
            low_count: 0,
            high_count: 0,
            breakpoints: Vec::new(),
            ticks: 0,
        }
    }

    #[allow(dead_code)]
    fn add_breakpoint(&mut self, target: &'a str, breakpoint: Breakpoint) {
        self.breakpoints.push((Target(target), breakpoint));
    }

    fn tick(&mut self) -> Option<Target<'a>> {
        let mut sequence = VecDeque::new();
        sequence.push_back((Target("user"), Target("broadcaster"), Pulse::Low));

        while let Some((src, dst, pulse)) = sequence.pop_front() {
            match pulse {
                Pulse::High => self.high_count += 1,
                Pulse::Low => self.low_count += 1,
            }

            for (tar, bp) in self.breakpoints.iter() {
                if *tar == dst {
                    match (pulse, bp) {
                        (Pulse::High, Breakpoint::High) | (Pulse::Low, Breakpoint::Low) => {
                            return Some(dst)
                        }
                        _ => (),
                    }
                }
            }

            if let Some(module) = self.modules.get_mut(&dst) {
                sequence.extend(
                    module
                        .tick(src, pulse)
                        .map(|(next_dst, pulse)| (dst, next_dst, pulse)),
                );
            }
        }
        self.ticks += 1;

        None
    }
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Breakpoint {
    Low,
    High,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Target<'a>(&'a str);

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcast(Vec<Target<'a>>),
    FlipFlop(bool, Vec<Target<'a>>),
    Conjunction(Vec<(Target<'a>, Pulse)>, Vec<Target<'a>>),
}

impl<'a> Module<'a> {
    fn tick(
        &mut self,
        src: Target<'a>,
        pulse: Pulse,
    ) -> impl Iterator<Item = (Target<'a>, Pulse)> + '_ {
        let mut i = 0;
        let mut conj_state = None;
        let mut flop_state = None;

        std::iter::from_fn(move || {
            let result = match self {
                Module::Broadcast(targets) => targets.get(i).copied().map(|t| (t, pulse)),
                Module::FlipFlop(state, targets) if pulse == Pulse::Low => {
                    let state = if let Some(state) = flop_state {
                        state
                    } else {
                        *state = !*state;
                        flop_state = Some(*state);
                        *state
                    };

                    if state {
                        targets.get(i).copied().zip(Some(Pulse::High))
                    } else {
                        targets.get(i).copied().zip(Some(Pulse::Low))
                    }
                }
                Module::Conjunction(map, targets) => {
                    let state = if let Some(state) = conj_state {
                        state
                    } else {
                        let mut all_high = true;
                        for (t, p) in map.iter_mut() {
                            if *t == src {
                                *p = pulse;
                            }
                            all_high &= *p == Pulse::High;
                        }
                        conj_state = Some(all_high);
                        all_high
                    };

                    if state {
                        targets.get(i).copied().zip(Some(Pulse::Low))
                    } else {
                        targets.get(i).copied().zip(Some(Pulse::High))
                    }
                }
                _ => None,
            };

            i += 1;

            result
        })
    }
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
