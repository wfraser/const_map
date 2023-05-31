const_map
=========

Define a const map and a const lookup function as associated items of a struct.

When used, if the argument to the lookup function is a constant, the whole lookup will be done
at compile-time and replaced with the resulting constant.

If the argument is not a constant, it will still work, but won't be as efficient as an ordinary
map, because the lookup is just iterating all elements of the map.

Note that due to current limitations in the Rust standard library and compiler, the key type needs
to be an integral type, bool, or char, because other types don't implement `PartialEq` in a const
way yet.

# Example:
```
# #[macro_use] extern crate const_map;
struct Fruits {
    name: String,
}

impl Fruits {
    const_map!(MAP, get(), (char => &'static str) {
        'a' => "apple",
        'b' => "banana",
        'c' => "clementine",
        'd' => "durian",
    });
    
    fn new(name: &str) -> Self {
        if let Some(s) = Self::get(name.chars().next().expect("shouldn't be empty")) {
            Self { name: s.to_owned() }
        } else {
            Self { name: name.to_owned() }
        }
    }
}

assert_eq!(Fruits::new("bread").name, "banana");
assert_eq!(Fruits::new("kiwi").name, "kiwi");

// Because the lookup is a `const fn`, it can be used with generic consts:

struct MyFruit<const C: char> {
    count: u64,
}

impl<const C: char> MyFruit<C> {
    const NAME: &'static str = match Fruits::get(C) {
        Some(s) => s,
        None => panic!("no fruit found"),
    };

    pub fn desc(&self) -> String {
        format!("{} {}", self.count, Self::NAME)
    }
}

let f = MyFruit::<'d'> { count: 42 };
assert_eq!(f.desc(), "42 durian");
```
