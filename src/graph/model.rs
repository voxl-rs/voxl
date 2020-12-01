use std::collections::HashMap;

use crate::{
    graph::texture::Texture,
    math::cg::Point3,
    vox::{
        chunk::{Accessor, Unit},
        Chunk,
    },
};

/// Can represent a chunk/compilation of chunks
#[derive(Debug)]
pub struct VoxlModel<'a, D, T, const N: usize>
where
    D: Accessor,
    T: Unit,
{
    chunks: HashMap<Point3<u128>, &'a Chunk<D, T, N>>,
    reference: Point3<u128>,
}

impl<'a, D, T, const N: usize> Default for VoxlModel<'a, D, T, N>
where
    D: Accessor,
    T: Unit,
{
    fn default() -> Self {
        Self {
            chunks: HashMap::with_capacity(128),
            reference: Point3::new(0, 0, 0),
        }
    }
}

impl<'a, D, T, const N: usize> VoxlModel<'a, D, T, N>
where
    D: Accessor,
    T: Unit,
{
}

/// A Block Face
#[derive(Debug)]
pub struct Face<'a> {
    texture: &'a Texture,
}
