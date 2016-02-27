use cocoa::base::id;

pub struct ComputeCommandEncoder(id);

impl_from_into_raw!(ComputeCommandEncoder, of protocol "MTLComputeCommandEncoder");
