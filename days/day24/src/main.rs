use std::{cell::RefCell, cmp::Reverse};

use common::read_input;

#[derive(Debug, PartialEq)]
struct Wire {
    name: String,
    value: Option<bool>,
}

impl From<&str> for Wire {
    fn from(input: &str) -> Self {
        let (name, value) = input.split_once(": ").unwrap();
        let value = match value {
            "1" => true,
            "0" => false,
            s => panic!("Unrecognized wire value: {s}"),
        };
        Self::new(name.to_string()).with_value(value)
    }
}

impl Wire {
    fn new(name: String) -> Self {
        Self { name, value: None }
    }

    fn with_value(self, value: bool) -> Self {
        Self {
            value: Some(value),
            ..self
        }
    }
}

#[derive(Debug, PartialEq)]
enum GateKind {
    AND,
    XOR,
    OR,
}

impl From<&str> for GateKind {
    fn from(value: &str) -> Self {
        match value.trim() {
            "AND" => Self::AND,
            "XOR" => Self::XOR,
            "OR" => Self::OR,
            s => panic!("Unrecognized gate kind: {s}"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Gate {
    kind: GateKind,
    input_1: String,
    input_2: String,
    output: String,
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        let (inputs, output) = value.trim().split_once(" -> ").unwrap();
        let mut inputs = inputs.trim().split_whitespace();
        let input_1 = inputs.next().unwrap().to_string();
        let kind = inputs.next().unwrap().into();
        let input_2 = inputs.next().unwrap().to_string();
        Self::new(kind, input_1, input_2, output.to_string())
    }
}

impl Gate {
    fn new(kind: GateKind, input_1: String, input_2: String, output: String) -> Self {
        Self {
            kind,
            input_1,
            input_2,
            output,
        }
    }

    fn apply(&self, wires: &[RefCell<Wire>]) -> Option<bool> {
        let input_1 = wires
            .iter()
            .find(|w| w.borrow().name == self.input_1)
            .unwrap();
        let input_2 = wires
            .iter()
            .find(|w| w.borrow().name == self.input_2)
            .unwrap();
        if let Some(input_1) = input_1.borrow().value {
            if let Some(input_2) = input_2.borrow().value {
                match self.kind {
                    GateKind::AND => Some(input_1 && input_2),
                    GateKind::XOR => Some(input_1 != input_2),
                    GateKind::OR => Some(input_1 || input_2),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Circuit {
    wires: Vec<RefCell<Wire>>,
    gates: Vec<Gate>,
}

impl From<&str> for Circuit {
    fn from(value: &str) -> Self {
        let (wires, gates) = value.trim().split_once("\n\n").unwrap();
        let mut wires = wires
            .lines()
            .map(|l| RefCell::<Wire>::new(l.trim().into()))
            .collect::<Vec<_>>();
        let gates = gates.lines().map(|l| l.trim().into()).collect::<Vec<_>>();
        let mut other_wires: Vec<RefCell<Wire>> = vec![];
        gates
            .iter()
            .flat_map(|gate: &Gate| {
                vec![
                    gate.input_1.clone(),
                    gate.input_2.clone(),
                    gate.output.clone(),
                ]
            })
            .for_each(|w| {
                if wires.iter().find(|wire| wire.borrow().name == w).is_none()
                    && other_wires
                        .iter()
                        .find(|wire| wire.borrow().name == w)
                        .is_none()
                {
                    other_wires.push(RefCell::new(Wire::new(w)));
                }
            });
        wires.extend(other_wires);
        Self { wires, gates }
    }
}

impl Circuit {
    fn apply_gate(&self, gate: &Gate) {
        if let Some(wire_ref) = self.wires.iter().find(|w| w.borrow().name == gate.input_1) {
            let wire = wire_ref.borrow();
            if wire.value.is_none() {
                if let Some(next_gate) = self.gates.iter().find(|g| g.output == wire.name) {
                    drop(wire);
                    self.apply_gate(next_gate);
                }
            }
        }
        if let Some(wire_ref) = self.wires.iter().find(|w| w.borrow().name == gate.input_2) {
            let wire = wire_ref.borrow();
            if wire.value.is_none() {
                if let Some(next_gate) = self.gates.iter().find(|g| g.output == wire.name) {
                    drop(wire);
                    self.apply_gate(next_gate);
                }
            }
        }
        let val = gate.apply(&self.wires).unwrap();
        if let Some(wire_ref) = self.wires.iter().find(|w| w.borrow().name == gate.output) {
            wire_ref.borrow_mut().value = Some(val);
        }
    }

    fn apply(&self) {
        self.gates.iter().for_each(|gate| {
            self.apply_gate(gate);
        });
    }

    fn get_z_value(&self) -> usize {
        let mut wires = self
            .wires
            .iter()
            .filter(|w| w.borrow().name.starts_with("z"))
            .map(|w| w.borrow())
            .collect::<Vec<_>>();
        wires.sort_by_key(|w| Reverse(w.name.clone()));
        let binary = wires
            .iter()
            .map(|w| match w.value.unwrap() {
                true => "1",
                false => "0",
            })
            .collect::<String>();
        usize::from_str_radix(binary.as_str(), 2).unwrap()
    }
}

fn main() {
    let input = read_input("day24.txt");
    let circuit = Circuit::from(input.as_str());
    circuit.apply();
    println!("Part 1 = {}", circuit.get_z_value());
}

#[cfg(test)]
mod day24_tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"#;
        let circuit = Circuit::from(input);
        assert_eq!(
            circuit.wires,
            vec![
                RefCell::new(Wire::new(String::from("x00")).with_value(true)),
                RefCell::new(Wire::new(String::from("x01")).with_value(true)),
                RefCell::new(Wire::new(String::from("x02")).with_value(true)),
                RefCell::new(Wire::new(String::from("y00")).with_value(false)),
                RefCell::new(Wire::new(String::from("y01")).with_value(true)),
                RefCell::new(Wire::new(String::from("y02")).with_value(false)),
                RefCell::new(Wire::new(String::from("z00"))),
                RefCell::new(Wire::new(String::from("z01"))),
                RefCell::new(Wire::new(String::from("z02"))),
            ]
        );
        assert_eq!(
            circuit.gates,
            vec![
                Gate::new(
                    GateKind::AND,
                    String::from("x00"),
                    String::from("y00"),
                    String::from("z00")
                ),
                Gate::new(
                    GateKind::XOR,
                    String::from("x01"),
                    String::from("y01"),
                    String::from("z01")
                ),
                Gate::new(
                    GateKind::OR,
                    String::from("x02"),
                    String::from("y02"),
                    String::from("z02")
                ),
            ]
        );
    }

    #[test]
    fn test_apply() {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        let circuit = Circuit::from(input);
        circuit.apply();
        let mut wires = circuit.wires;
        wires.sort_by_key(|w| w.borrow().name.clone());
        assert_eq!(
            wires,
            vec![
                RefCell::new(Wire::new(String::from("bfw")).with_value(true)),
                RefCell::new(Wire::new(String::from("bqk")).with_value(true)),
                RefCell::new(Wire::new(String::from("djm")).with_value(true)),
                RefCell::new(Wire::new(String::from("ffh")).with_value(false)),
                RefCell::new(Wire::new(String::from("fgs")).with_value(true)),
                RefCell::new(Wire::new(String::from("frj")).with_value(true)),
                RefCell::new(Wire::new(String::from("fst")).with_value(true)),
                RefCell::new(Wire::new(String::from("gnj")).with_value(true)),
                RefCell::new(Wire::new(String::from("hwm")).with_value(true)),
                RefCell::new(Wire::new(String::from("kjc")).with_value(false)),
                RefCell::new(Wire::new(String::from("kpj")).with_value(true)),
                RefCell::new(Wire::new(String::from("kwq")).with_value(false)),
                RefCell::new(Wire::new(String::from("mjb")).with_value(true)),
                RefCell::new(Wire::new(String::from("nrd")).with_value(true)),
                RefCell::new(Wire::new(String::from("ntg")).with_value(false)),
                RefCell::new(Wire::new(String::from("pbm")).with_value(true)),
                RefCell::new(Wire::new(String::from("psh")).with_value(true)),
                RefCell::new(Wire::new(String::from("qhw")).with_value(true)),
                RefCell::new(Wire::new(String::from("rvg")).with_value(false)),
                RefCell::new(Wire::new(String::from("tgd")).with_value(false)),
                RefCell::new(Wire::new(String::from("tnw")).with_value(true)),
                RefCell::new(Wire::new(String::from("vdt")).with_value(true)),
                RefCell::new(Wire::new(String::from("wpb")).with_value(false)),
                RefCell::new(Wire::new(String::from("x00")).with_value(true)),
                RefCell::new(Wire::new(String::from("x01")).with_value(false)),
                RefCell::new(Wire::new(String::from("x02")).with_value(true)),
                RefCell::new(Wire::new(String::from("x03")).with_value(true)),
                RefCell::new(Wire::new(String::from("x04")).with_value(false)),
                RefCell::new(Wire::new(String::from("y00")).with_value(true)),
                RefCell::new(Wire::new(String::from("y01")).with_value(true)),
                RefCell::new(Wire::new(String::from("y02")).with_value(true)),
                RefCell::new(Wire::new(String::from("y03")).with_value(true)),
                RefCell::new(Wire::new(String::from("y04")).with_value(true)),
                RefCell::new(Wire::new(String::from("z00")).with_value(false)),
                RefCell::new(Wire::new(String::from("z01")).with_value(false)),
                RefCell::new(Wire::new(String::from("z02")).with_value(false)),
                RefCell::new(Wire::new(String::from("z03")).with_value(true)),
                RefCell::new(Wire::new(String::from("z04")).with_value(false)),
                RefCell::new(Wire::new(String::from("z05")).with_value(true)),
                RefCell::new(Wire::new(String::from("z06")).with_value(true)),
                RefCell::new(Wire::new(String::from("z07")).with_value(true)),
                RefCell::new(Wire::new(String::from("z08")).with_value(true)),
                RefCell::new(Wire::new(String::from("z09")).with_value(true)),
                RefCell::new(Wire::new(String::from("z10")).with_value(true)),
                RefCell::new(Wire::new(String::from("z11")).with_value(false)),
                RefCell::new(Wire::new(String::from("z12")).with_value(false))
            ]
        )
    }

    #[test]
    fn part1() {
        let input = r#"x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj"#;
        let circuit = Circuit::from(input);
        circuit.apply();
        assert_eq!(circuit.get_z_value(), 2024);
    }
}
