pub struct MatrixTransform {
    translation: [usize; 3],
    rotation: [[usize; 3]; 3],
    scale: usize
}

pub fn local_to_world_coord(transform: &MatrixTransform, local_coord: &[usize; 3]) -> [usize; 3] {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_to_world() {
        unimplemented!();
    }
}
