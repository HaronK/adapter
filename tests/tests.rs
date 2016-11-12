
#[macro_use]
extern crate adapter;

adapter_decl! {
    StringSource<obj: T>: String => String::default();
    Str(String) {s => s.clone()};
    Func(Fn(&T) -> String) {f => f(&obj), boxed};
};

adapter_decl! {
    IntSource: i64 => i64::default();
    Int(i64) {s => *s};
    Func(Fn() -> i64) {f => f(), boxed};
};

struct MyStruct {
    val: i64,
    name: StringSource<MyStruct>,
}

impl MyStruct {
    adapter_accessors! { get_name, set_name, name -> StringSource<Self>: String };
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
