pub mod halvar {
    use vulkano::{
        buffer::BufferContents, 
        pipeline::graphics::vertex_input::Vertex
    };

    #[derive(BufferContents, Vertex)]
    #[repr(C)]
    pub struct StandardVertex {
        #[format(R32G32_SFLOAT)]
        pub position: [f32; 2],
    }
}