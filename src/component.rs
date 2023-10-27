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
}
