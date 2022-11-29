type Umi = u32;
pub struct Field {
    width: u32,
    lsb: u32,
}
static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 {
    (instruction >> field.lsb) & mask(field.width)
}
pub fn op(instruction: Umi) -> u32 {
    (instruction >> OP.lsb) & mask(OP.width)
}
enum Opcode {CMov, Load, Store, Add, Mul, Div, NAND, HALT, MapSeg, UMapSeg, Out, In, LP, LV}

pub fn disassemble(inst: Umi) -> String {
    return match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            format!("if (r{} != 0) r{} := r{};", get(&RC, inst), get(&RA, inst), get(&RB, inst))
        },
        o if o == Opcode::Load as u32 => {
            format!("r{} := m[r{}][r{}];", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::Store as u32 => {
            format!("m[r{}][r{}] := r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::Add as u32 => {
            format!("r{} := r{} + r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::Mul as u32 => {
            format!("r{} := r{} * r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::Div as u32 => {
            format!("r{} := r{} / r{};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::NAND as u32 => {
            format!("r{} := r{} nand {};", get(&RA, inst), get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::HALT as u32 => {
            format!("Halt")
        },
        o if o == Opcode::MapSeg as u32 => {
            format!("r{} := map segment (r{} words);", get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::UMapSeg as u32 => {
            format!("unmap segment r{};", get(&RC, inst))
        },
        o if o == Opcode::Out as u32 => {
            format!("The output is r{};", get(&RC, inst))
        },
        o if o == Opcode::In as u32 => {
            format!("The input is r{} := input();", get(&RC, inst))
        },
        o if o == Opcode::LP as u32 => {
            format!("If m[r{}]= 0, Jump to r{}", get(&RB, inst), get(&RC, inst))
        },
        o if o == Opcode::LV as u32 => {
            format!("r{} := {};", get(&RL, inst), get(&VL, inst))
        },
        _ => {
            format!("Error")
        }

    }
}
