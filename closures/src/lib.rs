use std::collections::HashMap;
use std::hash::Hash;

// Use generic and trait bound to define a structure to hold closure and a return value
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    caculation : T,
    value      : HashMap<u32, u32>,
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{
    fn new(caculation : T) -> Cacher<T> {
        Cacher {
            caculation,
            value : HashMap::new(),
        }
    }

    fn value(&mut self, arg : u32) -> u32 {
        if self.value.contains_key(&arg) {
            self.value[&arg]
        } else {
            let v = (self.caculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

struct GenericCacher<T, U, W>
    where U : Eq + Hash + Copy,
          W : Copy,
          T : Fn(U) -> W
{
    caculation : T,
    value      : HashMap<U, W>,
}

impl<T, U, W> GenericCacher<T, U, W>
    where U : Eq + Hash + Copy,
          W : Copy,
          T : Fn(U) -> W
{
    fn new(caculation : T) -> GenericCacher<T, U, W> {
        GenericCacher {
            caculation,
            value : HashMap::new(),
        }
    }

    fn value(&mut self, arg : U) -> W {
        if self.value.contains_key(&arg) {
            self.value[&arg]
        } else {
            let v = (self.caculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(v2, 2);
    }

    #[test]
    fn call_with_generic_cacher() {
        let mut c = GenericCacher::new(|line : &str| {4});
        let v1 = c.value("hello");
        let v2 = c.value("world");
        assert_eq!(v2, 4);
    }
}
