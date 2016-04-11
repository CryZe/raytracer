use std::ops::{Index, IndexMut};
use std::slice;
use prelude::*;
use RgbaImage;

pub struct Image {
    pub dimensions: (usize, usize),
    field: Vec<Rgb>,
}

impl Index<(usize, usize)> for Image {
    type Output = Rgb;

    fn index(&self, (x, y): (usize, usize)) -> &Rgb {
        let (nx, _) = self.dimensions;
        let index = x + nx * y;
        &self.field[index]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Rgb {
        let (nx, _) = self.dimensions;
        let index = x + nx * y;
        &mut self.field[index]
    }
}

impl Image {
    pub fn new((nx, ny): (usize, usize)) -> Self {
        Image {
            dimensions: (nx, ny),
            field: vec![Rgb::new(0.0, 0.0, 0.0); nx * ny],
        }
    }

    pub fn contains(&self, (x, y): (usize, usize)) -> bool {
        let (nx, ny) = self.dimensions;
        x < nx && y < ny
    }

    pub fn set(&mut self, coord: (usize, usize), value: Rgb) {
        if self.contains(coord) {
            self[coord] = value;
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            iter: self.field.iter_mut(),
            nx: self.dimensions.0,
            x: 0,
            y: 0,
        }
    }

    pub fn iter_inner_mut(&mut self) -> IterInnerMut {
        let field = &mut self.field[..(self.dimensions.1 - 1) * self.dimensions.0];
        let field = &mut field[self.dimensions.0 + 1..];
        IterInnerMut {
            iter: field.iter_mut(),
            nx: self.dimensions.0 - 1,
            x: 1,
            y: 1,
        }
    }

    pub fn chunks_mut(&mut self, count: usize) -> ChunksMut {
        let chunk_size = (self.field.len() as f32 / count as f32).ceil() as usize;
        ChunksMut {
            iter: self.field.chunks_mut(chunk_size),
            nx: self.dimensions.0,
            x: 0,
            y: 0,
        }
    }

    pub fn chunks_inner_mut(&mut self, count: usize) -> ChunksInnerMut {
        let len = self.field.len();
        let sliced_field = &mut self.field[self.dimensions.0..len - self.dimensions.0];
        let cell_count = sliced_field.len();
        let chunk_size = (cell_count as f32 / count as f32).ceil() as usize;
        ChunksInnerMut {
            iter: sliced_field.chunks_mut(chunk_size),
            nx: self.dimensions.0,
            x: 0,
            y: 1,
        }
    }

    pub fn to_rgba_image(&self, gamma: f32) -> RgbaImage {
        use std::f32;

        let (nx, ny) = self.dimensions;
        let mut image = RgbaImage::new(nx as u32, ny as u32);

        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let value = self[(x as usize, y as usize)];
            let (r, g, b) = (255.0 * f32::powf(value.red.saturate(), 1.0 / gamma),
                             255.0 * f32::powf(value.green.saturate(), 1.0 / gamma),
                             255.0 * f32::powf(value.blue.saturate(), 1.0 / gamma));
            pixel.data = [r as u8, g as u8, b as u8, 0xFF];
        }

        image
    }
}

pub struct IterMut<'field> {
    iter: slice::IterMut<'field, Rgb>,
    nx: usize,
    x: usize,
    y: usize,
}

impl<'field> Iterator for IterMut<'field> {
    type Item = ((usize, usize), &'field mut Rgb);
    fn next(&mut self) -> Option<((usize, usize), &'field mut Rgb)> {
        let value = self.iter.next();
        if let Some(value) = value {
            let result = ((self.x, self.y), value);
            self.x += 1;
            if self.x >= self.nx {
                self.x = 0;
                self.y += 1;
            }
            Some(result)
        } else {
            None
        }
    }
}

pub struct ChunksMut<'field> {
    iter: slice::ChunksMut<'field, Rgb>,
    nx: usize,
    x: usize,
    y: usize,
}

impl<'field> Iterator for ChunksMut<'field> {
    type Item = IterMut<'field>;
    fn next(&mut self) -> Option<IterMut<'field>> {
        let chunk = self.iter.next();
        if let Some(chunk) = chunk {
            let len = chunk.len();
            let result = IterMut {
                iter: chunk.iter_mut(),
                nx: self.nx,
                x: self.x,
                y: self.y,
            };
            self.x += len;
            self.y += self.x / self.nx;
            self.x = self.x % self.nx;
            Some(result)
        } else {
            None
        }
    }
}

pub struct IterInnerMut<'field> {
    iter: slice::IterMut<'field, Rgb>,
    nx: usize,
    x: usize,
    y: usize,
}

impl<'field> Iterator for IterInnerMut<'field> {
    type Item = ((usize, usize), &'field mut Rgb);
    fn next(&mut self) -> Option<((usize, usize), &'field mut Rgb)> {
        let value = self.iter.next();
        if let Some(value) = value {
            let result = ((self.x, self.y), value);
            self.x += 1;
            if self.x >= self.nx {
                self.x = 1;
                self.y += 1;
                self.iter.next();
                self.iter.next();
            }
            Some(result)
        } else {
            None
        }
    }
}

pub struct ChunksInnerMut<'field> {
    iter: slice::ChunksMut<'field, Rgb>,
    nx: usize,
    x: usize,
    y: usize,
}

impl<'field> Iterator for ChunksInnerMut<'field> {
    type Item = IterInnerMut<'field>;
    fn next(&mut self) -> Option<IterInnerMut<'field>> {
        let chunk = self.iter.next();
        if let Some(chunk) = chunk {
            let chunk = if self.x == self.nx {
                self.x = 1;
                self.y += 1;
                &mut chunk[2..]
            } else if self.x == 0 {
                self.x = 1;
                &mut chunk[1..]
            } else {
                chunk
            };
            let len = chunk.len();
            let result = IterInnerMut {
                iter: chunk.iter_mut(),
                nx: self.nx - 1,
                x: self.x,
                y: self.y,
            };
            self.x += len;
            self.y += self.x / self.nx;
            self.x = self.x % self.nx;
            Some(result)
        } else {
            None
        }
    }
}
