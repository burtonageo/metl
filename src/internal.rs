use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::runtime::{BOOL, NO};

pub unsafe fn conforms_to_protocol(object: id, protocol_name: &str) -> bool {
    #[link(name = "Foundation", kind = "framework")]
    extern "C" {
        fn NSProtocolFromString(namestr: id) -> id;
    }

    let protocol_name_nsstr = NSString::alloc(nil).init_str(protocol_name);
    let protocol = NSProtocolFromString(protocol_name_nsstr);
    let does_conform: BOOL = msg_send![object, conformsToProtocol:protocol];
    does_conform == NO // TODO(George): Investigate why I need to compare to NO rather then YES
}
