use std::ptr::NonNull;

#[test]
fn test() {
  #[repr(C)]
  #[derive(Debug, Default)]
  struct Base {
    identifier: i32,
  }

  #[repr(C)]
  #[derive(Debug, Default)]
  struct Derived {
    base: Base,
    data: String,
  }

  let derived_ptr = NonNull::new(Box::into_raw(Box::new(Derived {
    base: Base { identifier: 114 },
    data: "514".into(),
  })))
  .unwrap();
  let casted_base_ptr = derived_ptr.cast::<Base>();
  let casted_base = unsafe { casted_base_ptr.as_ref() };

  println!("{:?}", unsafe { derived_ptr.as_ref() });
  println!("{:?}", casted_base)
}
