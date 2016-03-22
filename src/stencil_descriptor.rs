use cocoa::base::id;

pub struct StencilDescriptor(id);

impl Clone for StencilDescriptor {
    fn clone(&self) -> Self {
        unimplemented!();
    }
}

impl_from_into_raw!(StencilDescriptor, of class "MTLStencilDescriptor");
