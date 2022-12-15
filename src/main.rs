use std::env;

use rum::rumdis;
use rum::rumload;
use rum::rummemory::Rummemory;



fn main(){
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    //println!("{} instructions", instructions.len());
    let mut my_memory = Rummemory::make_memory(instructions);
    //let mut wowi:u128 = 0;
    //let instructs = my_memory.get_instructions().clone();
    while my_memory.get_instruction_counter() < my_memory.get_instructions().len().try_into().unwrap() {
        let word_index = my_memory.get_instruction_counter().try_into().unwrap();
        let word =  my_memory.get_mem_val(0,word_index);
        rumdis::disassemble(&mut my_memory,word/* ,wowi*/);
        //wowi += 1;
        //println!("{}",wowi); 
    }
    //println!("Its no use!");
}

