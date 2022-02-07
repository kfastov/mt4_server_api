#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use lazy_static::lazy_static;
use std::sync::Mutex;

pub use mt4_server_api_macros::{hook, plugin_info, vtable};

lazy_static!{ pub static ref server_api: ServerRef = ServerRef::new(); }

// TODO server struct stores reference and is initialized by pointer

// TODO move bindings to separate crate
// TODO: try to keep a reference, not pointer
// TODO: think about why returning a static reference created from mutable pointer is unsafe
pub struct ServerRef {
    ptr: Mutex<Option<Wrapper>>,
}

unsafe impl Sync for ServerRef {}

impl ServerRef {
    // TODO std::sync::Once
    pub unsafe fn put(&self, ptr: *mut VTableHolder) {
        let mut opt = self.ptr.lock().unwrap();
        if let Some(_) = *opt {
            panic!("Attempt to initialize ServerRef more than once");
        } else {
            let _ = opt.insert(Wrapper {
                this_ptr: ptr,
            });
        }
    }
    pub fn new () -> Self {
        Self {
            ptr: Mutex::new(None),
        }
    }
    pub fn get (&self) -> Wrapper {
        self.ptr.lock().unwrap().unwrap()
    }
}

// impl Deref for ServerRef {
//     type Target = Wrapper;
//     fn deref(&self) -> &Self::Target {
//         self.ptr.lock().unwrap().as_ref().unwrap()
//     }
// }

#[vtable(CServerInterface, "bindings.rs")]
pub struct ServerVTable {
}