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
enum ParticleType {
    sand,
    water,
}
struct Particle{
    ptype: ParticleType, 
    x: usize,
    y: usize,
    color:Rgb,
}
impl Particle {
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }
    pub fn color(&self) -> Rgb {
        self.color
    }
}
#[wasm_bindgen]
pub struct Image{
    width: usize,
    height: usize,
    cell_size: usize,
    cells: Vec<Rgb>,
    particles: Vec<Particle>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, cell_size:usize) -> Image{
        //let cells = vec![vec![Rgb { r: 200, g: 200, b: 255 }; height]; width];
        //let cells = vec![Particle{ptype: ParticleType::empty, color:Rgb { r: 200, g: 200, b: 255 }}; width * height];
        let cells = vec![Rgb { r: 200, g: 200, b: 255 }; width * height];
        let particles:Vec<Particle> = Vec::new();
        Image {width, height, cell_size, cells, particles} 
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
        //self.cells[x][y] = Rgb{r:color[0], g:color[1], b:color[2]};
    }
    pub fn addParticle(&mut self, x:usize, y:usize){
        let new_particle = Particle{ptype:ParticleType::sand, x:x, y:y, color:Rgb{ r: 255, g: 200, b: 200 }};
        self.particles.push(new_particle);
    }
    pub fn updateColor(&mut self) {
        let mut new_cells = vec![Rgb { r: 200, g: 200, b: 255 }; self.width() * self.height()];
        let iter = self.particles.iter();
        for particle in iter {
            let index = (particle.y()*self.width)+particle.x();
            let color = particle.color();
            new_cells[index] = color;
        }
        self.cells = new_cells;
    }
    pub fn updateParticle(&mut self){
        let height = self.height;
        let iter = self.particles.iter_mut();

        for particle in iter {
            particle.x = particle.x();
            if particle.y() + 1 != height {
                particle.y = particle.y() + 1;
            } else {
                particle.y = particle.y();
            }
        } 
    }
}