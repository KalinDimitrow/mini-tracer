use crate::scene::{
    elements::scene_element::{SceneElement, Geometry, Material},
};

pub struct GenericSceneElement {
    pub geometry : Box<dyn Geometry>,
    pub material : Box<dyn Material>,
}

impl SceneElement for GenericSceneElement {
    fn geometry(&self)-> &dyn Geometry {
        self.geometry.as_ref()
    }

    fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}