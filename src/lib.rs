#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// Define a const map and a const lookup function as associated items of a struct.
///
/// The syntax is:
/// ```no_run
/// use const_map::const_map;
///
/// # // placeholders to make the example compile
/// # type KeyType = i32;
/// # type ValueType = i32;
/// # const key1: i32 = 0;
/// # const key2: i32 = 0;
/// # const value1: i32 = 0;
/// # const value2: i32 = 0;
///
/// struct YourStruct { /* ... */ }
///
/// impl YourStruct {
///     const_map!(
///         // The name of the associated constant holding the map.
///         // It will have type `[(KeyType, ValueType); N]` where `N` is the number of elements.
///         NAME,
///
///         // The name of the lookup function.
///         // It will have signature `const fn(k: KeyType) -> Option<ValueType>`.
///         lookup(),
///
///         // Specify the types of the keys and values of the map.
///         (KeyType => ValueType) {
///
///             // Followed by the entries of the map, which must be expressions that can be
///             // evaluated in a const (compile-time) context.
///             key1 => value1,
///             key2 => value2,
///             // etc.
///         }
///     );
///
///     // ...
/// }
/// ```
#[macro_export]
macro_rules! const_map {
    ($name_vis:vis $name:ident, $lookup_vis:vis $lookup:ident(), ($kty:ty => $vty:ty) { $($k:expr => $v:expr),* $(,)? }) => {
        $name_vis const $name: [($kty, $vty); $crate::count!($(($k, $v))*)] = [$(($k, $v)),*];

        $lookup_vis const fn $lookup(key: $kty) -> Option<$vty> {
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
        const_map!(pub MAP, pub map_get(), (char => &'static str) {
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
