1. Larry Khun

2. I got help from help hours, Connor Gray helped me come up with the idea to hold the memory segs
in a  2d vec. I also  got the idea from help hours to add a queue that would help keep track of
deallocated memory segs. Issac helped me with input and output and getting read() to work. 

3. Everything works well but I don't have every fail case implemented. Some invariants exist since 
we assume that the program inputed works and we also assume that when the instructions get changed that it won't
be replaced with an empty vec. Those are the main invariants i could think of

4. I made a bunch of methods for my Rummemory struct which each had the sole purpose to help me with 
the opcodes.I could most likely condense these methods down into ones that have a more general purpose
but I'll do that for optimization. For example I have a get_intrustuctions method that gets me mem seg 0
but also a get_mem_seg which gets me any memory segment, i could get rid of the get_instructions.

5. Besides main and lib there are 4 modules:
rumload:
Contains the load function that reads in an input name read the program into bytes and returns a vector
of words represented by u32.
rumdis:
the main file is the one that inputs a word but in rumdis is where the opcodes are ran. There are a few 
helper functions that help seperate the word into its opcodes and regs. The actual disassembler func
reads reads the opc code of each word and matches it with its appropiate opcode functions. 
opcodes:
module where the the opcode functions lives. Its a nice seperate place to test and keep the opcode funcs
that will be called in rumdis later.
rummemory:
The module that holds the Rummemory Struct which holdes the memory segs, an instruction counter, a queue and 
regs. There are also many methods that helped get the information from the struct for other modules to use.
The only method worth mentioning is the mem_seg function that takes a vector and allocates  it at a deallocated
memory spot or the end of the memory depending if theres anything in the que. 

6. Midmark has 85070521 or 85070522 instructions (my instruction counter variable adds 1 after the instruction)
so I used that as my testing for 50 mil instructions. I takes around 20 seconds to run midmark in release mode
and 40 seconds in debug mode.I used a timer on my ophone to test the time it takes rather than a timer in the
program so I might be a few seconds off. 

*****TESTING!!!!!*******
I used the rum-binary files to test my program. Each file works except for advent since that file takes forever
to run. the files helped me run and test all of the opcode functions. 

7. I spent about 5 hours in office hours trying to understand what rum is supposed to do and what each opcode
are suppose to do.I spent way too much time trying to figure out the map seg opcode because the part about it's
index confused me. 

8. I spent about 3 hours at most making the design doc which is essential is my design. 

9. Coding this assignment and getting and testing it to work took me about 6-8 hours. 