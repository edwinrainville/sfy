#![no_std]

// Enable standard lib for tests.
#[cfg(test)]
#[macro_use]
extern crate std;

mod messages;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
