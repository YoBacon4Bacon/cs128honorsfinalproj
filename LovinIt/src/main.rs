use macroquad::prelude::*;
use macroquad::color::Color;

<<<<<<< HEAD
use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};
pub enum ItemType{
    Hamburger,
    Cheeseburger,
    Double_Hamburger,
    Double_Cheeseburger,
    McDouble,
    Big_Mac,
    Quarter_Pounder,
    Quarter_Pounder_with_Cheese,
    Double_Quarter_Pounder,
    Double_Quarter_Pounder_with_Cheese,
    Small_Fry,
    Medium_Fry,
    Large_Fry,
    Null,
}
pub struct Item {
    name: ItemType,
    customizations: Vec<String>,
}
impl Item {
    pub fn new(item_type:&str) -> Item{
        Item{
            name:match item_type{
                "Hamburger"=>ItemType::Hamburger,
                "Cheeseburger"=>ItemType::Cheeseburger,
                "Double Hamburger"=>ItemType::Double_Hamburger,
                "Double Cheeseburger"=>ItemType::Double_Cheeseburger,
                "McDouble"=>ItemType::McDouble,
                "Big Mac"=>ItemType::Big_Mac,
                "Quarter Pounder"=>ItemType::Quarter_Pounder,
                "Quarter Pounder with Cheese"=>ItemType::Quarter_Pounder_with_Cheese,
                "Double Quarter Pounder"=>ItemType::Double_Quarter_Pounder,
                "Double Quarter Pounder with Cheese"=>ItemType::Double_Quarter_Pounder_with_Cheese,
                _=> ItemType::Null
            },
            customizations:vec![],
        }
    }
}
pub struct Order {
    inventory: Vec<Item>,
}
impl Order {
    pub fn new() -> Order {
        Order {
            inventory: vec![],
        }
    }
}
#[macroquad::main("lovin_it")]
async fn main() {
    //adding image into program
    let worker1: Texture2D = load_texture("images/worker1.png").await.unwrap();
    let worker2: Texture2D = load_texture("images/worker2.png").await.unwrap();
    let worker3: Texture2D = load_texture("images/worker3.png").await.unwrap();
    let worker4: Texture2D = load_texture("images/worker4.png").await.unwrap();
    let worker5: Texture2D = load_texture("images/worker5.png").await.unwrap();
    let worker6: Texture2D = load_texture("images/worker6.png").await.unwrap();

    let mut Order = Order::new();

    loop {
        clear_background(WHITE);
        //drawing the image
        //texture methods for image manipulation
        //https://github.com/not-fl3/macroquad/blob/master/src/texture.rs
        draw_texture_ex(
            worker1,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker2,
            250.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker3,
            500.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker4,
            0.0,
            250.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker5,
            250.0,
            250.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker6,
            500.0,
            250.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        
=======
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
>>>>>>> 0f6947ea0e3b3e781b401bb21fee1b2f1d05131c

            //right vertical (top), vertical (bottom), horizontal (up), horizontal (bottom)
            draw_line(750.0, 50.0, 750.0, 250.0, 50.0, LIGHTGRAY);
            draw_line(750.0, 350.0, 750.0, 550.0, 50.0, LIGHTGRAY);
            draw_line(470.0, 70.0, 775.0, 70.0, 50.0, LIGHTGRAY);
            draw_line(470.0, 550.0, 775.0, 550.0, 50.0, LIGHTGRAY);

<<<<<<< HEAD
    widgets::Window::new(hash!(), vec2(0., 0.), vec2(300., 300.))
        .label("Menu")
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Click to add to cart.");
            ui.tree_node(hash!(), "Burgers", |ui| {
                if ui.button(None, "Hamburger") {
                    println!("Hamburger added");
                    Order.inventory.push(Item::new("Hamburger"));
                }
                ui.separator();
                if ui.button(None, "Cheeseburger") {
                    println!("Cheeseburger added");
                    Order.inventory.push(Item::new("Cheeseburger"));
                }
                ui.separator();
                if ui.button(None, "Double Hamburger") {
                    println!("Double Hamburger added");
                    Order.inventory.push(Item::new("Double Hamburger"));
                }
                ui.separator();
                if ui.button(None, "Double Cheeseburger") {
                    println!("Double Cheeseburger added");
                    Order.inventory.push(Item::new("Double Cheeseburger"));
                }
                ui.separator();
                if ui.button(None, "McDouble") {
                    println!("McDouble added");
                    Order.inventory.push(Item::new("McDouble"));
                }
                ui.separator();
                if ui.button(None, "Big Mac") {
                    println!("Big Mac added");
                    Order.inventory.push(Item::new("Big Mac"));
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder") {
                    println!("Quarter Pounder added");
                    Order.inventory.push(Item::new("Quarter Pounder"));
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder with Cheese") {
                    println!("Quarter Pounder with Cheese added");
                    Order.inventory.push(Item::new("Quarter Pounder with Cheese"));
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder") {
                    println!("Double Quarter Pounder added");
                    Order.inventory.push(Item::new("Double Quarter Pounder"));
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder with Cheese") {
                    println!("Double Quarter Pounder with Cheese added");
                    Order.inventory.push(Item::new("Double Quarter Pounder with Cheese"));
                }
                ui.separator();
            });
            ui.tree_node(hash!(), "Fries", |ui| {
                if ui.button(None, "Small Fry") {
                    println!("Small Fry added");
                    Order.inventory.push(Item::new("Small Fry"));
                }
                ui.separator();
                if ui.button(None, "Medium Fry") {
                    println!("Medium Fry added");
                    Order.inventory.push(Item::new("Medium Fry"));
                }
                ui.separator();
                if ui.button(None, "Large Fry") {
                    println!("Large Fry added");
                    Order.inventory.push(Item::new("Large Fry"));
                }
                ui.separator();
            });
        });
    
    next_frame().await;
=======
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
>>>>>>> 0f6947ea0e3b3e781b401bb21fee1b2f1d05131c
    }
}