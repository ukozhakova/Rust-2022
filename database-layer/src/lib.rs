/*
 *
 *         Please, implement the library of functions
 *         here. Remove this pre-made code and create
 *         the rest of the library to use in other
 *         projects.
 *
 *         This file should ideally be used just to
 *         group code and export it to the end users:
 *         other Rust developers, and in the case
 *         of this task: you, the author.
 *
 */

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
