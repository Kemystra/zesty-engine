use core::fmt;

use lib_derive::{Component, ComponentType};

mod mesh;


pub trait Component {}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Component")
    }
}

pub trait ComponentType: Component {
    const TYPE: String;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct ManualImplComponent;

    impl Component for ManualImplComponent {}

    impl ComponentType for ManualImplComponent {
        const TYPE: String = "Test".to_string();
    }

    #[test]
    fn component_type() {
        assert_eq!(ManualImplComponent::TYPE, "Test");
    }

    #[derive(Component, ComponentType)]
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
            .downcast_ref::<AutoImplComponent>();

        assert_eq!(original_a, Some(&a));
    }

    #[test]
    fn get_concrete_type_back_mut() {
        let mut a = AutoImplComponent{};
        let trait_obj_a: Box<dyn Component> = Box::new(a);

        let original_a = trait_obj_a
            .as_any()
            .downcast_mut::<AutoImplComponent>();

        assert_eq!(original_a, Some(&mut a));
    }
}
