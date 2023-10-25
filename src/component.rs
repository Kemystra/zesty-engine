use core::fmt;


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ComponentType {
    Mesh
}

pub trait Component {
    fn type(&self) -> ComponentType;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent {
        component_type: ComponentType
    }

    impl TestComponent {
        fn new() -> Self {
            Self {
                component_type: ComponentType::Mesh
            }
        }
    }

    impl Component for TestComponent {
        fn type(&self) -> ComponentType {
            self.component_type
        }
    }

    #[test]
    fn type() {
        let test_comp = TestComponent::new();
        assert_eq!(test_comp.type(), ComponentType::Mesh);
    }
}
