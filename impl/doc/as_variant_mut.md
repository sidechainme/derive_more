# What `#[derive(AsVariant)]` generates

When an enum is decorated with `#[derive(AsVariantMut)]`, for each variant `foo` in
the enum, a public instance method `as_foo_mut(&mut self) -> Option<&mut T>` is generated
where T is the field type of that variant. AsVariantMut cannot be derived on enums
with named fields. If a variant is a unit type, the return type will be `Option<()>`
and if a variant has more than one unnamed field (e.g. `Enum::Variant(A, B)`,
the return type will be `Option<(&mut A, &mut B)>`. If you don't want the `as_foo_mut` method
generated for a variant you can put the `#[as_variant_mut(ignore)]` attribute on that variant.




## Example usage

```rust
# use derive_more::AsVariantMut;
#
#[derive(AsVariantMut)]
enum Maybe<T, U> {
    Just(T),
    Nothing,
    Many(T, U)
}


assert_eq!(Some(&mut 1)), Maybe::<u8, u16>::Just(1).as_just_mut());
assert_eq!(None, Maybe::<u8, u16>::Just(1).as_nothing());
assert_eq!(None, Maybe::<u8, u16>::Just(1).as_many());

assert_eq!(None, Maybe::<u8, u16>::Nothing.as_just());
assert_eq!(Some(())), Maybe::<u8, u16>::Nothing.as_nothing_mut());
assert_eq!(None, Maybe::<u8, u16>::Nothing.as_many());

assert_eq!(None, Maybe::<u8, u16>::Many(1, 2).as_just());
assert_eq!(None, Maybe::<u8, u16>::Many(1, 2).as_nothing());
assert_eq!(Some((&mut 1, &mut 2)), Maybe::<u8, u16>::Many(1, 2).as_many_mut());
```


### What is generated?

The derive in the above example code generates the following code:
```rust
# enum Maybe<T, U> {
#     Just(T),
#     Nothing,
#     Many(T, U)
# }
impl <T, U> Maybe<T, U>{
    pub fn as_just_mut(&mut self) -> Option<&mut T> {
        match self {Self::Just(_1) => Some((_1)), _ => None}
    }
    pub fn as_nothing_mut(&mut self) -> Option<()> {
        match self {Self::Nothing => Some(()), _ => None}
    }
    pub fn as_many_mut(&mut self) -> Option<(&mut T, &mut U)> {
        match self {Self::Many(_1, _2) => Some((_1, _2)), _ => None}
    }
}
```


