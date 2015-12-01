extern crate mtl;
extern crate libc;

use mtl::sys::{MTLCopyAllDevices,MTLOrigin, MTLOriginMake, MTLClearColorMake};

#[test]
fn it_works() {
    let _origin = MTLOriginMake(50, 21, 11);
    let _color = MTLClearColorMake(50.0, 21.0, 11.0, 100.0);
    let _hi = unsafe { MTLCopyAllDevices() };
}
