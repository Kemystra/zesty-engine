use core::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComponentType {
    Mesh
}

pub trait Component {
    fn component_type(&self) -> ComponentType;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        component_component_type: ComponentType
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                component_component_type: ComponentType::Mesh
            }
        }
    }

    impl Component for TestComponent {
        fn component_type(&self) -> ComponentType {
            self.component_component_type
        }
    }

    #[test]
    fn component_type() {
        let test_comp = TestComponent::new();
        assert_eq!(test_comp.component_type(), ComponentType::Mesh);
    }
}
