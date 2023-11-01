use core::fmt;
use std::any::Any;

use lib_derive::{Component, ComponentType};

pub mod mesh;


pub trait Component {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component")
    }
}

pub trait ComponentType: Component {
    const TYPE: &'static str;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct ManualImplComponent;

    impl Component for ManualImplComponent {
        fn as_any(&self) -> &dyn Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn Any {
            self
        }
    }

    impl ComponentType for ManualImplComponent {
        const TYPE: &'static str = "Test";
    }

    #[test]
    fn component_type() {
        assert_eq!(ManualImplComponent::TYPE, "Test");
    }

    #[derive(Component, ComponentType, Debug, PartialEq)]
    struct AutoImplComponent;

    #[test]
    fn component_custom_derive() {
        assert_eq!(AutoImplComponent::TYPE, "AutoImplComponent");
    }

    #[test]
    fn get_concrete_type_back() {
        let a = AutoImplComponent{};
        let trait_obj_a: Box<dyn Component> = Box::new(a);

        let original_a = trait_obj_a
            .as_any()
            .downcast_ref::<AutoImplComponent>()
            .unwrap();

        assert_eq!(original_a, &a);
    }

    #[test]
    fn get_concrete_type_back_mut() {
        let mut a = AutoImplComponent{};
        let mut trait_obj_a: Box<dyn Component> = Box::new(a);

        let original_a = trait_obj_a
            .as_any_mut()
            .downcast_mut::<AutoImplComponent>()
            .unwrap();

        assert_eq!(original_a, &mut a);
    }
}
