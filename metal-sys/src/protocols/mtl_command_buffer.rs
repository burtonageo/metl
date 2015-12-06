use block::Block;
use cocoa::base::{BOOL, id};
use core_foundation::date::CFTimeInterval;
use MTLCommandBufferStatus;

pub trait MTLCommandBuffer {
    unsafe fn renderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id;
    unsafe fn computeCommandEncoder(self) -> id;
    unsafe fn blitCommandEncoder(self) -> id;
    unsafe fn parallelRenderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id;

    unsafe fn enqueue(self);
    unsafe fn commit(self);
    unsafe fn addScheduledHandler(self, block: MTLCommandBufferHandler);
    unsafe fn addCompletedHandler(self, block: MTLCommandBufferHandler);
    unsafe fn presentDrawable(self, drawable: id);
	unsafe fn presentDrawable_atTime(self, drawable: id, time: CFTimeInterval);
	unsafe fn waitUntilScheduled(self);
	unsafe fn waitUntilCompleted(self);

	unsafe fn status(self) -> MTLCommandBufferStatus;
	unsafe fn error(self) -> id;

	unsafe fn retainedReferences(self) -> BOOL;

	unsafe fn device(self) -> id;
	unsafe fn commandQueue(self) -> id;
	unsafe fn label(self) -> id;
	unsafe fn setLabel(self, label: id);
}

impl MTLCommandBuffer for id {
	unsafe fn renderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id {
		msg_send![self, renderCommandEncoderWithDescriptor:renderPassDescriptor]
	}

	unsafe fn computeCommandEncoder(self) -> id {
		msg_send![self, computeCommandEncoder]
	}

	unsafe fn blitCommandEncoder(self) -> id {
		msg_send![self, blitCommandEncoder]
	}

	unsafe fn parallelRenderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id {
		msg_send![self, parallelRenderCommandEncoderWithDescriptor:renderPassDescriptor]
	}

	unsafe fn enqueue(self) {
		msg_send![self, enqueue]
	}

	unsafe fn commit(self) {
		msg_send![self, commit]
	}

	unsafe fn addScheduledHandler(self, block: MTLCommandBufferHandler) {
		msg_send![self, addScheduledHandler:block]
	}

	unsafe fn addCompletedHandler(self, block: MTLCommandBufferHandler) {
		msg_send![self, addCompletedHandler:block]
	}

	unsafe fn presentDrawable(self, drawable: id) {
		msg_send![self, presentDrawable:drawable]
	}

	unsafe fn presentDrawable_atTime(self, drawable: id, time: CFTimeInterval) {
		msg_send![self, presentDrawable:drawable atTime:time]
	}

	unsafe fn waitUntilScheduled(self) {
		msg_send![self, waitUntilScheduled]
	}

	unsafe fn waitUntilCompleted(self) {
		msg_send![self, waitUntilCompleted]
	}

	unsafe fn status(self) -> MTLCommandBufferStatus {
		msg_send![self, status]
	}

	unsafe fn error(self) -> id {
		msg_send![self, error]
	}

	unsafe fn retainedReferences(self) -> BOOL {
		msg_send![self, retainedReferences]
	}

	unsafe fn device(self) -> id {
		msg_send![self, device]
	}

	unsafe fn commandQueue(self) -> id {
		msg_send![self, commandQueue]
	}

	unsafe fn label(self) -> id {
		msg_send![self, label]
	}

	unsafe fn setLabel(self, label: id) {
		msg_send![self, setLabel:label]
	}
}

pub type MTLCommandBufferHandler = Block<(id,), ()>;
