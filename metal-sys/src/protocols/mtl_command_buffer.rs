use cocoa::base::id;

pub trait MTLCommandBuffer {
    fn renderCommandEncoderWithDescriptor(self, renderPassDescriptor: id) -> id;
}

impl MTLCommandBuffer for id {

}
