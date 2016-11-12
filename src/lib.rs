//! Represents a type that accepts values of other types and returns value of target type.

macro_rules! adapter_enum_item {
    ([] $($in_type:tt)*) => ($($in_type)*);
    ([boxed] $($in_type:tt)*) => (Box<$($in_type)*>);
}

macro_rules! adapter_enum {
    ($enum_name:ident($($t:ident)*), $([[$($boxed:ident)*] $in_name:ident: $($in_type:tt)*])+) => (
        pub enum $enum_name$(<$t>)* {
            $($in_name(adapter_enum_item!([$($boxed)*] $($in_type)*)),)+
            None
        }
    );
    
}

macro_rules! adapter_impl {
    ($enum_name:ident($($tvar:ident: $t:ident)*), $out_type:ty, $def:expr, $($in_name:ident, $var:ident, $getter:expr)+) => (
        impl$(<$t>)* $enum_name$(<$t>)* {
            fn set<U>(&mut self, val: U) where $enum_name$(<$t>)*: From<U> {
                *self = $enum_name::from(val);
            }
            fn get(&self $(, $tvar: &$t)*) -> $out_type {
                match *self {
                    $($enum_name::$in_name(ref $var) => { $getter },)+
                    $enum_name::None => { $def }
                }
            }
        }
    );
}

macro_rules! adapter_default {
    ($enum_name:ident($($t:ident)*)) => (
        impl$(<$t>)* Default for $enum_name$(<$t>)* {
            fn default() -> Self {
                $enum_name::None
            }
        }
    );
}

macro_rules! adapter_from {
    ($enum_name:ident($($t:ident)*), [], $in_name:ident, $($in_type:tt)*) => (
        impl$(<$t>)* From<$($in_type)*> for $enum_name$(<$t>)* {
            fn from(val: $($in_type)*) -> Self {
                $enum_name::$in_name(val)
            }
        }
    );
    ($enum_name:ident($($t:ident)*), [boxed], $in_name:ident, $($in_type:tt)*) => (
        impl<$($t,)* F> From<F> for $enum_name$(<$t>)* where F: 'static + $($in_type)* {
            fn from(val: F) -> Self {
                $enum_name::$in_name(Box::new(val))
            }
        }
    );
}

#[macro_export]
macro_rules! adapter_decl {
    (
        $enum_name:ident<$tvar:ident: $t:ident>: $out_type:ty => $def:expr;
        $($in_name:ident($($in_type:tt)*) {$var:ident => $getter:expr $(, $boxed:ident)*};)+
    ) => (
        adapter_enum!($enum_name($t), $([[$($boxed)*] $in_name: $($in_type)*])+);

        adapter_impl!($enum_name($tvar: $t), $out_type, $def, $($in_name, $var, $getter)+);

        adapter_default!($enum_name($t));

        $(adapter_from!($enum_name($t), [$($boxed)*], $in_name, $($in_type)*);)+
    );

    (
        $enum_name:ident: $out_type:ty => $def:expr;
        $($in_name:ident($($in_type:tt)*) {$var:ident => $getter:expr $(, $boxed:ident)*};)+
    ) => (
        adapter_enum!($enum_name(), $([[$($boxed)*] $in_name: $($in_type)*])+);

        adapter_impl!($enum_name(), $out_type, $def, $($in_name, $var, $getter)+);

        adapter_default!($enum_name());

        $(adapter_from!($enum_name(), [$($boxed)*], $in_name, $($in_type)*);)+
    );
}

#[macro_export]
macro_rules! adapter_accessors {
    ($getter:ident, $setter:ident, $field:ident -> $enum_name:ident<$t:ident>: $out_type:ty) => (
        fn $setter<T>(&mut self, $field: T) where $enum_name<$t>: From<T> {
            self.$field.set($field);
        }
        fn $getter(&self) -> $out_type {
            self.$field.get(self)
        }
    );
    ($getter:ident, $setter:ident, $field:ident -> $enum_name:ident: $out_type:ty) => (
        fn $setter<T>(&mut self, $field: T) where $enum_name: From<T> {
            self.$field.set($field);
        }
        fn $getter(&self) -> $out_type {
            self.$field.get()
        }
    );
}

#[cfg(test)]
mod tests {
    adapter_decl!(StringSource<obj: T>: String => String::default();
        Str(String) {s => s.clone()};
        Func(Fn(&T) -> String) {f => f(&obj), boxed};
    );

    adapter_decl!(IntSource: i64 => i64::default();
        Int(i64) {s => *s};
        Func(Fn() -> i64) {f => f(), boxed};
    );

    struct MyStruct {
        val: i64,
        name: StringSource<MyStruct>,
    }

    impl MyStruct {
        adapter_accessors!(get_name, set_name, name -> StringSource<Self>: String);
    }

    #[test]
    fn test_adaptor_in_struct() {
        let mut s1 = MyStruct { val: 3, name: Default::default() };
        assert_eq!("", s1.get_name());

        s1.set_name("s1".to_string());
        assert_eq!("s1", s1.get_name());

        s1.set_name(|ms: &MyStruct| -> String { format!("s1:{}", ms.val) });
        assert_eq!("s1:3", s1.get_name());

        s1.val = 42;
        assert_eq!("s1:42", s1.get_name());
    }

    #[test]
    fn test_adaptor_variable() {
        let mut a1 = IntSource::None;
        assert_eq!(0, a1.get());

        a1.set(7);
        assert_eq!(7, a1.get());

        a1.set(|| -> i64 { 42 });
        assert_eq!(42, a1.get());
    }
}