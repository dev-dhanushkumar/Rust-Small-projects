use std::{iter::FromIterator, ops::Index};

use wasm_bindgen::prelude::*;
use im::Vector;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Rgb {
    r:u8,
    g:u8,
    b:u8,
}

#[wasm_bindgen]
#[derive(Clone)]
#[allow(dead_code)]
pub struct Image {
    width: usize,
    height: usize,
    cells: Vector<Rgb>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize,height: usize) -> Image {
        let  cells = Vector::from_iter((0..width * height).map(|_| 
            Rgb {
                r:200,
                g:200,
                b:255,
            }
        ));
        Image {width,
            height, 
            cells
        }
    }

    pub fn cells(&self) -> Vec<u8> {
        self.cells
            .iter()
            .map(|&rgb| vec![rgb.r, rgb.g, rgb.b])
            .collect::<Vec<Vec<u8>>>()
            .concat()
    }
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn brush(&mut self, x:usize, y: usize, color: Vec<u8>) -> Image{
        let index  = (y * self.width) + x;
        let new_cells = self.cells.update(index,Rgb { 
            r:color[0],
            g:color[1], 
            b:color[2]
        });

        Image {
            width: self.width,
            height: self.height,
            cells: new_cells,
        }
    }
}


struct UndoQueue<T: Clone> {
    queue: Vec<T>,
    index: usize,
}

impl<T: Clone> UndoQueue<T> {
    pub fn new(entry: T) -> UndoQueue<T>{
        UndoQueue {
            queue: vec![entry],
            index: 0,
        }
    }

    pub fn current(&self) -> T {
        self.queue[self.index].Clone()
    }
}


#[wasm_bindgen]
struct  InternalState {
    undo_queue: UndoQueue<Image>,
}

#[wasm_bindgen]
impl InternalState {
    #[wasm_bindgen(constructor)]
    pub fn new (width: usize, height: usize,) -> InternalState {
        InternalState {
            undo_queue: UndoQueue::new(Image::new(width, height)),
        }
    }

    pub fn image(&self) -> Image {
        self.undo_queue.current();
    }
}