use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

use ipc_channel::ipc::IpcSharedMemory;
use malloc_size_of::MallocSizeOf;
use serde::{Deserialize, Serialize};

/// The main type of having either an [`IpcSharedMemory`] or an [`Arc<Mutex<Vec<u8>>>`]
#[derive(Clone, Deserialize)]
pub struct GenericSharedMemory(GenericSharedMemoryVariant);

#[derive(Clone, Deserialize)]
/// The type variant.
enum GenericSharedMemoryVariant {
    Ipc(IpcSharedMemory),
    Arc(Arc<Mutex<Vec<u8>>>),
}

/// We implement Serialize to guard against errournously serializing the ['GenericSharedMemory'] in Non-Ipc mode.
/// We will panic if this is the case.
impl Serialize for GenericSharedMemory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self.0 {
            GenericSharedMemoryVariant::Ipc(ipc_shared_memory) => {
                serializer.serialize_newtype_struct("IpcSharedMemory", &ipc_shared_memory)
            },
            GenericSharedMemoryVariant::Arc(_) => {
                unreachable!("You try to serialize a byte array in non-ipc mode.")
            },
        }
    }
}

impl MallocSizeOf for GenericSharedMemory {
    fn size_of(&self, ops: &mut malloc_size_of::MallocSizeOfOps) -> usize {
        match &self.0 {
            GenericSharedMemoryVariant::Ipc(ipc_shared_memory) => ipc_shared_memory.size_of(ops),
            GenericSharedMemoryVariant::Arc(mutex) => mutex.lock().unwrap().size_of(ops),
        }
    }
}

impl std::fmt::Debug for GenericSharedMemory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            GenericSharedMemoryVariant::Ipc(_) => f.debug_tuple("GenericSharedMemoryIpc").finish(),
            GenericSharedMemoryVariant::Arc(_) => f.debug_tuple("GenericSharedMemoryArc").finish(),
        }
    }
}

pub enum SharedMemoryView<'a> {
    Ipc(IpcSharedMemoryView<'a>),
    Arc(ArcSharedMemoryView<'a>),
}

impl<'a> Deref for SharedMemoryView<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        match self {
            SharedMemoryView::Ipc(ipc_shared_memory_view) => &ipc_shared_memory_view,
            SharedMemoryView::Arc(arc_shared_memory_view) => &arc_shared_memory_view,
        }
    }
}


/// The view into an IpcSharedMemory
struct IpcSharedMemoryView<'a>(&'a IpcSharedMemory);

/// The view into the Arc<Mutex<Vec>>, meaning a MutexGuard
struct ArcSharedMemoryView<'a>(MutexGuard<'a, Vec<u8>>);

impl<'a> Deref for IpcSharedMemoryView<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Deref for ArcSharedMemoryView<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl GenericSharedMemory {
    /// Get a view of the data which can be dereferences to a `&[u8]`
    pub fn view(&self) -> SharedMemoryView<'_> {
        match self.0 {
            GenericSharedMemoryVariant::Ipc(ref ipc_shared_memory) => {
                SharedMemoryView::Ipc(IpcSharedMemoryView(ipc_shared_memory))
            },
            GenericSharedMemoryVariant::Arc(ref mutex) => {
                SharedMemoryView::Arc(ArcSharedMemoryView(
                    mutex
                        .lock()
                        .expect("You borrowed an ipc shared memory readable two times."),
                ))
            },
        }
    }

    /// Create shared memory initialized with the bytes provided.
    pub fn from_bytes(bytes: &[u8]) -> GenericSharedMemory {
        if servo_config::opts::get().multiprocess || servo_config::opts::get().force_ipc {
            GenericSharedMemory(GenericSharedMemoryVariant::Ipc(
                IpcSharedMemory::from_bytes(bytes),
            ))
        } else {
            GenericSharedMemory(GenericSharedMemoryVariant::Arc(Arc::new(Mutex::new(
                Vec::from(bytes),
            ))))
        }
    }

    /// Create a shared memory initialized with 'byte' for 'length'
    pub fn from_byte(byte: u8, length: usize) -> GenericSharedMemory {
        if servo_config::opts::get().multiprocess || servo_config::opts::get().force_ipc {
            GenericSharedMemory(GenericSharedMemoryVariant::Ipc(IpcSharedMemory::from_byte(
                byte, length,
            )))
        } else {
            GenericSharedMemory(GenericSharedMemoryVariant::Arc(Arc::new(Mutex::new(
                vec![byte; length],
            ))))
        }
    }
}
