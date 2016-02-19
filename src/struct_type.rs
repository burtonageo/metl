use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc_bringup::NSArray;
use sys::MTLStructType;
use {FromRaw, StructMember};

pub struct StructType(id);

impl StructType {
    pub fn members(&self) -> Vec<StructMember> {
        let raw_members = unsafe { self.0.members() };
        let mut members_vec = vec![];
        unsafe {
            for i in 0..raw_members.count() {
                let member = raw_members.objectAtIndex(i);
                members_vec.push(FromRaw::from_raw(member).unwrap());
            }
        }
        members_vec
    }

    pub fn member_by_name(&self, name: &str) -> Option<StructMember> {
        let member_name_nsstr = unsafe { NSString::alloc(nil).init_str(name) };
        FromRaw::from_raw(unsafe { self.0.memberByName(member_name_nsstr) }).ok()
    }
}

impl_from_into_raw!(StructType, of class "MTLStructType");
