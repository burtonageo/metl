use cocoa::base::id;
use sys::MTLComputePipelineState;

pub struct ComputePipelineState(id);

impl_from_into_raw!(ComputePipelineState, "MTLComputePipelineState");

impl ComputePipelineState {
    fn max_total_threads_per_thread_group(self) -> usize {
        self.0.maxTotalThreadsPerThreadGroup() as usize
    }

    fn thread_execution_width(self) -> usize {
        self.0.threadExecutionWidth() as usize
    }
}
