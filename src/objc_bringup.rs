#![allow(dead_code, non_snake_case)]

/// This file contains various traits/functions for interacting with Cocoa/objc classes which
/// are not available in the cocoa/objc crates. They should probably be migrated to them soon.

use cocoa::base::{BOOL, class, id};
use cocoa::foundation::NSUInteger;
use std::any::Any;

pub trait NSArray {
    unsafe fn arrayWithArray(array: id) -> Self
        where Self: Sized + Any {
        msg_send![class("NSArray"), arrayWithArray:array]
    }
    unsafe fn arrayWithObject(object: id) -> Self
        where Self: Sized + Any {
        msg_send![class("NSArray"), arrayWithObject:object]
    }
    unsafe fn arrayWithObjects_count(objects: *mut id, count: NSUInteger) -> Self
        where Self: Sized + Any {
        msg_send![class("NSArray"), arrayWithObjects:objects count:count]
    }

    unsafe fn containsObject(self, anObject: id) -> BOOL;
    unsafe fn count(self) -> NSUInteger;
    unsafe fn firstObject(self) -> id;
    unsafe fn lastObject(self) -> id;
    unsafe fn objectAtIndex(self, index: NSUInteger) -> id;
    unsafe fn indexOfObject(self, anObject: id) -> NSUInteger;
    unsafe fn indexOfObjectIdenticalTo(self, anObject: id) -> NSUInteger;
}

impl NSArray for id {
    unsafe fn containsObject(self, anObject: id) -> BOOL {
        msg_send![self, containsObject:anObject]
    }

    unsafe fn count(self) -> NSUInteger {
        msg_send![self, count]
    }

    unsafe fn firstObject(self) -> id {
        msg_send![self, firstObject]
    }

    unsafe fn lastObject(self) -> id {
        msg_send![self, lastObject]
    }

    unsafe fn objectAtIndex(self, index: NSUInteger) -> id {
        msg_send![self, objectAtIndex:index]
    }

    unsafe fn indexOfObject(self, anObject: id) -> NSUInteger {
        msg_send![self, indexOfObject:anObject]
    }

    unsafe fn indexOfObjectIdenticalTo(self, anObject: id) -> NSUInteger {
        msg_send![self, indexOfObjectIdenticalTo:anObject]
    }
}

pub trait NSDictionary {
    unsafe fn dictionaryWithObjects_forKeys_count(objects: *mut id, keys: *mut id,
                                                  count: NSUInteger)
                                                  -> Self
        where Self: Sized + Any {
        msg_send![class("NSDictionary"), dictionaryWithObjects:objects forKeys:keys count:count]
    }

    unsafe fn count(self) -> NSUInteger;
    unsafe fn objectForKey(self, key: id) -> id;
}

impl NSDictionary for id {
    unsafe fn count(self) -> NSUInteger {
        msg_send![self, count]
    }

    unsafe fn objectForKey(self, key: id) -> id {
        msg_send![self, objectForKey:key]
    }
}

pub trait NSError {
    unsafe fn code(self) -> NSUInteger;
    unsafe fn domain(self) -> id;
    unsafe fn userInfo(self) -> id;

    unsafe fn localizedDescription(self) -> id;
    unsafe fn localizedRecoveryOptions(self) -> id;
    unsafe fn localizedRecoverySuggestion(self) -> id;
    unsafe fn localizedFailureReason(self) -> id;
}

impl NSError for id {
    unsafe fn code(self) -> NSUInteger {
        msg_send![self, code]
    }

    unsafe fn domain(self) -> id {
        msg_send![self, domain]
    }

    unsafe fn userInfo(self) -> id {
        msg_send![self, userInfo]
    }

    unsafe fn localizedDescription(self) -> id {
        msg_send![self, localizedDescription]
    }

    unsafe fn localizedRecoveryOptions(self) -> id {
        msg_send![self, localizedRecoveryOptions]
    }

    unsafe fn localizedRecoverySuggestion(self) -> id {
        msg_send![self, localizedRecoverySuggestion]
    }

    unsafe fn localizedFailureReason(self) -> id {
        msg_send![self, localizedFailureReason]
    }
}
