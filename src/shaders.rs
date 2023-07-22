pub mod halvar {
    pub mod vs {
        vulkano_shaders::shader!{
            ty: "vertex",
            path: "shaders/standard.vert",
        }
    }

    pub mod fs {
        vulkano_shaders::shader!{
            ty: "fragment",
            path: "shaders/standard.frag",
        }
    }
}