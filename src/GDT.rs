use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

lazy_static! 
{
    static ref GDT: GlobalDescriptorTable 
        = {
            let mut gdt 
                = GlobalDescriptorTable::new();

            let code_selector 
                = gdt
                    .add_entry(Descriptor::kernel_code_segment());

            let data_selector 
                = gdt
                    .add_entry(Descriptor::kernel_data_segment());
                
            GlobalDescriptorTable
                ::new()
        };
}

pub fn init() 
{
    GDT
        .load();
}