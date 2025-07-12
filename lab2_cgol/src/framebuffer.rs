use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color, // Color para células muertas
    pub current_color: Color,    // Color para células vivas
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        // Inicializa el buffer de color con una imagen negra
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: Color::BLACK, // Por defecto, las células muertas son negras
            current_color: Color::WHITE,    // Por defecto, las células vivas son blancas
        }
    }

    pub fn clear(&mut self) {
        // Rellena todo el buffer de color con el color de fondo actual
        for y in 0..self.height {
            for x in 0..self.width {
                Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.background_color);
            }
        }
    }

    // Renombrada set_pixel a point según la solicitud
    pub fn point(&mut self, x: u32, y: u32, color: Color) {
        // Dibuja un solo píxel en (x, y) con el color especificado, si está dentro de los límites
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, color);
        }
    }

    // Nueva función para obtener el color de un píxel
    // Corregido: Ahora toma &mut self porque Image::get_color requiere mutabilidad
    pub fn get_pixel_color(&mut self, x: u32, y: u32) -> Color {
        // Si las coordenadas están fuera de los límites, asume que la célula está "muerta" (negra)
        if x < self.width && y < self.height {
            self.color_buffer.get_color(x as i32, y as i32)
        } else {
            Color::BLACK // Las células fuera de los límites se consideran muertas
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        // Establece el color de fondo para futuras operaciones de limpieza
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        // Establece el color para futuras operaciones de dibujo (como point)
        self.current_color = color;
    }

    pub fn render_to_file(&self, filename: &str) {
        // Exporta el buffer de color actual a un archivo de imagen
        let img = self.color_buffer.clone();
        img.export_image(filename);
    }

    pub fn swap_buffers(
        &self,
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
    ) {
        // Carga el buffer de color actual en una textura de Raylib
        if let Ok(texture) = window.load_texture_from_image(raylib_thread, &self.color_buffer) {
            // Obtener las dimensiones de la pantalla antes de comenzar a dibujar
            // Esto resuelve el error de préstamo mutable/inmutable
            let screen_width = window.get_screen_width() as f32;
            let screen_height = window.get_screen_height() as f32;

            // Comienza las operaciones de dibujo en la ventana principal
            let mut renderer = window.begin_drawing(raylib_thread);
            // Limpia el fondo de la ventana principal antes de dibujar el nuevo fotograma
            renderer.clear_background(Color::RAYWHITE); // O cualquier otro color de fondo que prefieras para la ventana

            // Calcula la escala para ajustar el framebuffer a la ventana
            let scale_x = screen_width / self.width as f32;
            let scale_y = screen_height / self.height as f32;
            let scale = f32::min(scale_x, scale_y);

            let scaled_width = self.width as f32 * scale;
            let scaled_height = self.height as f32 * scale;

            // Centra el framebuffer en la ventana
            let offset_x = (screen_width - scaled_width) / 2.0;
            let offset_y = (screen_height - scaled_height) / 2.0;

            // Dibuja la textura (que contiene el contenido de nuestro framebuffer) en la ventana
            renderer.draw_texture_pro(
                &texture,
                Rectangle::new(0.0, 0.0, self.width as f32, self.height as f32), // Rectángulo de origen (todo el framebuffer)
                Rectangle::new(offset_x, offset_y, scaled_width, scaled_height), // Rectángulo de destino
                Vector2::new(0.0, 0.0), // Origen
                0.0, // Rotación
                Color::WHITE, // Tinte
            );
        }
    }
}
