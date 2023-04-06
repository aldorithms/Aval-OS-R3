#[derive(Debug, Clone)]
struct Message 
{
    sender: usize,
    data: [u8; 128],
}

fn receive_message() 
    -> Option<Message> 
    {
        let mut mailbox 
            = current_process()
                .mailbox
                    .lock();
        if let Some(message) = mailbox.pop() 
        {
            Some(message)
        } 
        else 
        {
            None
        }
    }

fn send_message(pid: usize, message: Message) 
    -> Result<(), &'static str> 
    {
        current_process()
            .send_message(pid, message)
    }
    

static MAILBOX: Mutex<Vec<Message>> 
    = Mutex::new(Vec::new());
