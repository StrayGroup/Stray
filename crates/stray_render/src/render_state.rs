use hal::*;

pub struct RenderState{
    objects: Vec<RenderObject>
}

impl RenderState{
    pub fn empty() -> Self{
        Self { 
            objects: vec![] 
        }
    }

    pub fn find(&mut self, id: u32) -> &mut RenderObject{
        let object = self.objects.iter_mut().find(|object| object.id == id).unwrap();
        object
    }   

    pub fn exist(&self, id: u32) -> bool{
        let result = self.objects.iter().find(|object| object.id == id);
        matches!(result, Some(_))
    }
    pub fn push(&mut self, obj: RenderObject){
        self.objects.push(obj);
    }
    pub fn redraw(
        &self,
        surface: &SSurface, 
        device: &SDevice,
        queue: &SQueue,
    ){
        let output = surface.raw().get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        //let smaa_frame = smaa_target.start_frame(&device, &queue, &view);
        let mut encoder = device.raw().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    //view: &*smaa_frame,
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.06,
                            g: 0.06,
                            b: 0.06,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            for obj in &self.objects{
                
                if let Some(bind_group) = &obj.bind_group{
                    render_pass.set_bind_group(0, bind_group, &[]);
                }
                if let Some(pipeline) = &obj.pipeline{
                    render_pass.set_pipeline(pipeline.raw());
                    if let Some(vertex) = &obj.vertex{
                        render_pass.set_vertex_buffer(
                            0, 
                            vertex.buffer().as_ref().unwrap().slice(..)
                        );
                        if matches!(pipeline.mode(), RenderMode::DRAW){
                            render_pass.draw(0..vertex.len(), 0..1);
                        }
                    }

                    if let Some(index) = &obj.index{
                        render_pass.set_index_buffer(
                            index.buffer().as_ref().unwrap().slice(..), 
                            wgpu::IndexFormat::Uint16
                        );
                        if matches!(pipeline.mode(), RenderMode::DRAW_INDEXED){
                            render_pass.draw_indexed(0..index.len(), 0, 0..1)
                        }
                    }

                }


            }
        }
        queue.raw().submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

pub struct RenderObject{
    pub id: u32,
    pub pipeline: Option<hal::SRenderPipeline>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub vertex: Option<stray_scene::SVertexBuffer>,
    pub index: Option<stray_scene::SIndexBuffer>,
}

impl RenderObject{
    pub fn update<F: Rendy>(
        &mut self, 
        from: &F, 
        device: &SDevice,
        config: &SConfig,
        queue: &SQueue,
        transform: &stray_scene::Transform2D,
    ){
        let vertices = from.create_vertices(device, config, transform);
        let bind_group = if matches!(self.bind_group, Some(_)){
            from.create_bind_group(
                device, 
                queue, 
                &self.pipeline.as_ref().unwrap().layout().as_ref().unwrap()
            )
        } else {None};
        self.vertex = vertices;
        self.bind_group = bind_group;
    }
}

pub trait Rendy{
    fn get_id(&self) -> u32;
    fn render(
        &self,
        device: &SDevice,
        config: &SConfig,
        queue: &SQueue,
        transform: &stray_scene::Transform2D
    ) -> RenderObject;

    fn create_vertices(
        &self, 
        device: &SDevice,
        config: &SConfig,
        transform: &stray_scene::Transform2D
    ) -> Option<stray_scene::SVertexBuffer>{None}

   fn create_bind_group(
        &self, 
        device: &SDevice,
        queue: &SQueue,
        layout: &wgpu::BindGroupLayout
    ) -> Option<wgpu::BindGroup>{None}

    fn create_pipeline(&self, 
        device: &SDevice, 
        config: &SConfig
    ) -> SRenderPipeline;

    fn create_indices(
        &self,
        device: &SDevice,
    ) -> Option<stray_scene::SIndexBuffer>{None}
}