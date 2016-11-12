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
