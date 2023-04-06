use crate::interrupts::{ handle_interrupt, InterruptIndex };
use x86_64::structures::idt::{ InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode, };

lazy_static! 
{
    static ref IDT: InterruptDescriptorTable 
        = {
            let mut idt 
                = InterruptDescriptorTable::new();
                
            idt.breakpoint
                .set_handler_fn(handle_interrupt::<InterruptIndex::Breakpoint>);

            idt.page_fault
                .set_handler_fn(handle_page_fault);

            idt.double_fault
                .set_handler_fn(handle_double_fault);

            idt
        };
}

pub fn init() 
{
    IDT.load();
}

extern "x86-interrupt" fn handle_page_fault(stack_frame: &mut InterruptStackFrame,_error_code: PageFaultErrorCode,) 
{
    panic!("EXCEPTION: PAGE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn handle_double_fault(stack_frame: &mut InterruptStackFrame,_error_code: u64,) 
    -> ! 
    {
        panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    }