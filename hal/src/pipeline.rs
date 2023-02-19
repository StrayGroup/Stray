pub struct SRenderPipeline{
    texture_layout: Option<wgpu::BindGroupLayout>,
    pipeline: wgpu::RenderPipeline,
    render_mode: RenderMode
}

impl SRenderPipeline{
    pub fn with(
        pipeline: wgpu::RenderPipeline, 
        render_mode: RenderMode, 
        texture_layout: Option<wgpu::BindGroupLayout>
    ) -> Self{
        Self{
            texture_layout,
            pipeline,
            render_mode
        }
    }

    pub fn raw(&self) -> &wgpu::RenderPipeline{
        &self.pipeline
    }

    pub fn mode(&self) -> &RenderMode{
        &self.render_mode
    }

    pub fn layout(&self) -> &Option<wgpu::BindGroupLayout>{
        &self.texture_layout
    }
}   


#[derive(Debug)]
pub enum RenderMode{
    DRAW,
    DRAW_INDEXED
}