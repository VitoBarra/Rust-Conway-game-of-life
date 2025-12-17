use std::ffi::c_void;
use std::thread;
use std::time::Duration;
use minifb::{Window, WindowOptions};




const CELL_DIMENSION : usize = 25;
const ROW_SIZE: usize = 25;
const COLUMN_SIZE: usize = 25;

const HEIGHT: usize = CELL_DIMENSION*COLUMN_SIZE;
const WIDTH: usize = CELL_DIMENSION*ROW_SIZE;

const PADDING : usize = 1;
    fn drow_cell(row:usize, col:usize, alive:bool, buffer: &mut Vec<u32>) -> ()
{
    for y in (row*CELL_DIMENSION+PADDING)..((row+1)*CELL_DIMENSION-PADDING){
        for  x in (col*CELL_DIMENSION+PADDING)..((col+1)*CELL_DIMENSION-PADDING){
            let r = if alive { 255 } else { 0 };
            let g = if alive { 255 } else { 0 };
            let b = if alive { 255 } else { 0 };

            buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b;
        }
    }
}

fn main() {
    let mut cellPresence = vec![vec![false; ROW_SIZE]; COLUMN_SIZE];
    let mut buffer = vec![0u32; HEIGHT * WIDTH];
    let mut window = Window::new("Drawing Window", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    // Draw a gradient
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y % CELL_DIMENSION == 0 || x % CELL_DIMENSION == 0
            {
                let r = 255;
                let g = 255;
                let b = 255;

                // RGB to u32 (0xRRGGBB)
                buffer[y * WIDTH + x] = (r << 16) | (g << 8) | b;
            }
        }
    }
    cellPresence[15][15] = true;
    cellPresence[15][16] = true;
    cellPresence[15][17] = true;
    cellPresence[16][16] = true;
    let mut next_cell_presence = cellPresence.clone();

    while window.is_open() {
        // Check mouse click
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                // Convert mouse coordinates to cell indices
                let r = (mouse_y as usize) / CELL_DIMENSION;
                let c = (mouse_x as usize) / CELL_DIMENSION;

                if r < CELL_DIMENSION && c < CELL_DIMENSION {
                    cellPresence[r][c] = true; // spawn cell
                }
            }
        }
         
        //Draw all cells
        for r in 0..CELL_DIMENSION {
            for c in 0..CELL_DIMENSION {
                drow_cell(r,c,cellPresence[r][c], &mut buffer);
            }
        }

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        thread::sleep(Duration::from_millis(500));

        // Conway’s Game of Life rules:
        // 1. Any live cell with fewer than two live neighbors dies (underpopulation).
        // 2. Any live cell with two or three live neighbors lives on to the next generation.
        // 3. Any live cell with more than three live neighbors dies (overpopulation).
        // 4. Any dead cell with exactly three live neighbors becomes a live cell (reproduction).

        for r in 0..CELL_DIMENSION {
            for c in 0..CELL_DIMENSION {
                let mut alive_count =0u32;
                for (dr, dc) in (-1..=1).flat_map(|dr| (-1..=1).map(move |dc| (dr, dc))) {
                     if dr == 0 && dc == 0 {
                         continue;
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;

                    if nr >= 0 && nr < CELL_DIMENSION as i32 && nc >= 0 && nc < CELL_DIMENSION as i32 {
                        if cellPresence[nr as usize][nc as usize] {
                            alive_count += 1;
                        }
                    }
                }
                if cellPresence[r][c]{
                    if alive_count < 2u32 || alive_count > 3u32 {
                        next_cell_presence[r][c] = false;
                    }
                }
                else if alive_count == 3u32 {
                    next_cell_presence[r][c] = true;
                }
            }
        }
        cellPresence = next_cell_presence.clone();
    }
}
