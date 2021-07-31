//! Entities for interacting with mesh-produced output (vertices, faces(indices) and normals)
use std::fmt;

use ttf2mesh_sys as sys;

pub trait IteratorValue<'a> {
    type Output: fmt::Debug;

    fn value(&self) -> Self::Output;
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh__bindgen_ty_1 {
    type Output = (f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn value(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_1 {
    type Output = (f32, f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_2 {
    type Output = (i32, i32, i32);

    fn value(&self) -> Self::Output {
        (self.v1, self.v2, self.v3)
    }
}

impl<'a> IteratorValue<'a> for sys::ttf_mesh3d__bindgen_ty_3 {
    type Output = (f32, f32, f32);

    fn value(&self) -> Self::Output {
        (self.x, self.y, self.z)
    }
}

/// An iterator over an array of mesh values (separate output type for 2d vertices, 3d vertices, faces and normals)
///
/// Usage:
/// ```rust
/// # use ttf2mesh::{TTFFile, Quality};
/// # let mut ttf = TTFFile::from_file("./fonts/FiraMono-Medium.ttf").unwrap();
/// # let mut glyph = ttf.glyph_from_char('€').unwrap();
/// # let mut mesh_2d = glyph.to_2d_mesh(Quality::Medium).unwrap();
/// #
/// let vertices = mesh_2d.iter_vertices();
/// for vertice in vertices {
///     let value: (f32, f32) = vertice.val();
/// }
/// ```
pub struct MeshIterator<'a, T: IteratorValue<'a>> {
    index: usize,
    iterable: &'a [T],
}

impl<'a, T: IteratorValue<'a>> MeshIterator<'a, T> {
    pub(crate) fn new(iterable: &'a [T]) -> Self {
        Self { index: 0, iterable }
    }
}

impl<'a, T: IteratorValue<'a>> Iterator for MeshIterator<'a, T> {
    type Item = Value<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterable.get(self.index) {
            Some(item) => {
                self.index += 1;

                Some(Value { inner: item })
            }
            None => None,
        }
    }
}

/// Wrapper for an iterator value. Use [`Value::val`] to get inner value
pub struct Value<'a, T> {
    inner: &'a T,
}

impl<'a, T: IteratorValue<'a>> Value<'a, T> {
    pub fn val(&self) -> T::Output {
        self.inner.value()
    }
}

impl<'a, T: IteratorValue<'a>> fmt::Debug for Value<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Value {{ {:?} }}", self.val())
    }
}