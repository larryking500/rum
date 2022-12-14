pub struct Rummemory{
    memory: Vec<Vec<u32>>,
    regs: Vec<u32>,
    instruction_counter:u32,
    queue: Vec<u32>
}

impl Rummemory{
    //Constructor 

    /// Constructor that makes a Rummemory given an instruction vector
    /// 
    /// # Arguments:
    /// * `instructions`: memory that is being manipulated
    pub fn make_memory(instructions:Vec<u32>)->Self{
        Self{
            memory: vec![instructions],
            regs: vec![0;8],
            instruction_counter: 0,
            queue: vec![]
        }
    }

    /// Returns a reference to the instructions of the memory aka memory[0]
    pub fn get_instructions(&mut self)->&Vec<u32>{
        &self.memory[0]
    }

    /// change the instructions aka change mem seg at m[0]
    /// 
    /// # Arguments:
    /// * `new_instruct`: the vector that will replace the previous instructions
    pub fn set_instructions(&mut self, new_instruct:Vec<u32>){
        self.memory[0] = new_instruct;
    }

    /// change the memory seg at a specific spot, rb
    /// 
    /// # Arguments:
    /// * `new_instruct`: the vector that will replace the previous instructions
    /// * `rb`: the index of the mem seg that is getting replaced
    pub fn mem_seg(&mut self, new_mem:Vec<u32>, rb:usize){
        if !self.queue.is_empty(){
            self.memory[self.queue[0] as usize] = new_mem;
            self.regs[rb] = self.queue[0];
            self.queue.remove(0);
        }
        else{
            self.memory.push(new_mem);
            self.regs[rb] = (self.memory.len() - 1) as u32;
         }
           
    }

    /// removes the memory seg at the given index, keeps the index in queue.
    /// 
    /// # Arguments:
    /// * `unmap_index`: the index of the mem seg that is going to be removed. 
    pub fn pop_mem_seg(&mut self, unmap_index:usize){
        self.memory[unmap_index].clear();
        self.queue.push(unmap_index.try_into().unwrap());  
    }

    /// set the instuction counter to a specific num
    /// 
    /// # Arguments:
    /// * `new_count`: the new value the instruction counter will be
    pub fn set_instruction_counter(&mut self, new_count:u32){
        self.instruction_counter = new_count;
    }

    /// set the instuction counter to a specific num
    /// 
    /// # Arguments:
    /// * `new_count`: the new value the instruction counter will be
    pub fn get_instruction_counter(&mut self)->u32{
        self.instruction_counter
    }

    /// add one to instruction counter
    pub fn counter_up_one(&mut self){
        self.instruction_counter += 1;
    }

    /// set the regs at a given index to a new val
    /// 
    /// # Arguments:
    /// * `reg_index`: index of which regs will be changed
    /// * `new_regs`: the new value the regs will be 
    pub fn set_regs(&mut self, reg_index:usize,new_regs:u32){
        self.regs[reg_index] = new_regs;
    }
    
    /// get a u32 from regs.
    /// 
    /// # Arguments:
    /// * `reg_index`: index of which regs whose value you want.
    pub fn get_regs(&mut self, reg_index:usize)->u32{
        self.regs[reg_index]
    }

    /// Change a value at a specified memory seg.
    /// 
    /// # Arguments:
    /// * `mem_index`: index of which mem whose value you want to change.
    /// * `vec_index`: index of which value in mem whose value you want to change.
    pub fn set_mem_val(&mut self, mem_index:usize, vec_index:usize, new_val:u32){
        self.memory[mem_index][vec_index] = new_val;
    }

    /// get a specific value stored in memory.
    /// 
    /// # Arguments:
    /// * `mem_index`: index of which mem whose value you want.
    /// * `vec_index`: index of which value in mem whose value you want.
    pub fn get_mem_val(&mut self, mem_index:usize, vec_index:usize)->u32{
        self.memory[mem_index][vec_index] 
    }

    /// get a mem seg, an entire vector of words.
    /// 
    /// # Arguments:
    /// * `mem_index`: index of which mem seg that you want.
    pub fn get_mem_seg(&mut self, mem_index:usize)->&Vec<u32>{
        &self.memory[mem_index]
    }
}