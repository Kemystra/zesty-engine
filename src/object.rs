use std::collections::HashMap;
use std::f64::consts::PI;

use crate::transform::Transform;
use crate::math_utils::vector3d::Vector3D;
use crate::component::{Component, ComponentType};


#[derive(Debug)]
pub struct Object {
    pub transform: Transform,
    components: HashMap<&'static str, Box<dyn Component>>
}

impl Object {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            components: HashMap::new()
        }
    }

    pub fn add_component<T>(&mut self, component: T)
    where T: Component + ComponentType + 'static {
        self.components.insert(T::TYPE, Box::new(component));
    }

    pub fn get_component<T>(&self) -> Option<&T>
    where T: Component + ComponentType + 'static {
        let dyn_obj = self.components.get(&T::TYPE)?;
        dyn_obj.as_any().downcast_ref::<T>()
    }

    pub fn get_component_mut<T>(&mut self) -> Option<&mut T>
    where T: Component + ComponentType + 'static {
        let dyn_obj = self.components.get_mut(&T::TYPE)?;
        dyn_obj.as_any_mut().downcast_mut::<T>()
    }
}

#[derive(Debug)]
pub struct Camera {
    pub transform: Transform,
    near_clip_distance: f64,
    far_clip_distance: f64,
    field_of_view: f64,
    aspect_ratio: AspectRatio,
    projection_data: ProjectionData,
    dirty_flag: bool
}


impl Camera {
    pub fn new<T: Into<f64>+Copy>(n: T, f: T, fov: T, aspect_ratio: AspectRatio) -> Self {
        Self {
            transform: Transform::new(),
            near_clip_distance: n.into(),
            far_clip_distance: f.into(),
            field_of_view: fov.into(),
            aspect_ratio,
            projection_data: ProjectionData::generate(
                n.into(), f.into(), fov.into(), aspect_ratio
            ),
            dirty_flag: false
        }
    }

    pub fn projection_data(&mut self) -> &ProjectionData {
        if self.dirty_flag {
            self.projection_data = ProjectionData::generate(
                self.near_clip_distance, 
                self.far_clip_distance,
                self.field_of_view,
                self.aspect_ratio
            )
        }

        &self.projection_data
    }

    pub fn project_to_screen_space(&mut self, point: Vector3D) -> Vector3D {
        // Deconstructing the data
        // What the actual frick
        let ProjectionData(w_scaler, h_scaler, m1, m2) = self.projection_data();
        let x = point.x * w_scaler;
        let y = point.y * h_scaler;
        let z = (point.z * m1) + m2;
        let w = -point.z;

        Vector3D {
            x: x/w,
            y: y/w,
            z: z/w,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct AspectRatio(pub f64, pub f64);


#[derive(Debug)]
pub struct ProjectionData(f64, f64, f64, f64);

// I'm just gonna hard code the aspect ratio lol
impl ProjectionData {
    pub fn generate(n: f64, f: f64, fov: f64, ratio: AspectRatio) -> ProjectionData {
        let fov_tan_val = (fov/2.0 * PI/180.0).tan();
        let near_far_interval = f - n;
        ProjectionData(
            1.0 / (fov_tan_val),
            ratio.0 / (ratio.1*fov_tan_val),
            -f / near_far_interval,
            f*n / near_far_interval
        )
    }
}


#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::component::{Component, ComponentType};
    use lib_derive::{Component, ComponentType};

    use super::*;

    #[test]
    fn new_obj() {
        let obj = Object::new();
        assert_eq!(obj.transform, Transform::new());
    }

    #[derive(Debug, Component, ComponentType, PartialEq)]
    struct TestComponent {}

    #[test]
    fn get_none_component() {
        let obj = Object::new();
        let result = obj.get_component::<TestComponent>();

        assert_eq!(result, None);
    }

    #[test]
    fn add_comp_to_obj() {
        let mut obj = Object::new();
        obj.add_component(TestComponent{});
        let result = obj.get_component::<TestComponent>();

        assert_eq!(result, Some(&TestComponent{}));
    }

    #[derive(Debug, Component, ComponentType, PartialEq)]
    struct ComponentWithField {
        name: String
    }

    #[test]
    fn get_mutable_component() {
        let mut obj = Object::new();
        let comp = ComponentWithField {
            name: "Hewwo".to_string()
        };

        obj.add_component(comp);

        let mut mutable_comp = obj
            .get_component_mut::<ComponentWithField>()
            .unwrap();
        mutable_comp.name = "Za Warudo!".to_string();

        let new_comp = obj
            .get_component::<ComponentWithField>()
            .unwrap();

        assert_eq!(new_comp.name, "Za Warudo!".to_string());
    }
}
