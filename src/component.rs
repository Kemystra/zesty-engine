use core::fmt;


pub trait Component {}

impl fmt::Debug for dyn Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Help")
    }
}

pub trait ComponentType: Component {
    const TYPE: String;
}


#[cfg(test)]
mod tests {
    use super::*;

    struct TestComponent;

    impl Component for TestComponent {}

    impl ComponentType for TestComponent {
        const TYPE: String = "Mesh".to_string();
    }

    #[test]
    fn component_type() {
        assert_eq!(TestComponent::TYPE, "Mesh");
    }
}
