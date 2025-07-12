

mod framebuffer; // Declara el módulo framebuffer
use framebuffer::Framebuffer; // Importa tu estructura Framebuffer personalizada


use raylib::prelude::*; // Importa tipos comunes de raylib como Color, Vector2, Rectangle
use raylib::callbacks::TraceLogLevel; // Importa TraceLogLevel para la configuración de registro
use std::thread; // Importa el módulo thread para sleep
use std::time::Duration; // Importa Duration para thread::sleep

// --- Funciones para definir patrones iniciales ---
fn add_glider(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 1 + offset_y));
    pattern.push((0 + offset_x, 2 + offset_y));
    pattern.push((1 + offset_x, 2 + offset_y));
    pattern.push((2 + offset_x, 2 + offset_y));
}

fn add_lightweight_spaceship(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((4 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((0 + offset_x, 2 + offset_y));
    pattern.push((4 + offset_x, 2 + offset_y));
    pattern.push((0 + offset_x, 3 + offset_y));
    pattern.push((1 + offset_x, 3 + offset_y));
    pattern.push((2 + offset_x, 3 + offset_y));
    pattern.push((3 + offset_x, 3 + offset_y));
}

fn add_blinker(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((0 + offset_x, 0 + offset_y));
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 0 + offset_y));
}

fn add_toad(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 0 + offset_y));
    pattern.push((3 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 1 + offset_y));
    pattern.push((2 + offset_x, 1 + offset_y));
}

fn add_beacon(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((0 + offset_x, 0 + offset_y));
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((3 + offset_x, 2 + offset_y));
    pattern.push((2 + offset_x, 3 + offset_y));
    pattern.push((3 + offset_x, 3 + offset_y));
}

fn add_pulsar(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    let base_x = offset_x;
    let base_y = offset_y;

    // Outer ring
    pattern.push((base_x + 2, base_y));
    pattern.push((base_x + 3, base_y));
    pattern.push((base_x + 4, base_y));

    pattern.push((base_x, base_y + 2));
    pattern.push((base_x, base_y + 3));
    pattern.push((base_x, base_y + 4));

    pattern.push((base_x + 5, base_y + 2));
    pattern.push((base_x + 5, base_y + 3));
    pattern.push((base_x + 5, base_y + 4));

    pattern.push((base_x + 2, base_y + 5));
    pattern.push((base_x + 3, base_y + 5));
    pattern.push((base_x + 4, base_y + 5));

    // Inner ring (offset by 6 from outer ring's base)
    pattern.push((base_x + 8, base_y));
    pattern.push((base_x + 9, base_y));
    pattern.push((base_x + 10, base_y));

    pattern.push((base_x + 7, base_y + 2));
    pattern.push((base_x + 7, base_y + 3));
    pattern.push((base_x + 7, base_y + 4));

    pattern.push((base_x + 12, base_y + 2));
    pattern.push((base_x + 12, base_y + 3));
    pattern.push((base_x + 12, base_y + 4));

    pattern.push((base_x + 8, base_y + 5));
    pattern.push((base_x + 9, base_y + 5));
    pattern.push((base_x + 10, base_y + 5));

    // Mirror vertically (offset by 6 from original y)
    pattern.push((base_x + 2, base_y + 7));
    pattern.push((base_x + 3, base_y + 7));
    pattern.push((base_x + 4, base_y + 7));

    pattern.push((base_x, base_y + 8));
    pattern.push((base_x, base_y + 9));
    pattern.push((base_x, base_y + 10));

    pattern.push((base_x + 5, base_y + 8));
    pattern.push((base_x + 5, base_y + 9));
    pattern.push((base_x + 5, base_y + 10));

    pattern.push((base_x + 2, base_y + 12));
    pattern.push((base_x + 3, base_y + 12));
    pattern.push((base_x + 4, base_y + 12));

    // Mirror vertically and horizontally
    pattern.push((base_x + 8, base_y + 7));
    pattern.push((base_x + 9, base_y + 7));
    pattern.push((base_x + 10, base_y + 7));

    pattern.push((base_x + 7, base_y + 8));
    pattern.push((base_x + 7, base_y + 9));
    pattern.push((base_x + 7, base_y + 10));

    pattern.push((base_x + 12, base_y + 8));
    pattern.push((base_x + 12, base_y + 9));
    pattern.push((base_x + 12, base_y + 10));

    pattern.push((base_x + 8, base_y + 12));
    pattern.push((base_x + 9, base_y + 12));
    pattern.push((base_x + 10, base_y + 12));
}

fn add_pentadecathlon(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    // Central line
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 0 + offset_y));
    pattern.push((3 + offset_x, 0 + offset_y));
    pattern.push((4 + offset_x, 0 + offset_y));
    pattern.push((5 + offset_x, 0 + offset_y));
    pattern.push((6 + offset_x, 0 + offset_y));
    pattern.push((7 + offset_x, 0 + offset_y));
    pattern.push((8 + offset_x, 0 + offset_y));
    pattern.push((9 + offset_x, 0 + offset_y));
    pattern.push((10 + offset_x, 0 + offset_y));

    // Side cells
    pattern.push((0 + offset_x, 1 + offset_y));
    // Ensure offset_y > 0 for this to not go out of bounds if y is 0
    if offset_y > 0 {
        pattern.push((0 + offset_x, offset_y - 1));
    }
    pattern.push((5 + offset_x, 1 + offset_y));
    if offset_y > 0 {
        pattern.push((5 + offset_x, offset_y - 1));
    }
    pattern.push((11 + offset_x, 1 + offset_y));
    if offset_y > 0 {
        pattern.push((11 + offset_x, offset_y - 1));
    }
}

// Block
fn add_block(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((0 + offset_x, 0 + offset_y));
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 1 + offset_y));
}

// Tub
fn add_tub(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((2 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 2 + offset_y));
}

// Boat
fn add_boat(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((0 + offset_x, 0 + offset_y));
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((2 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 2 + offset_y));
}

// Loaf
fn add_loaf(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((3 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 2 + offset_y));
    pattern.push((3 + offset_x, 2 + offset_y));
    pattern.push((2 + offset_x, 3 + offset_y));
}

// Still life: beehive
fn add_beehive(pattern: &mut Vec<(u32, u32)>, offset_x: u32, offset_y: u32) {
    pattern.push((1 + offset_x, 0 + offset_y));
    pattern.push((2 + offset_x, 0 + offset_y));
    pattern.push((0 + offset_x, 1 + offset_y));
    pattern.push((3 + offset_x, 1 + offset_y));
    pattern.push((1 + offset_x, 2 + offset_y));
    pattern.push((2 + offset_x, 2 + offset_y));
}

// --- Función render para la lógica del Juego de la Vida ---
fn render(framebuffer: &mut Framebuffer, grid: &mut Vec<Vec<bool>>) {
    let width = framebuffer.width;
    let height = framebuffer.height;

    // Crea una nueva cuadrícula para almacenar la próxima generación
    let mut next_grid = vec![vec![false; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let mut live_neighbors = 0;

            // Itera sobre los 8 vecinos (incluyendo el propio píxel, que se excluirá)
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue; // No contar la propia célula
                    }

                    // Calcula las coordenadas del vecino con envoltura (wrap-around)
                    let nx = (x as i32 + dx + width as i32) % width as i32;
                    let ny = (y as i32 + dy + height as i32) % height as i32;

                    // Asegúrate de que las coordenadas sean positivas (Rust % operator can return negative for negative inputs)
                    let nx = (nx + width as i32) % width as i32;
                    let ny = (ny + height as i32) % height as i32;


                    if grid[ny as usize][nx as usize] {
                        live_neighbors += 1;
                    }
                }
            }

            // Aplica las reglas del Juego de la Vida
            if grid[y as usize][x as usize] { // Célula viva
                if live_neighbors < 2 || live_neighbors > 3 {
                    next_grid[y as usize][x as usize] = false; // Muere por subpoblación o sobrepoblación
                } else {
                    next_grid[y as usize][x as usize] = true; // Sobrevive
                }
            } else { // Célula muerta
                if live_neighbors == 3 {
                    next_grid[y as usize][x as usize] = true; // Reproducción
                } else {
                    next_grid[y as usize][x as usize] = false; // Permanece muerta
                }
            }
        }
    }

    // Actualiza la cuadrícula principal con la próxima generación
    *grid = next_grid;

    // Dibuja la cuadrícula actualizada en el framebuffer
    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] {
                framebuffer.point(x, y, Color::WHITE); // Célula viva
            } else {
                framebuffer.point(x, y, Color::BLACK); // Célula muerta
            }
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    // La resolución del framebuffer es más pequeña para el Juego de la Vida
    let framebuffer_game_width = 100;
    let framebuffer_game_height = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height) // Establece las dimensiones de la ventana
        .title("Juego de la Vida de Conway") // Establece el título de la ventana
        .log_level(TraceLogLevel::LOG_WARNING) // Establece el nivel de registro
        .build(); // Construye la ventana

    // Crea una instancia de tu Framebuffer personalizado con la resolución del juego
    let mut framebuffer = Framebuffer::new(framebuffer_game_width, framebuffer_game_height);

    // Inicializa la cuadrícula del Juego de la Vida
    let mut game_grid = vec![vec![false; framebuffer_game_width as usize]; framebuffer_game_height as usize];

    // --- Definir el patrón inicial ---
    let mut initial_pattern_coords: Vec<(u32, u32)> = Vec::new();

    // Gliders
    add_glider(&mut initial_pattern_coords, 5, 5);
    add_glider(&mut initial_pattern_coords, 15, 10);
    add_glider(&mut initial_pattern_coords, 25, 15);
    add_glider(&mut initial_pattern_coords, 70, 50);

    // Lightweight Spaceships
    add_lightweight_spaceship(&mut initial_pattern_coords, 5, 30);
    add_lightweight_spaceship(&mut initial_pattern_coords, 20, 40);

    // Oscillators
    add_blinker(&mut initial_pattern_coords, 50, 5);
    add_blinker(&mut initial_pattern_coords, 50, 10);
    add_toad(&mut initial_pattern_coords, 60, 20);
    add_beacon(&mut initial_pattern_coords, 70, 30);

    // Still Lifes
    add_block(&mut initial_pattern_coords, 80, 5);
    add_tub(&mut initial_pattern_coords, 85, 15);
    add_boat(&mut initial_pattern_coords, 90, 25);
    add_loaf(&mut initial_pattern_coords, 75, 35);
    add_beehive(&mut initial_pattern_coords, 80, 45);

    // Complex oscillator (Pulsar) - requires more space
    add_pulsar(&mut initial_pattern_coords, 10, 60);

    // Pentadecathlon - requires more space
    add_pentadecathlon(&mut initial_pattern_coords, 40, 70);


    // Establece las células iniciales en la cuadrícula
    for &(x, y) in &initial_pattern_coords {
        if x < framebuffer_game_width && y < framebuffer_game_height {
            game_grid[y as usize][x as usize] = true;
        }
    }

    // Bucle principal del juego: continúa mientras la ventana no deba cerrarse
    while !window.window_should_close() {
        // 1. Dibuja y actualiza el estado del juego
        // La función render ahora maneja la lógica del Juego de la Vida y el dibujo.
        render(&mut framebuffer, &mut game_grid);

        // 2. Intercambia los buffers (muestra el framebuffer en la ventana)
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Introduce un pequeño retraso para controlar la velocidad de la animación
        thread::sleep(Duration::from_millis(100)); // Un delay más largo para visualizar mejor
    }
}
