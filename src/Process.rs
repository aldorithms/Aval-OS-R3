pub struct Process 
{
    id: u32,
    state: ProcessState,
    stack: [u8; STACK_SIZE],
    mailbox: Mutex<Vec<Message>>,
    stack_pointer: usize,
    program_counter: usize,
}

impl Process 
{
    fn send_message(&self, pid: usize, message: Message) 
        -> Result< (), &'static str> 
        {
            let mut processes 
                = PROCESSES
                    .lock();

            if let Some(process) = processes.get_mut(pid) 
            {
                process
                    .mailbox
                        .lock()
                        .push(message);

                Ok(())
            } 
        else 
        {
            Err("Invalid process ID")
        }
    }
    pub fn new(id: u32, stack_pointer: usize, program_counter: usize) 
        -> Self 
        {
            Process 
            { 
                id, 
                stack_pointer, 
                program_counter, 
            }
        }
}

use crate::process::Process;
use crate::memory::{allocate_stack, PAGE_SIZE};
use alloc::vec::Vec;

const SYS_CREATE_PROCESS: usize = 1;

pub fn handle_syscall(syscall_num: usize, args: [usize; 3]) 
-> isize 
    {
        match syscall_num 
        {
            SYS_CREATE_PROCESS 
            =>  
                create_process(args[0], args[1], args[2]),
                _ => 
                    panic!("unsupported syscall number: {}", syscall_num),
        }
}

fn create_process(id: u32, stack_size: usize, entry_point: usize) 
    -> isize 
    {
        let stack 
            = allocate_stack(stack_size);

        let process 
            = Process::new(id, stack.top(), entry_point);

        processes
            .push(process);

        processes
            .len() as isize - 1
    }

static mut processes: Vec<Process> 
    = Vec::new();

// switch_process function to switch to a new process
pub fn switch_process(new_process: &Process) 
{
    // Save the current process's state
    let mut current_process 
        = PROCESSOR
            .lock()
            .current_process()
            .unwrap();

    current_process
        .state 
            = ProcessState::Ready;

    current_process
        .cpu_state 
            = CpuState::from_registers();

    // Set the new process as the current process and mark it as running
    PROCESSOR
        .lock()
        .set_current_process(new_process);

    new_process
        .state 
            = ProcessState::Running;

    // Restore the new process's state
    new_process
        .cpu_state
            .load_registers();
}