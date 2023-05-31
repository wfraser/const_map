/// Define a const map and a const lookup function as associated items of a struct.
///
/// When used, if the argument to the lookup function is a constant, the whole lookup will be done
/// at compile-time and replaced with the resulting constant.
///
/// If the argument is not a constant, it will still work, but won't be as efficient as an ordinary
/// map, because the lookup is just iterating all elements of the map.
///
/// # Example:
/// ```
/// # #[macro_use] extern crate const_map;
/// struct Fruits {
///     name: String,
/// }
///
/// impl Fruits {
///     const_map!(MAP, get(), (char => &'static str) {
///         'a' => "apple",
///         'b' => "banana",
///         'c' => "clementine",
///         'd' => "durian",
///     });
///     
///     fn new(name: &str) -> Self {
///         if let Some(s) = Self::get(name.chars().next().expect("shouldn't be empty")) {
///             Self { name: s.to_owned() }
///         } else {
///             Self { name: name.to_owned() }
///         }
///     }
/// }
///
/// assert_eq!(Fruits::new("bread").name, "banana");
/// assert_eq!(Fruits::new("kiwi").name, "kiwi");
///
/// // Because the lookup is a `const fn`, it can be used with generic consts:
///
/// struct MyFruit<const C: char> {
///     count: u64,
/// }
///
/// impl<const C: char> MyFruit<C> {
///     const NAME: &'static str = match Fruits::get(C) {
///         Some(s) => s,
///         None => panic!("no fruit found"),
///     };
///
///     pub fn desc(&self) -> String {
///         format!("{} {}", self.count, Self::NAME)
///     }
/// }
///
/// let f = MyFruit::<'d'> { count: 42 };
/// assert_eq!(f.desc(), "42 durian");
/// ```
#[macro_export]
macro_rules! const_map {
    ($name:ident, $lookup:ident(), ($kty:ty => $vty:ty) { $($k:expr => $v:expr),* $(,)? }) => {
        pub const $name: [($kty, $vty); $crate::count!($(($k, $v))*)] = [$(($k, $v)),*];

        const fn $lookup(key: $kty) -> Option<$vty> {
            #[inline]
            const fn find(pairs: &[($kty, $vty)], key: $kty, n: usize) -> Option<$vty> {
                if n >= pairs.len() {
                    return None;
                }
                match pairs[n] {
                    (k, v) if k == key => Some(v),
                    _ => find(pairs, key, n + 1),
                }
            }
            find(&Self::$name, key, 0)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! count {
    () => (0usize);
    ($x:tt $($xs:tt)*) => (1usize + $crate::count!($($xs)*));
}

#[cfg(test)]
mod test {
    struct S1;

    impl S1 {
        const_map!(MAP, map_get(), (char => &'static str) {
            'a' => "apple",
            'b' => "banana",
            'c' => "clementine",
            'd' => "durian",
        });
    }

    pub struct S2<const TAG: char>;

    impl<const TAG: char> S2<TAG> {
        pub const FRUIT: &'static str = match S1::map_get(TAG) {
            Some(s) => s,
            None => panic!("no fruit found"),
        };
    }

    #[test]
    fn test() {
        assert_eq!(S1::map_get('b'), Some("banana"));
        assert_eq!(S1::map_get('x'), None);
    }

    #[test]
    fn test_generic_const() {
        assert_eq!(S2::<'d'>::FRUIT, "durian");
    }
}

/// ```compile_fail
/// struct S<const V: i32>;
/// impl<const V: i32> S<V> {
///     const_map::const_map!(MAP, get(), (i32 => char) {
///         1 => 'a',
///         2 => 'b',
///         3 => 'c',
///         4 => 'd',
///     });
/// 
///     pub const C: char = match Self::get(V) {
///         Some(c) => c,
///         None => panic!("not found"),
///     };
/// }
/// let x = S::<5>::C;
/// ```
#[cfg(doctest)]
fn test_generic_const_panic() {}
