#![allow(dead_code)]

use derive_more::AsVariant;

#[derive(AsVariant)]
enum Either<TLeft, TRight> {
    Left(TLeft),
    Right(TRight),
}

#[derive(AsVariant)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

#[derive(AsVariant)]
enum Color {
    RGB(u8, u8, u8),
    // NOTE: named fields are not allowed in variants of
    // enums deriving AsVariant
    // CMYK { c: u8, m: u8, y: u8, k: u8 },
}

#[derive(AsVariant)]
enum Nonsense<'a, T> {
    Ref(&'a T),
    NoRef,
    #[as_variant(ignore)]
    NoRefIgnored,
}

#[derive(AsVariant)]
enum WithConstraints<T>
where
    T: Copy,
{
    One(T),
    Two,
}
#[derive(AsVariant)]
enum KitchenSink<'a, 'b, T1: Copy, T2: Clone>
where
    T2: Into<T1> + 'b,
{
    Left(&'a T1),
    Right(&'b T2),
    Empty,
    NeverMind(),
}

#[test]
pub fn test_as_variant() {
    assert_eq!(Some(()), Maybe::<()>::Nothing.as_nothing());
    assert_eq!(None, Maybe::<()>::Nothing.as_just());
}
