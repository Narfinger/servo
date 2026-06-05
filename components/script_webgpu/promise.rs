use jstraceable_derive::JSTraceable;
use malloc_size_of_derive::MallocSizeOf;

#[derive(MallocSizeOf, JSTraceable)]
pub(crate) struct WebGPUPromise();

impl WebGPUPromise {
    pub(crate) fn is_fulfilled(&self) -> bool {
        todo!()
    }
}
