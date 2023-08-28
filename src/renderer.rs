use crate::object;
use crate::scene::Scene;


#[derive(Debug)]
pub struct Renderer {
    scene: Scene
}

impl Renderer {
    pub fn new(scene: Scene) -> Self {
        Self { scene }
    }
}
