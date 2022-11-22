use macroquad::prelude::*;
use macroquad::color::Color;

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
pub struct Hamburger {
    ingredients: Vec<String>,
}
impl Default for Hamburger {
    fn default() -> Hamburger {
        Hamburger {
            ingredients: vec!["reg bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
        }
    }
}
pub struct Cheeseburger {
    ingredients: Vec<String>,
}
impl Default for Cheeseburger {
    fn default() -> Cheeseburger {
        Cheeseburger {
            ingredients: vec!["reg bun".to_string(), "beef patty".to_string(), "amer cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
        }
    }
}
pub struct McDouble {
    ingredients: Vec<String>,
}
impl Default for McDouble {
    fn default() -> McDouble {
        McDouble {
            ingredients: vec!["reg bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "amer cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
        }
    }
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Window name".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

//#[macroquad::main("lovin_it")]
#[macroquad::main(window_conf)]
async fn main() {
    //adding image into program
    
    let worker1: Texture2D = load_texture("images/worker1.png").await.unwrap();
    let worker2: Texture2D = load_texture("images/worker2.png").await.unwrap();
    let worker3: Texture2D = load_texture("images/worker3.png").await.unwrap();
    let worker4: Texture2D = load_texture("images/worker4.png").await.unwrap();
    let worker5: Texture2D = load_texture("images/worker5.png").await.unwrap();
    
    let mut Order = Order::new();

loop {
        
    clear_background(WHITE);

    draw_rectangle_lines(10.0, 10.0, 1085.0, 880.0, 20.0, BLACK);

    let floor_tile = Color::from_rgba(226, 222, 221, 100);

    for n in (33..1080).step_by(17) {
        let a = n as f32;
        draw_line(a, 20.0, a, 880.0, 3.0, floor_tile);
    }
    
    for n in (33..870).step_by(17) {
        let a = n as f32;
        draw_line(20.0, a, 1085.0, a, 3.0, floor_tile);
    }

    //far right, x = 1085
    
    //burger
    let burger_floor = Color::from_rgba(138, 43, 226, 30);
    //let grill = Color::from_rgba(220, 220, 220, 255);
    //let grill2 = Color::from_rgba(169, 169, 169, 70);
    draw_rectangle(20.0, 20.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 20.0, 400.0, 300.0, burger_floor);
    //draw_rectangle(20.0, 20.0, 70.0, 200.0, grill);
    //draw_rectangle(85.0, 20.0, 5.0, 200.0, grill2);
    //draw_text("Burgers", 255.0, 35.0, 20.0, BLACK);
    
    //fries
    let fries_floor = Color::from_rgba(175, 238, 238, 30);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, fries_floor);
    //draw_text("Fries", 40.0, 230.0, 50.0, BLACK);

    //drinks
    let drink_floor = Color::from_rgba(255, 127, 80, 30);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, WHITE);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, drink_floor);
    //draw_text("Drinks", 40.0, 550.0, 50.0, BLACK);
    
    //cashier
    let cashier_floor = Color::from_rgba(255, 250, 205, 50);
    draw_rectangle(600.0, 630.0, 485.0, 248.0, WHITE);
    draw_rectangle(600.0, 630.0, 485.0, 248.0, cashier_floor);
    //draw_text("Cashier", 620.0, 410.0, 50.0, BLACK);
    
    //assembly 
    let assembly_floor = Color::from_rgba(152, 251, 152, 30);
    draw_rectangle(610.0, 20.0, 475.0, 510.0, WHITE);
    draw_rectangle(610.0, 20.0, 475.0, 510.0, assembly_floor);
    //draw_text("Assembly", 575.0, 50.0, 50.0, BLACK);
    
    //partitions/walls
    draw_line(20.0, 300.0, 420.0, 300.0, 10.0, BLACK);
    draw_line(20.0, 580.0, 420.0, 580.0, 10.0, BLACK);
    draw_line(610.0, 400.0, 1085.0, 400.0, 10.0, BLACK);
    draw_line(600.0, 575.0, 600.0, 880.0, 10.0, BLACK);

    //drawing the image
    //texture methods for image manipulation
    
    //https://github.com/not-fl3/macroquad/blob/master/src/texture.rs
        draw_texture_ex(
            worker1,
            150.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker2,
            150.0,
            400.0,
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
            150.0,
            200.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker5,
            600.0,
            400.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );

       
    widgets::Window::new(hash!(), vec2(1110., 25.), vec2(300., 400.))
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
            if ui.button(None, "Place Order") {
                println!("Order Placed!");
            }
        });


        next_frame().await
    }
}