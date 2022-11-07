use macroquad::prelude::*;

#[macroquad::main("Texture")]
async fn main() {
    //adding image into program
    let texture: Texture2D = load_texture("images/nowak1.png").await.unwrap();

    loop {
        //background
        clear_background(WHITE);

        //drawing the image
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );

        //mouse
        let mouse = mouse_position();
        draw_circle(mouse.0, mouse.1, 25.0, RED);


        
        next_frame().await
    }
}