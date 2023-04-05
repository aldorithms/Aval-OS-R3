use x86_64::structures::idt::InterruptStackFrame;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex { Breakpoint = 3, PageFault = 14, DoubleFault = 8, }

pub fn handle_interrupt(interrupt_index: InterruptIndex, _stack_frame: &mut InterruptStackFrame) 
{
    match interrupt_index 
    {
        InterruptIndex::Breakpoint 
        => 
            handle_breakpoint(), 
            _ => 
                panic!("Unhandled interrupt: {:?}", interrupt_index),
    }
}

fn handle_breakpoint() 
{
    // Do nothing for now
}