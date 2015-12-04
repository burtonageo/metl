use cocoa::base::{id, nil};
use command_queue::_get_raw_command_queue;
use sys::{MTLCommandBuffer, MTLCommandQueue};
use CommandQueue;

pub struct CommandBuffer(id);

impl CommandBuffer {
    pub fn new(command_queue: &mut CommandQueue) -> Self {
        let cmd_queue = _get_raw_command_queue(command_queue);
        let cmd_buf = unsafe { cmd_queue.commandBuffer() };
        CommandBuffer(cmd_buf)
    }

    pub fn with_unretained_references(command_queue: &mut CommandQueue) -> Self {
        let cmd_queue = _get_raw_command_queue(command_queue);
        let cmd_buf_unretained = unsafe { cmd_queue.commandBufferWithUnretainedReferences() };
        CommandBuffer(cmd_buf_unretained)
    }
}
