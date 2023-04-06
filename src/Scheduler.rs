use crate::process::{ProcessState, PROCESSOR};

pub fn round_robin_scheduler() 
{
    let mut current_process 
        = PROCESSOR
            .lock()
            .current_process()
            .unwrap();

    // Find the next process to run
    let mut next_process 
        = None;

    let mut found_current 
        = false;

    for process in PROCESSOR.lock().processes.iter_mut() 
    {
        if found_current && process.state == ProcessState::Ready 
        {
            next_process 
                = Some(process);

            break;
        } 
        else if process.id == current_process.id 
        {
            found_current 
                = true;
        }
    }

    // If no process was found, restart at the beginning of the process list
    if next_process.is_none() 
    {
        for process in PROCESSOR.lock().processes.iter_mut() 
        {
            if process.state == ProcessState::Ready 
            {
                next_process 
                    = Some(process);

                    break;
            }
        }
    }

    // If a next process was found, switch to it
    if let Some(process) = next_process 
    {
        crate::process::switch_process(process);
    }
}
