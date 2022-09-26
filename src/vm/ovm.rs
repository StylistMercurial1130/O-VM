use crate::vm::assembler::InstructionToken;
use crate::vm::assembler::InsturctionType;
use crate::vm::assembler::assemble;
pub struct VirtualMachineContext {
    instruction_buffer : Vec<InstructionToken>,
    virtual_machine_stack : Vec<i64>,
    virtual_machine_stack_index : usize,
    program_counter : usize
}

impl VirtualMachineContext {

    pub fn create_context(file_path : &str) -> VirtualMachineContext {
        VirtualMachineContext { 
            instruction_buffer: assemble(file_path), 
            virtual_machine_stack: Vec::new(),
            virtual_machine_stack_index : 0,
            program_counter : 0
        }
    }

    pub fn run(&mut self) {
        while self.program_counter < self.instruction_buffer.len() {
            let instruction = &self.instruction_buffer[self.program_counter].literal_type;
            let operand = &self.instruction_buffer[self.program_counter].operand;
            println!("{:?} {}",instruction,operand);
            match instruction {
                InsturctionType::Push => { 
                    self.virtual_machine_stack.push(*operand);
                    self.virtual_machine_stack_index += 1;
                    self.program_counter += 1;
                },
                InsturctionType::Pop => {
                    self.virtual_machine_stack.pop();
                    self.virtual_machine_stack_index -= 1;
                    self.program_counter += 1;
                },
                InsturctionType::Add => {
                    if self.virtual_machine_stack_index >= 2 {
                        let mut val = 0;
                        val += self.virtual_machine_stack[self.virtual_machine_stack_index - 1];
                        self.virtual_machine_stack.pop();
                        self.virtual_machine_stack_index -= 1;
                        val += self.virtual_machine_stack[self.virtual_machine_stack_index - 1];
                        self.virtual_machine_stack.pop();
                        self.virtual_machine_stack_index -= 1;
                        self.virtual_machine_stack.push(val);
                        self.virtual_machine_stack_index += 1;
                        self.program_counter += 1;
                    }
                    self.program_counter += 1;
                },
                InsturctionType::Subtract => {
                    if self.virtual_machine_stack_index >= 2 {
                        let mut val = 0;
                        val += self.virtual_machine_stack[self.virtual_machine_stack_index - 1];
                        self.virtual_machine_stack.pop();
                        self.virtual_machine_stack_index -= 1;
                        val -= self.virtual_machine_stack[self.virtual_machine_stack_index - 1];
                        self.virtual_machine_stack.pop();
                        self.virtual_machine_stack_index -= 1;
                        self.virtual_machine_stack.push(val);
                        self.virtual_machine_stack_index += 1;
                        self.program_counter += 1;
                    }
                    self.program_counter += 1;
                },
                InsturctionType::Halt => {
                    // do nothing for now !
                    self.program_counter += 1;
                },
                InsturctionType::Unknown => {
                    // do nothing for now !
                    self.program_counter += 1;
                }
            }
        }              
    }

}


    

