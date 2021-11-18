use wasm_bindgen::prelude::*;
//use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     // This provides better error messages in debug mode.
//     // It's disabled in release mode so it doesn't bloat up the file size.
//     #[cfg(debug_assertions)]
//     console_error_panic_hook::set_once();


//     // Your code goes here!
//     console::log_1(&JsValue::from_str("Hello world!"));

//     Ok(())
// }
#[derive(Clone, Copy)]
struct Rgb {
    r:u8,
    g:u8,
    b:u8,
}
#[wasm_bindgen]
pub struct Image{
    width: usize,
    height: usize,
    cell_size: usize,
    cells: Vec<Rgb>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, cell_size:usize) -> Image{
        let cells =  vec![Rgb { r: 200, g: 200, b: 255 }; width * height];
        Image {width, height, cell_size, cells} 
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn cell_size(&self) -> usize {
        self.cell_size
    }
    pub fn cells(&self) -> Vec<u8>{
        self.cells
        .iter()
        .map(|&rgb| vec![rgb.r, rgb.g, rgb.b])
        .collect::<Vec<Vec<u8>>>()
        .concat()
    }
    pub fn brush(&mut self, x:usize, y:usize, color:Vec<u8>){
        let index = (y*self.width)+x;
        self.cells[index] = Rgb{r:color[0], g:color[1], b:color[2]};
    }
}