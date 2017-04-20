/*!
Defines the vector types.

An overview of their implementations:

## Derived traits

`Copy`, `Clone` where T: `Copy`, `Clone`: For convenience all instances are passed by value, greatly simplifying usage.

`Debug`, `Display` where T: `Debug`, `Display`: Struct-based formatting and tuple-based formatting respectively.

`Eq`, `PartialEq` where T: `Eq`, `PartialEq`: Compares if _all_ the underlying components are equal.

`Ord`, `PartialOrd` where T: `Ord`, `PartialOrd`

`Hash` where T: `Hash`

### Examples

```
# use cgm::prelude::{Vec2, Vec3};
assert_eq!("(2,3,4)", format!("{}", Vec3::new(2, 3, 4)));
assert_eq!("(2.300,2.142)", format!("{:.3}", Vec2::new(2.3, 2.14159278)));
assert_eq!("Vec2 { x: 16, y: 25 }", format!("{:?}", Vec2::new(16, 25)));
assert_eq!("(  2,  3, 14)", format!("{:>3}", Vec3::new(2, 3, 14)));

assert_eq!(Vec2 { x: -5, y: 9 }, Vec2::new(-5, 9));
assert!(Vec2::new(1, 9) > Vec2::new(1, -2));
```

## Constructors

`new(x, y, ...)`: Constructs a new vector from components.

`dup(u)` where T: `Copy`: Constructs a new vector by splatting to its components.

`unit_x()`, `unit_y()`, `unit_z()`, `unit_w()` where T: `Zero` + `One`: Constructs a unit vector along the given axis (given that axis exists for the vector).

`set_x(self, x)`, `set_y(self, y)`, `set_z(self, z)`, `set_w(self, w)`: Note that these return new vectors with the respective component changed.

### Examples

```
# use cgm::prelude::{Vec2, Vec3};
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::new(1, 2));

assert_eq!(Vec3 { x: 42, y: 42, z: 42 }, Vec3::dup(42));

assert_eq!(Vec2 { x: 0.0, y: 1.0 }, Vec2::unit_y());
assert_eq!(Vec3 { x: 1, y: 0, z: 0 }, Vec3::unit_x());

assert_eq!(Vec3 { x: -12, y: 0, z: 12 }, Vec3::default().set_x(-12).set_z(12));
```

## Extending and truncating

`vec3(self, T)`: Extends `Vec2` with a `z` component.

`vec4(self, T)`: Extends `Vec3` with a `w` component.

`xy(self)`: Drops the `z` component from `Vec3` and `w` from `Vec4`.

`xyz(self)`: Drops the `w` component from `Vec4`.

### Examples

```
# use cgm::prelude::{Vec2, Vec3, Vec4};
assert_eq!(Vec3 { x: 3, y: 4, z: 5 }, Vec2::new(3, 4).vec3(5));

assert_eq!(Vec4 { x: -1, y: -2, z: -3, w: -4 }, Vec3::new(-1, -2, -3).vec4(-4));

assert_eq!(Vec2 { x: 2, y: 1 }, Vec3::new(2, 1, 0).xy());
assert_eq!(Vec2 { x: 1, y: 2 }, Vec4::new(1, 2, 3, 4).xy());

assert_eq!(Vec3 { x: 1, y: 2, z: 3 }, Vec4::new(1, 2, 3, 4).xyz());
```

## Transformations

`cast<U>(self)` where T: `Cast<U>`: Casts to a vector of type `U` with the same dimensions.

`map<U, F>(self, F)` where F: `FnMut(T) -> U`: Maps a callable over the components.

`zip<U, F>(self, rhs, F)` where F: `FnMut(T, T) -> U`: Zips two vectors together.

`reduce<F>(self, F)` where F: `Fn(T, T) -> T`: Reduces the vector. The `x` component is used as the initial value of the accumulator.

`fold<A, F>(self, acc, F)` where F: `Fn(A, T) -> A`: Folds the vector.

### Examples

```
# use cgm::prelude::{Vec2, Vec3};
assert_eq!(Vec2 { x: 2, y: 4 }, Vec2::new(2.2, 4.9).cast());

assert_eq!(Vec2 { x: 2, y: 4 }, Vec2::new(1, 2).map(|c| c * 2));

let left = Vec2::new(1, 2);
let right = Vec2::new(1, -1);
assert_eq!(Vec2 { x: 3, y: 3 }, Vec2::zip(left, right, |a, b| a * 2 + b));

let vec = Vec3::new(5, 3, 2);
assert_eq!(0, vec.reduce(|acc, c| acc - c));
assert_eq!(-10, vec.fold(0, |acc, c| acc - c));
```

## Conversions

`From`, `Into`: PointN, N-tuple and N-array conversions.

`AsRef`, `AsMut`: PointN, N-tuple, N-array and slice conversions.

### Examples

```
# use cgm::prelude::{Vec2};
assert_eq!(Vec2::from((2, 3)), Vec2::from([2, 3]));
```

## Operations where T is `Scalar`

`sqr(self)`: Squares the components.

`len_sqr(self)`: Calculates the squared length of the vector.

`len(self)` where T: `Float`: Calculates the length of the vector.

`dist_sqr(self, to)`: Calculates the squared euclidean distance to another vector.

`dist(self, to)` where T: `Float`: Calculates the euclidean distance to another vector.

`norm(self)` where T: `Float`: Normalizes the vector. The vector with length zero stays zero.

`resize(self, len)` where T: `Float`: Scales the vector such that its length equals the given value. The vector with length zero remains zero.

`scalar_project(self, v)` where T: `Float`: Scalar projection of `v` on `self`.

`project(self, v)` where T: `Float`: Projection of `v` on `self`.

`hadd(self)`: Horizontal adds all components.

`dot(self, rhs)`: Calculates the inner product.

`cos_angle(self, rhs)`: Calculates the cosine of the inner angle.

`angle(self, rhs)`: Calculates the inner angle.

Exclusive to `Vec2`:

`hsub(self)`: Horizontal subtracts the components of `Vec2`.

`polar_angle(self)`: Calculates the polar angle.

`ccw(self)`: Rotates the vector counter-clockwise by 90°.

`cw(self)`: Rotates the vector clockwise by 90°.

`cross(self, rhs)`: Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.

Exclusive to `Vec3`:

`cross(self, rhs)`: Calculates the 3D cross product.

### Examples

```
# use cgm::prelude::{Vec2, Vec3};
assert_eq!(Vec2 { x: 9, y: 16 }, Vec2::new(3, 4).sqr());

assert_eq!(25, Vec2::new(3, 4).len_sqr());
assert_eq!(5.0, Vec2::new(3.0, 4.0).len());

assert_eq!(2, Vec2::dist_sqr(Vec2::new(1, 1), Vec2::new(2, 2)));
assert_eq!(5.0, Vec2::dist(Vec2::new(10.0, 10.0), Vec2::new(13.0, 14.0)));

assert_eq!(Vec2 { x: 0.6, y: 0.8 }, Vec2::new(3.0, 4.0).norm());
assert_eq!(Vec2 { x: 0.0, y: 0.0 }, Vec2::new(0.0, 0.0).norm());

assert_eq!(Vec2 { x: 1.5, y: 2.0 }, Vec2::new(3.0, 4.0).resize(2.5));
assert_eq!(Vec2 { x: 0.0, y: 0.0 }, Vec2::new(0.0, 0.0).resize(2.0));

assert_eq!(2.2, Vec2::new(3.0, 4.0).scalar_project(Vec2::new(1.0, 2.0)));
assert_eq!(2.0, Vec3::new(4.0, 2.0, 4.0).scalar_project(Vec3::new(1.0, 4.0, 0.0)));

assert_eq!(12, Vec3::new(3, 4, 5).hadd());
assert_eq!(-1, Vec2::new(3, 4).hsub());

assert_eq!(Vec2 { x: 4, y: -3 }, Vec2::new(3, 4).ccw());
assert_eq!(Vec2 { x: -4, y: 3 }, Vec2::new(3, 4).cw());
assert_eq!(10, Vec2::cross(Vec2::new(3, 4), Vec2::new(-1, 2)));

assert_eq!(12, Vec3::dot(Vec3::new(1, 2, 3), Vec3::new(4, -5, 6)));
assert_eq!(Vec3 { x: -12, y: 1, z: 39 }, Vec3::cross((3, -3, 1).into(), (4, 9, 1).into()));
```

## Packed

`unpack32(v)`: Unpacks `u64` to 2×`u32`.

`unpack16(v)`: Unpacks `u64` to 4×`u16` or `u32` to 2×`u16`.

`unpack8(v)`: Unpacks `u32` to 4×`u8` or `u16` to 2×`u8`.

`pack(self)`: Packs back together as unsigned integer.

### Examples

```
# use cgm::prelude::{Vec2, Vec4};
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack32(0x00000002_00000001));
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack16(0x0002_0001));
assert_eq!(Vec2 { x: 1, y: 2 }, Vec2::unpack8(0x02_01));

assert_eq!(Vec4 { x: 1, y: 2, z: 3, w: 4 }, Vec4::unpack16(0x0004_0003_0002_0001));
assert_eq!(Vec4 { x: 1, y: 2, z: 3, w: 4 }, Vec4::unpack8(0x04_03_02_01));

// Example to unpack RGBA u32 where x: red, y: green, z: blue, w: alpha.
let color = Vec4::unpack8(0xFFC08040).cast::<f32>() / 255.0;
assert_eq!(Vec4 { x: 64.0/255.0, y: 128.0/255.0, z: 192.0/255.0, w: 1.0 }, color);
```

## Operators

`abs(self)`: Component-wise absolute value.

`min(self, rhs)`: Component-wise minimum value.

`max(self, rhs)`: Component-wise maximum value.

`mul_add(self, vec, scale)`: Adds the scaled value.

`Add`: Adds the vectors component-wise.

`Sub`: Subtracts the vectors component-wise.

`Neg`: Negates the vector component-wise.

`Mul`: Multiply by scalar or vector.

`Div`: Divide by scalar or vector.

`Rem`: Remainder by scalar or vector.

### Examples

```
# use cgm::prelude::{Vec2, Vec3};
```
*/

use ::std::{mem, ops};

use ::num::{Scalar, Zero, One, Abs, Min, Max, Float, Cast};

use ::angle::{Rad, Angle};

// /// A 1-dimensional vector.
// #[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
// #[repr(C)]
// pub struct Vec1<T> {
// 	pub x: T,
// }

/// A 2-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec2<T> {
	pub x: T,
	pub y: T,
}

/// A 3-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec3<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}

/// A 4-dimensional vector.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[repr(C)]
pub struct Vec4<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}

macro_rules! unit {
	(Vec1) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec1<T> where T: Zero + One { Vec1 { x: T::one() } }
	};
	(Vec2) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec2<T> where T: Zero + One { Vec2 { x: T::one(), y: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec2<T> where T: Zero + One { Vec2 { x: T::zero(), y: T::one() } }
	};
	(Vec3) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec3<T> where T: Zero + One { Vec3 { x: T::one(), y: T::zero(), z: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec3<T> where T: Zero + One { Vec3 { x: T::zero(), y: T::one(), z: T::zero() } }
		/// A unit vector in the `z` direction.
		pub fn unit_z() -> Vec3<T> where T: Zero + One { Vec3 { x: T::zero(), y: T::zero(), z: T::one() } }
	};
	(Vec4) => {
		/// A unit vector in the `x` direction.
		pub fn unit_x() -> Vec4<T> where T: Zero + One { Vec4 { x: T::one(), y: T::zero(), z: T::zero(), w: T::zero() } }
		/// A unit vector in the `y` direction.
		pub fn unit_y() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::one(), z: T::zero(), w: T::zero() } }
		/// A unit vector in the `z` direction.
		pub fn unit_z() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::zero(), z: T::one(), w: T::zero() } }
		/// A unit vector in the `w` direction.
		pub fn unit_w() -> Vec4<T> where T: Zero + One { Vec4 { x: T::zero(), y: T::zero(), z: T::zero(), w: T::one() } }
	};
}

macro_rules! set {
	(Vec1) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec1<T> { Vec1 { x: x } }
	};
	(Vec2) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec2<T> { Vec2 { x: x, y: self.y } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y: y } }
	};
	(Vec3) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec3<T> { Vec3 { x: x, y: self.y, z: self.z } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec3<T> { Vec3 { x: self.x, y: y, z: self.z } }
		/// Sets the `z` component.
		pub fn set_z(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: z } }
	};
	(Vec4) => {
		/// Sets the `x` component.
		pub fn set_x(self, x: T) -> Vec4<T> { Vec4 { x: x, y: self.y, z: self.z, w: self.w } }
		/// Sets the `y` component.
		pub fn set_y(self, y: T) -> Vec4<T> { Vec4 { x: self.x, y: y, z: self.z, w: self.w } }
		/// Sets the `z` component.
		pub fn set_z(self, z: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: z, w: self.w } }
		/// Sets the `w` component.
		pub fn set_w(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w: w } }
	};
}

macro_rules! cvt {
	(Vec1) => {
		/// Extends the 1D vector with a `y` component.
		pub fn vec2(self, y: T) -> Vec2<T> { Vec2 { x: self.x, y: y } }
	};
	(Vec2) => {
		/// Extends the 2D vector with a `z` component.
		pub fn vec3(self, z: T) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: z } }
		/// Extends the 2D vector with a `z` and `w` component.
		pub fn vec4(self, z: T, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: z, w: w } }
	};
	(Vec3) => {
		/// Extends the 3D vector with a `w` component.
		pub fn vec4(self, w: T) -> Vec4<T> { Vec4 { x: self.x, y: self.y, z: self.z, w: w } }
		/// Drops the `z` component.
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
	};
	(Vec4) => {
		/// Drops the `z` and `w` coordinates.
		pub fn xy(self) -> Vec2<T> { Vec2 { x: self.x, y: self.y } }
		/// Drops the `w` component.
		pub fn xyz(self) -> Vec3<T> { Vec3 { x: self.x, y: self.y, z: self.z } }
	};
}

macro_rules! ops {
	(Vec1) => {};
	(Vec2) => {
		/// Horizontal subtracts the components.
		pub fn hsub(self) -> T { self.x - self.y }
		/// Calculates the polar angle.
		pub fn polar_angle(self) -> Rad<T> where T: Float { Rad::atan2(self.y, self.x) }
		/// Rotates the vector counter-clockwise by 90°.
		pub fn ccw(self) -> Vec2<T> { Vec2 { x: self.y, y: -self.x } }
		/// Rotates the vector clockwise by 90°.
		pub fn cw(self) -> Vec2<T> { Vec2 { x: -self.y, y: self.x } }
		/// Calculates the 3D cross product where the inputs are extended with `z = 0` and returns the magnitude of the result.
		pub fn cross(self, rhs: Vec2<T>) -> T { self.x * rhs.y - self.y * rhs.x }
	};
	(Vec3) => {
		/// Calculates the 3D cross product.
		pub fn cross(self, rhs: Vec3<T>) -> Vec3<T> {
			Vec3 {
				x: self.y * rhs.z - self.z * rhs.y,
				y: self.z * rhs.x - self.x * rhs.z,
				z: self.x * rhs.y - self.y * rhs.x,
			}
		}
	};
	(Vec4) => {};
}

// This may or may not be horrible abuse of the `macro_rules!` system :)
macro_rules! vec {
	($vec:ident $N:tt { $($field:ident $I:tt $T:ident),+ }) => {

		//----------------------------------------------------------------
		// Constructors

		impl<T> $vec<T> {
			/// Constructs a new vector from components.
			pub fn new($($field: T),+) -> $vec<T> {
				$vec { $($field: $field),+ }
			}
			/// Constructs a new vector by broadcasting to all its components.
			pub fn dup(u: T) -> $vec<T> where T: Copy {
				$vec { $($field: u),+ }
			}
			unit!($vec);
		}

		impl<T> $vec<T> {
			set!($vec);
			cvt!($vec);
		}

		//----------------------------------------------------------------
		// Transformations

		impl<T> $vec<T> {
			/// Casts to a vector of different type with the same dimensions.
			pub fn cast<U>(self) -> $vec<U> where T: Cast<U> {
				$vec { $($field: self.$field.cast()),+ }
			}
			/// Maps a callable over the components.
			pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> $vec<U> {
				$vec { $($field: f(self.$field)),+ }
			}
			/// Zips two vectors together.
			pub fn zip<U, F: FnMut(T, T) -> U>(self, rhs: $vec<T>, mut f: F) -> $vec<U> {
				$vec { $($field: f(self.$field, rhs.$field)),+ }
			}
			/// Reduces the vector.
			pub fn reduce<F: Fn(T, T) -> T>(self, f: F) -> T {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, $(self.$field),+)
			}
			/// Folds the vector.
			pub fn fold<A, F: Fn(A, T) -> A>(self, acc: A, f: F) -> A {
				// These will end up nested without temporaries which won't work with `FnMut`...
				fold!(f, acc, $(self.$field),+)
			}
		}

		//----------------------------------------------------------------
		// Conversions

		impl<T> From<($($T,)+)> for $vec<T> {
			fn from(val: ($($T,)+)) -> $vec<T> {
				$vec { $($field: val.$I),+ }
			}
		}
		impl<T> Into<($($T,)+)> for $vec<T> {
			fn into(self) -> ($($T,)+) {
				($(self.$field,)+)
			}
		}

		impl<T: Copy> From<[T; $N]> for $vec<T> {
			fn from(val: [T; $N]) -> $vec<T> {
				// Can't move out of array...
				$vec { $($field: val[$I]),+ }
			}
		}
		impl<T> Into<[T; $N]> for $vec<T> {
			fn into(self) -> [T; $N] {
				[$(self.$field),+]
			}
		}

		//----------------------------------------------------------------
		// As references

		impl<T> AsRef<($($T,)+)> for $vec<T> {
			fn as_ref(&self) -> &($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T; $N]> for $vec<T> {
			fn as_ref(&self) -> &[T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsRef<[T]> for $vec<T> {
			fn as_ref(&self) -> &[T] {
				<Self as AsRef<[T; $N]>>::as_ref(self)
			}
		}
		
		impl<T> AsMut<($($T,)+)> for $vec<T> {
			fn as_mut(&mut self) -> &mut ($($T,)+) {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T; $N]> for $vec<T> {
			fn as_mut(&mut self) -> &mut [T; $N] {
				unsafe { mem::transmute(self) }
			}
		}
		impl<T> AsMut<[T]> for $vec<T> {
			fn as_mut(&mut self) -> &mut [T] {
				<Self as AsMut<[T; $N]>>::as_mut(self)
			}
		}

		//----------------------------------------------------------------
		// Operations

		impl<T: Scalar> $vec<T> {
			/// Squares the components.
			pub fn sqr(self) -> $vec<T> {
				$vec { $($field: self.$field * self.$field),+ }
			}
			/// Calculates the squared length of the vector.
			pub fn len_sqr(self) -> T {
				infix!(+ $(self.$field * self.$field),+)
			}
			/// Calculates the length of the vector.
			pub fn len(self) -> T where T: Float {
				self.len_sqr().sqrt()
			}
			/// Calculates the squared euclidean distance to another vector.
			pub fn dist_sqr(self, to: $vec<T>) -> T {
				infix!(+ $((to.$field - self.$field) * (to.$field - self.$field)),+)
			}
			/// Calculates the euclidean distance to another vector.
			pub fn dist(self, to: $vec<T>) -> T where T: Float {
				self.dist_sqr(to).sqrt()
			}
			/// Normalizes the vector.
			pub fn norm(self) -> $vec<T> where T: Float {
				let self_len = self.len();
				if self_len > T::zero() {
					self / self_len
				}
				else {
					Self::default()
				}
			}
			/// Scales the vector such that its length equals the given value.
			pub fn resize(self, len: T) -> $vec<T> where T: Float {
				let self_len = self.len();
				if self_len > T::zero() {
					self * (len / self_len)
				}
				else {
					Self::default()
				}
			}
			/// Scalar projection of `v` on `self`.
			pub fn scalar_project(self, v: $vec<T>) -> T where T: Float {
				self.dot(v) / self.len()
			}
			/// Projection of `v` on `self`.
			pub fn project(self, v: $vec<T>) -> $vec<T> where T: Float {
				self * (self.dot(v) / v.dot(v))
			}
			/// Horizontal adds all components.
			pub fn hadd(self) -> T {
				infix!(+ $(self.$field),+)
			}
			ops!($vec);
			/// Calculates the inner product.
			pub fn dot(self, rhs: $vec<T>) -> T {
				infix!(+ $(self.$field * rhs.$field),+)
			}
			/// Calculates the cosine of the inner angle.
			pub fn cos_angle(self, rhs: $vec<T>) -> T where T: Float {
				// |self| * |rhs| <=> √(self ∙ self * rhs ∙ rhs)
				let d = (self.dot(self) * rhs.dot(rhs)).sqrt();
				self.dot(rhs) / d
			}
			/// Calculates the inner angle.
			pub fn angle(self, rhs: $vec<T>) -> Rad<T> where T: Float {
				Rad::acos(self.cos_angle(rhs))
			}
		}

		//----------------------------------------------------------------
		// Operators

		impl<T> $vec<T> {
			/// Component wise absolute value.
			pub fn abs(self) -> $vec<T> where T: Abs<Output = T> {
				$vec { $($field: self.$field.abs()),+ }
			}
			/// Component wise minimum value.
			pub fn min(self, rhs: $vec<T>) -> $vec<T> where T: Min<Output = T> {
				$vec { $($field: T::min(self.$field, rhs.$field)),+ }
			}
			/// Component wise maximum value.
			pub fn max(self, rhs: $vec<T>) -> $vec<T> where T: Max<Output = T> {
				$vec { $($field: T::max(self.$field, rhs.$field)),+ }
			}
			/// Adds the scaled vector.
			pub fn mul_add(self, vec: $vec<T>, scale: T) -> $vec<T> where T: Scalar {
				$vec { $($field: self.$field + vec.$field * scale),+ }
			}
		}

		// Num traits
		impl<T: Abs> Abs for $vec<T> {
			type Output = $vec<T::Output>;
			fn abs(self) -> $vec<T::Output> {
				$vec { $($field: self.$field.abs()),+ }
			}
		}
		impl<U, T: Min<U>> Min<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn min(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field.min(rhs.$field)),+ }
			}
		}
		impl<U, T: Max<U>> Max<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn max(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field.max(rhs.$field)),+ }
			}
		}

		// Vector addition, subtraction and negation
		impl<U, T: ops::Add<U>> ops::Add<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn add(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field + rhs.$field),+ }
			}
		}
		impl<U, T: ops::Sub<U>> ops::Sub<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn sub(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field - rhs.$field),+ }
			}
		}
		impl<T: ops::Neg> ops::Neg for $vec<T> {
			type Output = $vec<T::Output>;
			fn neg(self) -> $vec<T::Output> {
				$vec { $($field: -self.$field),+ }
			}
		}

		// Scalar multiplication, division and remainder
		impl<U: Scalar, T: ops::Mul<U>> ops::Mul<U> for $vec<T> {
			type Output = $vec<T::Output>;
			fn mul(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field * rhs),+ }
			}
		}
		impl<U: Scalar, T: ops::Div<U>> ops::Div<U> for $vec<T> {
			type Output = $vec<T::Output>;
			fn div(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field / rhs),+ }
			}
		}
		impl<U: Scalar, T: ops::Rem<U>> ops::Rem<U> for $vec<T> {
			type Output = $vec<T::Output>;
			fn rem(self, rhs: U) -> $vec<T::Output> {
				$vec { $($field: self.$field % rhs),+ }
			}
		}

		// Vector multiplication, division and remainder
		impl<U, T: ops::Mul<U>> ops::Mul<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn mul(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field * rhs.$field),+ }
			}
		}
		impl<U, T: ops::Div<U>> ops::Div<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn div(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field / rhs.$field),+ }
			}
		}
		impl<U, T: ops::Rem<U>> ops::Rem<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn rem(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field % rhs.$field),+ }
			}
		}

		// Bitwise operators
		impl<U, T: ops::BitAnd<U>> ops::BitAnd<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn bitand(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field & rhs.$field),+ }
			}
		}
		impl<U, T: ops::BitOr<U>> ops::BitOr<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn bitor(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field | rhs.$field),+ }
			}
		}
		impl<U, T: ops::BitXor<U>> ops::BitXor<$vec<U>> for $vec<T> {
			type Output = $vec<T::Output>;
			fn bitxor(self, rhs: $vec<U>) -> $vec<T::Output> {
				$vec { $($field: self.$field ^ rhs.$field),+ }
			}
		}
		impl<T: ops::Not> ops::Not for $vec<T> {
			type Output = $vec<T::Output>;
			fn not(self) -> $vec<T::Output> {
				$vec { $($field: !self.$field),+ }
			}
		}

		//----------------------------------------------------------------
		// Formatting

		fmt!($vec { $($field),+ });
	}
}

// vec!(Vec1 1 { x 0 T });
vec!(Vec2 2 { x 0 T, y 1 T });
vec!(Vec3 3 { x 0 T, y 1 T, z 2 T });
vec!(Vec4 4 { x 0 T, y 1 T, z 2 T, w 3 T });

//----------------------------------------------------------------
// Packed integers

impl Vec2<u32> {
	/// Unpack `u64` into `u32 u32`.
	pub fn unpack32(v: u64) -> Vec2<u32> {
		Vec2 {
			x: ((v & 0x00000000FFFFFFFF) >> 0) as u32,
			y: ((v & 0xFFFFFFFF00000000) >> 32) as u32,
		}
	}
	/// Pack into `u64`.
	pub fn pack(self) -> u64 {
		(self.y as u64) << 32 | (self.x as u64)
	}
}
impl Vec2<u16> {
	/// Unpack `u32` into `u16 u16`.
	pub fn unpack16(v: u32) -> Vec2<u16> {
		Vec2 {
			x: ((v & 0x0000FFFF) >> 0) as u16,
			y: ((v & 0xFFFF0000) >> 16) as u16,
		}
	}
	/// Pack into `u32`.
	pub fn pack(self) -> u32 {
		(self.y as u32) << 16 | (self.x as u32)
	}
}
impl Vec2<u8> {
	/// Unpack `u16` into `u8 u8`.
	pub fn unpack8(v: u16) -> Vec2<u8> {
		Vec2 {
			x: ((v as u32 & 0x000000FF) >> 0) as u8,
			y: ((v as u32 & 0x0000FF00) >> 8) as u8,
		}
	}
	/// Pack into `u16`.
	pub fn pack(self) -> u16 {
		((self.y as u32) << 8 | (self.x as u32)) as u16
	}
}
impl Vec4<u16> {
	/// Unpack `u64` into `u16 u16 u16 u16`.
	pub fn unpack16(v: u64) -> Vec4<u16> {
		Vec4 {
			x: ((v & 0x000000000000FFFF) >> 0) as u16,
			y: ((v & 0x00000000FFFF0000) >> 16) as u16,
			z: ((v & 0x0000FFFF00000000) >> 32) as u16,
			w: ((v & 0xFFFF000000000000) >> 48) as u16,
		}
	}
	/// Pack into `u64`.
	pub fn pack(self) -> u64 {
		(self.w as u64) << 48 | (self.z as u64) << 32 | (self.y as u64) << 16 | (self.x as u64)
	}
}
impl Vec4<u8> {
	/// Unpack `u32` into `u8 u8 u8 u8`.
	pub fn unpack8(v: u32) -> Vec4<u8> {
		Vec4 {
			x: ((v & 0x000000FF) >> 0) as u8,
			y: ((v & 0x0000FF00) >> 8) as u8,
			z: ((v & 0x00FF0000) >> 16) as u8,
			w: ((v & 0xFF000000) >> 24) as u8,
		}
	}
	/// Pack into `u32`.
	pub fn pack(self) -> u32 {
		(self.w as u32) << 24 | (self.z as u32) << 16 | (self.y as u32) << 8 | (self.x as u32)
	}
}
