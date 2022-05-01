// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod stardust_xr {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

// struct vec3, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct vec3(pub [u8; 12]);
impl Default for vec3 { 
  fn default() -> Self { 
	Self([0; 12])
  }
}
impl std::fmt::Debug for vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	f.debug_struct("vec3")
	  .field("x", &self.x())
	  .field("y", &self.y())
	  .field("z", &self.z())
	  .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for vec3 {}
impl flatbuffers::SafeSliceAccess for vec3 {}
impl<'a> flatbuffers::Follow<'a> for vec3 {
  type Inner = &'a vec3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	<&'a vec3>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a vec3 {
  type Inner = &'a vec3;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	flatbuffers::follow_cast_ref::<vec3>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for vec3 {
	type Output = vec3;
	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(self as *const vec3 as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}
impl<'b> flatbuffers::Push for &'b vec3 {
	type Output = vec3;

	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(*self as *const vec3 as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}

impl<'a> flatbuffers::Verifiable for vec3 {
  #[inline]
  fn run_verifier(
	v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
	use self::flatbuffers::Verifiable;
	v.in_buffer::<Self>(pos)
  }
}

impl<'a> vec3 {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
	x: f32,
	y: f32,
	z: f32,
  ) -> Self {
	let mut s = Self([0; 12]);
	s.set_x(x);
	s.set_y(y);
	s.set_z(z);
	s
  }

  pub fn x(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[0..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_x(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[0..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

  pub fn y(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[4..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_y(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[4..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

  pub fn z(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[8..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_z(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[8..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

}

// struct quat, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct quat(pub [u8; 16]);
impl Default for quat { 
  fn default() -> Self { 
	Self([0; 16])
  }
}
impl std::fmt::Debug for quat {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	f.debug_struct("quat")
	  .field("x", &self.x())
	  .field("y", &self.y())
	  .field("z", &self.z())
	  .field("w", &self.w())
	  .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for quat {}
impl flatbuffers::SafeSliceAccess for quat {}
impl<'a> flatbuffers::Follow<'a> for quat {
  type Inner = &'a quat;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	<&'a quat>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a quat {
  type Inner = &'a quat;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	flatbuffers::follow_cast_ref::<quat>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for quat {
	type Output = quat;
	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(self as *const quat as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}
impl<'b> flatbuffers::Push for &'b quat {
	type Output = quat;

	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(*self as *const quat as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}

impl<'a> flatbuffers::Verifiable for quat {
  #[inline]
  fn run_verifier(
	v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
	use self::flatbuffers::Verifiable;
	v.in_buffer::<Self>(pos)
  }
}

impl<'a> quat {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
	x: f32,
	y: f32,
	z: f32,
	w: f32,
  ) -> Self {
	let mut s = Self([0; 16]);
	s.set_x(x);
	s.set_y(y);
	s.set_z(z);
	s.set_w(w);
	s
  }

  pub fn x(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[0..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_x(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[0..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

  pub fn y(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[4..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_y(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[4..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

  pub fn z(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[8..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_z(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[8..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

  pub fn w(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[12..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_w(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[12..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

}

// struct pose, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct pose(pub [u8; 28]);
impl Default for pose { 
  fn default() -> Self { 
	Self([0; 28])
  }
}
impl std::fmt::Debug for pose {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	f.debug_struct("pose")
	  .field("position", &self.position())
	  .field("rotation", &self.rotation())
	  .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for pose {}
impl flatbuffers::SafeSliceAccess for pose {}
impl<'a> flatbuffers::Follow<'a> for pose {
  type Inner = &'a pose;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	<&'a pose>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a pose {
  type Inner = &'a pose;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	flatbuffers::follow_cast_ref::<pose>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for pose {
	type Output = pose;
	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(self as *const pose as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}
impl<'b> flatbuffers::Push for &'b pose {
	type Output = pose;

	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(*self as *const pose as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}

impl<'a> flatbuffers::Verifiable for pose {
  #[inline]
  fn run_verifier(
	v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
	use self::flatbuffers::Verifiable;
	v.in_buffer::<Self>(pos)
  }
}

impl<'a> pose {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
	position: &vec3,
	rotation: &quat,
  ) -> Self {
	let mut s = Self([0; 28]);
	s.set_position(position);
	s.set_rotation(rotation);
	s
  }

  pub fn position(&self) -> &vec3 {
	unsafe { &*(self.0[0..].as_ptr() as *const vec3) }
  }

  #[allow(clippy::identity_op)]
  pub fn set_position(&mut self, x: &vec3) {
	self.0[0..0 + 12].copy_from_slice(&x.0)
  }

  pub fn rotation(&self) -> &quat {
	unsafe { &*(self.0[12..].as_ptr() as *const quat) }
  }

  #[allow(clippy::identity_op)]
  pub fn set_rotation(&mut self, x: &quat) {
	self.0[12..12 + 16].copy_from_slice(&x.0)
  }

}

// struct joint, aligned to 4
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct joint(pub [u8; 32]);
impl Default for joint { 
  fn default() -> Self { 
	Self([0; 32])
  }
}
impl std::fmt::Debug for joint {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	f.debug_struct("joint")
	  .field("position", &self.position())
	  .field("rotation", &self.rotation())
	  .field("radius", &self.radius())
	  .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for joint {}
impl flatbuffers::SafeSliceAccess for joint {}
impl<'a> flatbuffers::Follow<'a> for joint {
  type Inner = &'a joint;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	<&'a joint>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a joint {
  type Inner = &'a joint;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
	flatbuffers::follow_cast_ref::<joint>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for joint {
	type Output = joint;
	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(self as *const joint as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}
impl<'b> flatbuffers::Push for &'b joint {
	type Output = joint;

	#[inline]
	fn push(&self, dst: &mut [u8], _rest: &[u8]) {
	let src = unsafe {
	::std::slice::from_raw_parts(*self as *const joint as *const u8, Self::size())
	};
	dst.copy_from_slice(src);
	}
}

impl<'a> flatbuffers::Verifiable for joint {
  #[inline]
  fn run_verifier(
	v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
	use self::flatbuffers::Verifiable;
	v.in_buffer::<Self>(pos)
  }
}

impl<'a> joint {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
	position: &vec3,
	rotation: &quat,
	radius: f32,
  ) -> Self {
	let mut s = Self([0; 32]);
	s.set_position(position);
	s.set_rotation(rotation);
	s.set_radius(radius);
	s
  }

  pub fn position(&self) -> &vec3 {
	unsafe { &*(self.0[0..].as_ptr() as *const vec3) }
  }

  #[allow(clippy::identity_op)]
  pub fn set_position(&mut self, x: &vec3) {
	self.0[0..0 + 12].copy_from_slice(&x.0)
  }

  pub fn rotation(&self) -> &quat {
	unsafe { &*(self.0[12..].as_ptr() as *const quat) }
  }

  #[allow(clippy::identity_op)]
  pub fn set_rotation(&mut self, x: &quat) {
	self.0[12..12 + 16].copy_from_slice(&x.0)
  }

  pub fn radius(&self) -> f32 {
	let mut mem = core::mem::MaybeUninit::<f32>::uninit();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	self.0[28..].as_ptr(),
	mem.as_mut_ptr() as *mut u8,
	core::mem::size_of::<f32>(),
	  );
	  mem.assume_init()
	}.from_little_endian()
  }

  pub fn set_radius(&mut self, x: f32) {
	let x_le = x.to_little_endian();
	unsafe {
	  core::ptr::copy_nonoverlapping(
	&x_le as *const f32 as *const u8,
	self.0[28..].as_mut_ptr(),
	core::mem::size_of::<f32>(),
	  );
	}
  }

}

}  // pub mod StardustXR

