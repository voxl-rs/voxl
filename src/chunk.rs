use cgmath::Point3;
use std::{
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ChunkVec<T: Unit, A: Accessor> {
    data: Vec<T>,
    state: PhantomData<A>,
}

impl<A: Accessor, T: Unit> ChunkVec<T, A> {
    pub fn with_capacity(size: usize) -> Self {
        Self {
            data: Vec::with_capacity(size),
            state: PhantomData::default(),
        }
    }
}

impl<A: Accessor, T: Unit> Index<[usize; 3]> for ChunkVec<T, A> {
    type Output = T;

    fn index(&self, pos: [usize; 3]) -> &Self::Output {
        &self.data[A::to_index(pos)]
    }
}

impl<A: Accessor, T: Unit> IndexMut<[usize; 3]> for ChunkVec<T, A> {
    fn index_mut(&mut self, pos: [usize; 3]) -> &mut Self::Output {
        &mut self.data[A::to_index(pos)]
    }
}

impl<A: Accessor, T: Unit, const N: usize> From<Chunk<A, T, N>> for ChunkVec<T, A> {
    fn from(c: Chunk<A, T, N>) -> Self {
        Self {
            data: c.data.into(),
            state: PhantomData::default(),
        }
    }
}

/// Generic chunk data
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Chunk<A: Accessor, T: Unit, const N: usize> {
    data: [T; N],
    state: PhantomData<A>,
}

impl<A: Accessor, T: Unit, const N: usize> From<[T; N]> for Chunk<A, T, N> {
    fn from(data: [T; N]) -> Self {
        Self {
            data,
            state: PhantomData::default(),
        }
    }
}

impl<A: Accessor, T: Unit, const N: usize> Chunk<A, T, N> {
    pub fn iter_slice(&self) -> Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_slice_mut(&mut self) -> IterMut<'_, T> {
        self.data.iter_mut()
    }

    /// Mesh
    pub fn mesh<F: Fn(&T) -> bool>(&self, f: F) {
        use std::collections::HashSet;
        let data = self
            .iter_slice()
            .enumerate()
            .flat_map(|(i, b)| {
                if f(b) {
                    let [y, x, z] = A::from_index(i);

                    return vec![
                        Point3::new(y, x, z),
                        Point3::new(y, x + 1, z),
                        Point3::new(y, x + 1, z + 1),
                        Point3::new(y, x, z + 1),
                        //
                        Point3::new(y + 1, x, z),
                        Point3::new(y + 1, x + 1, z),
                        Point3::new(y + 1, x + 1, z + 1),
                        Point3::new(y + 1, x, z + 1),
                    ];
                }

                vec![]
            })
            .collect::<HashSet<Point3<usize>>>();

        log::debug!("Generated Points: {:?}", data);
        println!("Generated Points: {:?}", data);
    }
}

impl<A: Accessor, T: Unit, const N: usize> Default for Chunk<A, T, N> {
    fn default() -> Self {
        Self {
            data: [T::default(); N],
            ..Default::default()
        }
    }
}

use evmap::shallow_copy::ShallowCopy;
use std::mem::ManuallyDrop;
impl<D: Accessor, T: Unit, const N: usize> ShallowCopy for Chunk<D, T, N> {
    unsafe fn shallow_copy(&self) -> ManuallyDrop<Self> {
        ManuallyDrop::new(*self)
    }
}

impl<A: Accessor, T: Unit, const N: usize> Index<[usize; 3]> for Chunk<A, T, N> {
    type Output = T;

    fn index(&self, pos: [usize; 3]) -> &Self::Output {
        &self.data[A::to_index(pos)]
    }
}

impl<A: Accessor, T: Unit, const N: usize> IndexMut<[usize; 3]> for Chunk<A, T, N> {
    fn index_mut(&mut self, pos: [usize; 3]) -> &mut Self::Output {
        &mut self.data[A::to_index(pos)]
    }
}

pub trait ChunkPlots: Accessor {
    const SIDE_VERT: usize;
    const NUM_VERTS: usize = Self::SIDE_VERT * Self::SIDE_VERT * Self::SIDE_VERT;
}

impl<A: Accessor> ChunkPlots for A {
    const SIDE_VERT: usize = A::SIDE_LEN + 1;
}

pub trait ChunkWireFrame: ChunkPlots {
    fn indices() -> Vec<u16>;
    fn vertices() -> Vec<u16>;
}

/// Used for tracking Chunk Dimensions.
/// An accessor trait is necessary because of a limitation
/// of min_const_generics where we can't predetermine the Chunk size
/// of the data type through 'static expressions `struct Chunk<D, T, N * N * N>`
pub trait Accessor: Clone + Copy + Eq + std::hash::Hash {
    /// No. of elements in a row/column
    const SIDE_LEN: usize;
    /// No. of elements in a plane
    const QUAD_LEN: usize = Self::SIDE_LEN * Self::SIDE_LEN;
    /// No. of all elements in represented with a cube
    const CUBE_LEN: usize = Self::QUAD_LEN * Self::SIDE_LEN;

    /// returns components in this order YXZ
    fn from_index(i: usize) -> [usize; 3] {
        let y = i / Self::QUAD_LEN;
        let z = (i - y * Self::QUAD_LEN) / Self::SIDE_LEN;
        let x = i - (z * Self::SIDE_LEN + y * Self::QUAD_LEN);

        [y, x, z]
    }

    #[inline(always)]
    fn to_index([y, x, z]: [usize; 3]) -> usize {
        y * Self::QUAD_LEN + x + z * Self::SIDE_LEN
    }
}

/// Ideal type to represent each chunk element for safely working with ECS
pub trait Unit:
    'static + Debug + Default + Clone + Copy + Send + Sync + Eq + std::hash::Hash
{
}

impl<T> Unit for T where
    T: 'static + Debug + Default + Clone + Copy + Send + Sync + Eq + std::hash::Hash
{
}

#[derive(Debug)]
pub struct ChunkFlatIter<D: Accessor, T: Unit, const N: usize> {
    index: usize,
    chunk: [T; N],
    state_a: PhantomData<D>,
    state_b: PhantomData<T>,
}

impl<D: Accessor, T: Unit, const N: usize> Iterator for ChunkFlatIter<D, T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < N {
            self.index += 1;

            return Some(self.chunk[self.index]);
        }

        None
    }
}
