use crate::scene::scene::Scene;

pub struct SceneManager {

}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {}
    }
    pub fn load(&self, _path :  &str) -> Scene {
        Scene::new()
    }
}