use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::Range;

use js::jsapi::NewExternalArrayBuffer;
use js::rooted;
use js::typedarray::HeapArrayBuffer;
use jstraceable_derive::JSTraceable;
use malloc_size_of_derive::MallocSizeOf;
use script_bindings::DomTypes;
use script_bindings::script_runtime::CanGc;
use script_bindings::trace::RootedTraceableBox;
use servo_arc::Arc;

use crate::traits::WebGPUGlobalTrait;

#[derive(JSTraceable, MallocSizeOf)]
pub struct DataBlock<D>
where
    D: DomTypes,
    D::GlobalScope: WebGPUGlobalTrait<D>,
{
    #[conditional_malloc_size_of]
    data: Arc<Box<[u8]>>,
    /// Data views (mutable subslices of data)
    data_views: Vec<DataView<D>>,
}

/// Returns true if two non-inclusive ranges overlap
// https://stackoverflow.com/questions/3269434/whats-the-most-efficient-way-to-test-if-two-ranges-overlap
fn range_overlap<T: std::cmp::PartialOrd>(range1: &Range<T>, range2: &Range<T>) -> bool {
    range1.start < range2.end && range2.start < range1.end
}

impl<D> DataBlock<D>
where
    D: DomTypes,
    D::GlobalScope: WebGPUGlobalTrait<D>,
{
    pub fn new_zeroed(size: usize) -> Self {
        let data = vec![0; size];
        Self {
            data: Arc::new(data.into_boxed_slice()),
            data_views: Vec::new(),
        }
    }

    /// Panics if there is any active view or src data is not same length
    pub(crate) fn load(&mut self, src: &[u8]) {
        // `Arc::get_mut` ensures there are no views
        Arc::get_mut(&mut self.data).unwrap().clone_from_slice(src)
    }

    /// Panics if there is any active view
    pub(crate) fn data(&mut self) -> &mut [u8] {
        // `Arc::get_mut` ensures there are no views
        Arc::get_mut(&mut self.data).unwrap()
    }

    pub(crate) fn clear_views(&mut self) {
        self.data_views.clear()
    }

    /// Returns error if requested range is already mapped
    pub(crate) fn view(&mut self, range: Range<usize>, _can_gc: CanGc) -> Result<&DataView<D>, ()> {
        if self
            .data_views
            .iter()
            .any(|view| range_overlap(&view.range, &range))
        {
            return Err(());
        }
        let range_len = range
            .end
            .checked_sub(range.start)
            .expect("range end must be >= range start");
        assert!(range.end <= self.data.len());

        let cx = D::GlobalScope::get_cx();
        /// `freeFunc()` must be threadsafe, should be safely callable from any thread
        /// without causing conflicts, unexpected behavior.
        unsafe extern "C" fn free_func(_contents: *mut c_void, free_user_data: *mut c_void) {
            let raw: *const Box<[u8]> = free_user_data.cast();
            // SAFETY: `free_func` is called by SM and returns ownership of the Arc we
            // leaked below with `into_raw`. Hence it is safe to reconstruct the Arc,
            // and destroy it to release the reference count.
            drop(unsafe { Arc::from_raw(raw) });
        }
        let raw: *const Box<[u8]> = Arc::into_raw(Arc::clone(&self.data));
        // SAFETY: We leaked the Arc, so the underlying slice will stay alive
        // until `free_func` is called. `range.start..range.end` is inside
        // the valid range of the slice.
        let data_ptr = unsafe { (**raw).as_ptr().add(range.start) };
        rooted!(in(*cx) let object = unsafe {
            NewExternalArrayBuffer(
                *cx,
                range_len,
                // FIXME(jschwe): I believe casting to a mutable pointer is unsound.
                // We would need interior mutability.
                data_ptr.cast_mut().cast(),
                Some(free_func),
                raw as _,
            )
        });
        self.data_views.push(DataView {
            range,
            buffer: HeapArrayBuffer::from(*object).unwrap(),
            phantom: PhantomData,
        });
        Ok(self.data_views.last().unwrap())
    }
}

#[derive(JSTraceable, MallocSizeOf)]
#[cfg_attr(crown, expect(crown::unrooted_must_root))]
pub struct DataView<D>
where
    D: DomTypes,
    D::GlobalScope: WebGPUGlobalTrait<D>,
{
    #[no_trace]
    range: Range<usize>,
    #[ignore_malloc_size_of = "defined in mozjs"]
    buffer: HeapArrayBuffer,
    phantom: PhantomData<D>,
}

impl<D> DataView<D>
where
    D: DomTypes,
    D::GlobalScope: WebGPUGlobalTrait<D>,
{
    pub(crate) fn array_buffer(&self) -> RootedTraceableBox<HeapArrayBuffer> {
        RootedTraceableBox::new(unsafe {
            HeapArrayBuffer::from(self.buffer.underlying_object().get()).unwrap()
        })
    }
}

impl<D> Drop for DataView<D>
where
    D: DomTypes,
    D::GlobalScope: WebGPUGlobalTrait<D>,
{
    #[expect(unsafe_code)]
    fn drop(&mut self) {
        let cx = D::GlobalScope::get_cx();
        assert!(unsafe {
            js::jsapi::DetachArrayBuffer(*cx, self.buffer.underlying_object().handle())
        })
    }
}
