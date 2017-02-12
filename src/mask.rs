/*!
Boolean vectors.

## Comparison masks

Comparison masks are boolean vectors to be consumed by `select`.

`mask<F>(self, F)`: Creates a mask by applying the callable `F` to each component.

`masked<F>(self, rhs, F)`: Creates a mask by applying the callable `F` to each component on the left-hand and right-hand side.

`is_finite(self)`: Masks if the components are finite.

`is_infinite(self)`: Masks if the components are infinite.

`eq(self, rhs)`: Masks if the components are equal.

`ne(self, rhs)`: Masks if the components are not equal.

`lt(self, rhs)`: Masks if the left-hand side components are less than the right-hand side.

`le(self, rhs)`: Masks if the left-hand side components are less than or equal the right-hand side.

`gt(self, rhs)`: Masks if the left-hand side components are greater than the right-hand side.

`ge(self, rhs)`: Masks if the left-hand side components are greater than or equal the right-hand side.

`select(self, rhs, mask)`: Combines two vectors based on the mask, selecting components from the left-hand side if `true` and right-hand side if `false`.

### Examples

```
# use cgm::{Vec2, Vec3};
assert_eq!(Vec2 { x: true, y: false }, Vec2::new(1, 2).eq(Vec2::new(1, -2)));
```

## Comparison operators

`any(self)`: Returns `true` if any of the components is `true`.

`all(self)`: Returns `true` if all the components are `true`.

`none(self)`: Returns `true` if none of the components are `true`.

`BitAnd`, `BitOr`, `BitXor`, `Not`: Component-wise boolean operators.

### Examples

```
# use cgm::{Vec2};
assert!(Vec2 { x: true, y: false }.any());
assert!(Vec2 { x: true, y: true }.all());
assert!(Vec2 { x: false, y: false }.none());
```

*/

use ::std::{ops};

use ::vec::{Vec2, Vec3, Vec4};
use ::num::{Float};

macro_rules! mask {
	($vec:ident { $($field:ident),+ }) => {
		//----------------------------------------------------------------
		// Comparison masks

		impl<T> $vec<T> {
			/// Creates a mask by applying the callable `F` to each component.
			pub fn mask<F: FnMut(T) -> bool>(self, mut f: F) -> $vec<bool> {
				$vec { $($field: f(self.$field)),+ }
			}
			/// Creates a mask by applying the callable `F` to each component on the left-hand and right-hand side.
			pub fn masked<F: FnMut(T, T) -> bool>(self, rhs: $vec<T>, mut f: F) -> $vec<bool> {
				$vec { $($field: f(self.$field, rhs.$field)),+ }
			}
			/// Masks if the components are finite.
			pub fn is_finite(self) -> $vec<bool> where T: Float {
				$vec { $($field: self.$field.is_finite()),+ }
			}
			/// Masks if the components are infinite.
			pub fn is_infinite(self) -> $vec<bool> where T: Float {
				$vec { $($field: self.$field.is_infinite()),+ }
			}
			/// Masks if the components are equal.
			pub fn eq(self, rhs: $vec<T>) -> $vec<bool> where T: PartialEq {
				$vec { $($field: self.$field == rhs.$field),+ }
			}
			/// Masks if the components are not equal.
			pub fn ne(self, rhs: $vec<T>) -> $vec<bool> where T: PartialEq {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
			/// Masks if the left-hand side components are less than the right-hand side.
			pub fn lt(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field < rhs.$field),+ }
			}
			/// Masks if the left-hand side components are less than or equal the right-hand side.
			pub fn le(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field <= rhs.$field),+ }
			}
			/// Masks if the left-hand side components are greater than the right-hand side.
			pub fn gt(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field > rhs.$field),+ }
			}
			/// Masks if the left-hand side components are greater than or equal the right-hand side.
			pub fn ge(self, rhs: $vec<T>) -> $vec<bool> where T: PartialOrd {
				$vec { $($field: self.$field >= rhs.$field),+ }
			}
			/// Combines two vectors based on the mask, selecting components from the left-hand side if `true` and right-hand side if `false`.
			pub fn select(self, rhs: $vec<T>, mask: $vec<bool>) -> $vec<T> {
				$vec { $($field: if mask.$field { self.$field } else { rhs.$field }),+ }
			}
		}

		//----------------------------------------------------------------
		// Comparison operators

		impl $vec<bool> {
			/// Returns `true` if any of the components is `true`.
			pub fn any(self) -> bool {
				infix!(|| $(self.$field),+)
			}
			/// Returns `true` if all the components are `true`.
			pub fn all(self) -> bool {
				infix!(&& $(self.$field),+)
			}
			/// Returns `true` if none of the components are `true`.
			pub fn none(self) -> bool {
				!self.any()
			}
		}

		impl ops::BitAnd<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitand(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field && rhs.$field),+ }
			}
		}
		impl ops::BitOr<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitor(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field || rhs.$field),+ }
			}
		}
		impl ops::BitXor<$vec<bool>> for $vec<bool> {
			type Output = $vec<bool>;
			fn bitxor(self, rhs: $vec<bool>) -> $vec<bool> {
				$vec { $($field: self.$field != rhs.$field),+ }
			}
		}
		impl ops::Not for $vec<bool> {
			type Output = $vec<bool>;
			fn not(self) -> $vec<bool> {
				$vec { $($field: !self.$field),+ }
			}
		}
	};
}

mask!(Vec2 { x, y });
mask!(Vec3 { x, y, z });
mask!(Vec4 { x, y, z, w });
