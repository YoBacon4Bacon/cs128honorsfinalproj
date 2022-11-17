use macroquad::prelude::*;
use macroquad::color::Color;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(WHITE);

        //weird L shaped stations
        /*
        while {
            /*
            //left vertical (top), vertical (bottom), horizontal (up), horizontal (bottom)
            draw_line(50.0, 50.0, 50.0, 250.0, 50.0, LIGHTGRAY);
            draw_line(50.0, 350.0, 50.0, 550.0, 50.0, LIGHTGRAY);
            draw_line(25.0, 70.0, 330.0, 70.0, 50.0, LIGHTGRAY);
            draw_line(25.0, 550.0, 330.0, 550.0, 50.0, LIGHTGRAY);

            //right vertical (top), vertical (bottom), horizontal (up), horizontal (bottom)
            draw_line(750.0, 50.0, 750.0, 250.0, 50.0, LIGHTGRAY);
            draw_line(750.0, 350.0, 750.0, 550.0, 50.0, LIGHTGRAY);
            draw_line(470.0, 70.0, 775.0, 70.0, 50.0, LIGHTGRAY);
            draw_line(470.0, 550.0, 775.0, 550.0, 50.0, LIGHTGRAY);

            //text
            draw_text("Burger", 40.0, 40.0, 20.0, BLACK);
            draw_text("Fries", 40.0, 590.0, 20.0, BLACK);
            draw_text("Drink", 700.0, 40.0, 20.0, BLACK);
            draw_text("Cashier", 680.0, 590.0, 20.0, BLACK);
            */
        }
        */

        draw_rectangle_lines(10.0, 10.0, 775.0, 575.0, 20.0, BLACK);

        let c = Color::from_rgba(226, 222, 221, 100);

        for n in (33..770).step_by(17) {
            let a = n as f32;
            draw_line(a, 20.0, a, 575.0, 3.0, c);
        }

        for n in (33..570).step_by(17) {
            let a = n as f32;
            draw_line(20.0, a, 775.0, a, 3.0, c);
        }

        //
        //burger
        draw_rectangle(20.0, 20.0, 300.0, 200.0, LIGHTGRAY);
        draw_text("Burgers", 40.0, 50.0, 50.0, BLACK);

        //fries
        draw_rectangle(20.0, 220.0, 300.0, 200.0, BEIGE);
        draw_text("Fries", 40.0, 255.0, 50.0, BLACK);

        //drinks
        draw_rectangle(20.0, 375.0, 300.0, 200.0, LIGHTGRAY);
        draw_text("Drinks", 40.0, 550.0, 50.0, BLACK);

        //cashier
        draw_rectangle(525.0, 375.0, 250.0, 200.0, GOLD);
        draw_text("Cashier", 600.0, 550.0, 50.0, BLACK);

        //assembly 
        draw_rectangle(475.0, 20.0, 300.0, 250.0, GREEN);
        draw_text("Assembly", 575.0, 50.0, 50.0, BLACK);

        next_frame().await
    }
}