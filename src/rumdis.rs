use crate::opcodes::{cmove,segload,segstore,addition,multiplication,division,bitnand,halt,mapseg,unmapseg,output,input,loadp,loadval};
use crate::rummemory::Rummemory;
use std::process::exit;

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

enum Opcode{CMov,SegLoad,SegStore,Add,Mult,Div,Nand,Halt,MapSeg,UnmapSeg,Output,Input,LoadProg,LoadVal}

/// Gets a word and Rummemory, read a u32 word instruction and depending on
/// the opcode, applies that instruction to Rummemory.
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `inst`: the u32 word instruction that is being broken into opcode and ra
/// rb, and rc.
pub fn disassemble(current_memory:&mut Rummemory,inst: Umi){
    match get(&OP, inst) {
        o if o == Opcode::CMov as u32 => {
            cmove(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::SegLoad as u32 => {
            segload(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::SegStore as u32 => {
            segstore(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },    
        o if o == Opcode::Add as u32 => {
            addition(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Mult as u32 => {
            multiplication(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Div as u32 => {
            division(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Nand as u32 => {
            bitnand(current_memory,get(&RA, inst), get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Halt as u32 => {
            //println!("Proper halt");
            //println!("{}",countz);
            halt();
        },
        o if o == Opcode::MapSeg as u32 => {
            mapseg(current_memory, get(&RB, inst), get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::UnmapSeg as u32 => {
            unmapseg(current_memory, get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Output as u32 => {
            output(current_memory, get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::Input as u32 => {
            input(current_memory, get(&RC, inst));
            current_memory.counter_up_one();
        },
        o if o == Opcode::LoadProg as u32 => {
            //println!("Ahhh!");
            loadp(current_memory, get(&RB, inst), get(&RC, inst));
            //println!("Phew...");
        },
        o if o == Opcode::LoadVal as u32 => {
            loadval(current_memory, get(&RL, inst), get(&VL, inst));
            current_memory.counter_up_one();
        },
            _ => exit(1),
     }
}