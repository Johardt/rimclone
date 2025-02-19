use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use crate::math::matrix::Matrix;

use super::tile::Tile;

pub const CHUNK_SIZE: usize = 32;

/// A chunk is a region of CHUNK_SIZExCHUNK_SIZE tiles.
/// Multiple chunks form a BoardLayer, and multiple
/// BoardLayers then form a Board. The chunk exists
/// mainly for optimization reasons.
#[derive(Component, Clone)]
pub struct Chunk {
    tile_data: Matrix<Option<Tile>>,
}

impl Default for Chunk {
    fn default() -> Self {
        let mut tiles = Matrix::new(CHUNK_SIZE, CHUNK_SIZE);
        tiles.iter_mut().for_each(|tile| {
            if rand::random::<f32>() < 0.7 {
                *tile = Some(Tile::default());
            } else {
                *tile = None;
            }
        });

        Chunk { tile_data: tiles }
    }
}

impl Chunk {
    pub fn build_chunk_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut uv: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let tile_size = 32.0;
        let mut vertex_index: u32 = 0;

        for row in 0..CHUNK_SIZE {
            for col in 0..CHUNK_SIZE {
                if self.tile_data.get(row, col).is_some() {
                    // Compute the bottom-left corner position of the tile.
                    let x = col as f32 * tile_size;
                    let y = row as f32 * tile_size;
                    // Four corners for a quad in 2D (z is 0)
                    positions.push([x, y, 0.0]); // bottom-left
                    positions.push([x + tile_size, y, 0.0]); // bottom-right
                    positions.push([x + tile_size, y + tile_size, 0.0]); // top-right
                    positions.push([x, y + tile_size, 0.0]); // top-left

                    // Since every tile uses the same texture full (tile.png), set UVs to cover it fully.
                    uv.push([0.0, 0.0]);
                    uv.push([1.0, 0.0]);
                    uv.push([1.0, 1.0]);
                    uv.push([0.0, 1.0]);

                    // Two triangles per quad (six indices)
                    indices.push(vertex_index);
                    indices.push(vertex_index + 1);
                    indices.push(vertex_index + 2);

                    indices.push(vertex_index);
                    indices.push(vertex_index + 2);
                    indices.push(vertex_index + 3);

                    vertex_index += 4;
                }
            }
        }

        // Insert the vertex data into the mesh.
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        mesh.insert_indices(Indices::U32(indices));

        // Add the mesh to the asset collection.
        let _mesh_handle = &meshes.add(mesh);
    }
}
