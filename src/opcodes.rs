use crate::rummemory::Rummemory;
use std::process::exit;
use std::io::Read;
//use std::io;
//opcode 1

/// Moves values in a Rummemory regs. moves val in rb to ra
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn cmove(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    if mem.get_regs(rc.try_into().unwrap()) != 0{
        let new_val = mem.get_regs(rb.try_into().unwrap());
        mem.set_regs(ra.try_into().unwrap(),new_val);
    }
}

/// Loads a value in memory into reg[A]
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn segload(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let regs_b = mem.get_regs(rb.try_into().unwrap());
    let regs_c = mem.get_regs(rc.try_into().unwrap());
    let new_val = mem.get_mem_val(regs_b.try_into().unwrap(),regs_c.try_into().unwrap());
    mem.set_regs(ra.try_into().unwrap(),new_val);
}

/// Stores the value at r[c] into the memory at mem seg memory[r[a]][r[b]]
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn segstore(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let regs_a = mem.get_regs(ra.try_into().unwrap());
    let regs_b = mem.get_regs(rb.try_into().unwrap());
    let new_val = mem.get_regs(rc.try_into().unwrap());
    mem.set_mem_val(regs_a.try_into().unwrap(),regs_b.try_into().unwrap(),new_val);
}

/// add the values at reg b and c and stores in reg A
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn addition(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let new_val = mem.get_regs(rb.try_into().unwrap()).wrapping_add(mem.get_regs(rc.try_into().unwrap()));
    mem.set_regs(ra.try_into().unwrap(),new_val);
}

/// multiplies the values at reg b and c and stores in reg A
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn multiplication(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let new_val = mem.get_regs(rb.try_into().unwrap()).wrapping_mul(mem.get_regs(rc.try_into().unwrap()));
    mem.set_regs(ra.try_into().unwrap(),new_val);
}

/// divides the values at reg b and c and stores in reg A
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn division(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let new_val = mem.get_regs(rb.try_into().unwrap()) / mem.get_regs(rc.try_into().unwrap());
    mem.set_regs(ra.try_into().unwrap(),new_val);
}

/// Bitwise Nand the values at reg b and c and stores in reg A.
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the r[A] regs
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn bitnand(mem:&mut Rummemory ,ra:u32, rb:u32, rc:u32){
    let new_val = !(mem.get_regs(rb.try_into().unwrap()) & mem.get_regs(rc.try_into().unwrap()));
    mem.set_regs(ra.try_into().unwrap(),new_val);
}

/// Halts the program.
pub fn halt(){
    exit(0);
}

/// takes the value of r[c] make a vector of 0s r[c] long and stores in
/// a memory seg. The index of that new mem seg is stored in r[b]. It could 
/// be at the end of the memory or a deallocated mem seg spot.
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn mapseg(mem:&mut Rummemory , rb:u32, rc:u32){
    let word_amount = mem.get_regs(rc.try_into().unwrap());
    let new_seg = vec![0;word_amount.try_into().unwrap()];
    mem.mem_seg(new_seg,rb.try_into().unwrap());
}

/// Unmaps the memory seg at m[r[c]] and stores that index in queue to
/// be used later.
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `rc`: the index of the r[C] regs
pub fn unmapseg(mem:&mut Rummemory, rc:u32){
    let regs_c = mem.get_regs(rc.try_into().unwrap());
    mem.pop_mem_seg(regs_c.try_into().unwrap());
}

/// Takes the value at r[c] changes it into a char and prints it out.
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `rc`: the index of the r[C] regs
pub fn output(mem:&mut Rummemory, rc:u32){
    let output_val = mem.get_regs(rc.try_into().unwrap());
    print!("{}",char::from_u32(output_val).unwrap());
}

/// gets input fron the reader and stores each char in r[c]
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `rc`: the index of the r[C] regs
pub fn input(mem:&mut Rummemory, rc:u32){
    let mut buffer: [u8; 1] = [0; 1];
    let new_val = std::io::stdin().read(&mut buffer);
    match new_val {
        Ok(1)=>{
            mem.set_regs(rc.try_into().unwrap(),buffer[0].try_into().unwrap());
        }
        Ok(_) => {
            mem.set_regs(rc.try_into().unwrap(),u32::MAX);
        }
        Err(_)=>{
            panic!();
        }
    }
}

/// Mem Seg [r[b]] is cloned, replaces the instructions and the instruction counter is set 
/// to r[c]
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `rb`: the index of the r[B] regs
/// * `rc`: the index of the r[C] regs
pub fn loadp(mem:&mut Rummemory, rb:u32, rc:u32){
    //Comment out for performance
    //if mem.get_regs(rb.try_into().unwrap()) != 0{
        let new_index = mem.get_regs(rb.try_into().unwrap());
        let dopple = mem.get_mem_seg(new_index.try_into().unwrap()).clone();
        //println!("{:?}",dopple);
        mem.set_instructions(dopple);
    //}
    let reg_c = mem.get_regs(rc.try_into().unwrap());
    //let new_counter = mem.get_mem_val(0,reg_c.try_into().unwrap());
    mem.set_instruction_counter(reg_c);
}

/// Takes the "r[a]" or the 3 bits at lsb 25, goes to that regs and stores
/// the 25 bits in that reg value. 
/// 
/// # Arguments:
/// * `mem`: memory that is being manipulated
/// * `ra`: the index of the "r[a]" regs biu is actualy the 3 bits at lsb 25 
/// * `val`: the 25 bit val before "r[a]" to be stored.
pub fn loadval(mem:&mut Rummemory, ra:u32, val:u32){
    mem.set_regs(ra.try_into().unwrap(),val);
}