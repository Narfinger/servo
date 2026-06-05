impl<'a, D> WebGPUConvert<ProgrammableStageDescriptor<'a>> for &GPUProgrammableStage<D>
where
    D: DomTypes,
{
    fn convert(self) -> ProgrammableStageDescriptor<'a> {
        ProgrammableStageDescriptor {
            module: self.module.id().0,
            entry_point: self
                .entryPoint
                .as_ref()
                .map(|ep| Cow::Owned(ep.to_string())),
            constants: self
                .constants
                .as_ref()
                .map(|records| records.iter().map(|(k, v)| (k.0.clone(), **v)).collect())
                .unwrap_or_default(),
            zero_initialize_workgroup_memory: true,
        }
    }
}

impl<D> WebGPUConvert<WebGPUTextureView> for &GPUTextureOrGPUTextureView<D>
where
    D: DomTypes,
{
    fn convert(self) -> WebGPUTextureView {
        match self {
            GPUTextureOrGPUTextureView::GPUTextureView(view) => view.id(),
            GPUTextureOrGPUTextureView::GPUTexture(texture) => texture.get_default_view(),
        }
    }
}

impl<'a, D> WebGPUConvert<BindGroupEntry<'a>> for &GPUBindGroupEntry<D>
where
    D: DomTypes<
            GPUSampler = GPUSampler,
            GPUTexture = GPUTexture,
            GPUBuffer = GPUBuffer,
            GPUTextureView = GPUTextureView,
        >,
{
    fn convert(self) -> BindGroupEntry<'a> {
        BindGroupEntry {
            binding: self.binding,
            resource: match self.resource {
                GPUBindingResource::GPUSampler(ref s) => BindingResource::Sampler(s.id().0),
                GPUBindingResource::GPUTextureView(ref t) => BindingResource::TextureView(t.id().0),
                GPUBindingResource::GPUTexture(ref t) => {
                    BindingResource::TextureView(t.get_default_view().0)
                },
                GPUBindingResource::GPUBufferBinding(ref b) => {
                    BindingResource::Buffer(BufferBinding {
                        buffer: b.buffer.id().0,
                        offset: b.offset,
                        size: b.size,
                    })
                },
                GPUBindingResource::GPUBuffer(ref b) => BindingResource::Buffer(BufferBinding {
                    buffer: b.id().0,
                    offset: 0,
                    size: None,
                }),
            },
        }
    }