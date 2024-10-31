
use abstract_algebra_manual::algbra_structs::Set;
use abstract_algebra_manual::utils::axiom::axiom;
use abstract_algebra_manual::algbra_structs::Group;

struct Group7<T> {
    elements: Vec<T>,
    operation: fn(T, T) -> T,
}

impl<T> Set<T> for Group7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {
    fn new_set(elements: Vec<T>) -> Self {
        Group7 { elements, operation: |_, _| panic!("Operation not defined") }
    }

    fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    fn sample(&self, index: Option<usize>) -> Option<&T> {
        let idx = index.unwrap_or(0);
        self.elements().get(idx)
    }

    fn contains(&self, element: &T) -> bool {
        self.elements().contains(element)
    }
}

impl<T> Group<T> for Group7<T>
where T: std::ops::Add<Output = T> + std::ops::Neg<Output = T> + std::cmp::PartialEq + Copy {

    fn new_group(elements: Vec<T>, operation: fn(T, T) -> T) -> Self {
        Group7 { elements, operation }
    }

    fn has_identity(&self) -> T {
        self.elements[0]
    }

    fn get_operation(&self) -> fn(T, T) -> T {
        self.operation
    }
}

fn main() {
    // let group = Group7::<i32>::new(vec![0, 1, 2, 3, 4, 5, 6]);

}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_group() -> Group7<i32> {
        Group7::new_group(vec![0, 1, 2, 3, 4, 5, 6], |x, y| (x + y) % 7)
    }

    #[test]
    fn test_elements() {
        let group = get_test_group();
        assert_eq!(group.elements(), &vec![0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_sample() {
        let group = get_test_group();
        assert_eq!(group.sample(Some(1)), Some(&1));
    }

    #[test]
    fn test_contains() {
        let group = get_test_group();
        assert_eq!(group.contains(&10), false);
        assert_eq!(group.contains(&7), false);
        assert_eq!(group.contains(&3), true);
    }

    #[test]
    fn axiom_test() {
        let group = get_test_group();
        assert_eq!(axiom((1, 2, 3), group.has_identity(), |x, y| x + y), 4);
    }

    #[test]
    fn test_operation() {
        let group = get_test_group();
        assert_eq!(group.get_operation()(1, 2), 3);
    }


}