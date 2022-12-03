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
    str_name: String,
    number: f32,
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
            str_name:item_type.to_string(),
            number:1 as f32,
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

fn ham_bot(x : f32, y : f32) {
    let burg = Color::from_rgba(244, 164, 96, 250);
    draw_circle(x, y, 26.0, burg);
}

fn ham_bun(x : f32, y : f32) {
    ham_bot(x, y);

    pub struct Offset {
        pub xCoord : f32,
        pub yCoord : f32,
    }

    let mut vec : Vec<Offset> = Vec::new();
    vec =  vec![Offset {xCoord : -18.0, yCoord : 0.0},
                Offset {xCoord : -15.0, yCoord : -7.0},
                Offset {xCoord : 5.0, yCoord : -7.0},
                Offset {xCoord : -5.0, yCoord : -2.0},
                Offset {xCoord : -10.0, yCoord : -18.0},
                Offset {xCoord : -5.0, yCoord : -13.0},
                Offset {xCoord : 7.0, yCoord : -17.0},
                Offset {xCoord : 14.0, yCoord : -12.0},
                Offset {xCoord : 15.0, yCoord : -3.0},
                Offset {xCoord : 5.0, yCoord : 4.0},
                Offset {xCoord : 17.0, yCoord : 5.0},
                Offset {xCoord : -10.0, yCoord : 5.0},
                Offset {xCoord : -17.0, yCoord : 10.0},
                Offset {xCoord : -12.0, yCoord : 16.0},
                Offset {xCoord : -3.0, yCoord : 10.0},
                Offset {xCoord : -4.0, yCoord : 18.0},
                Offset {xCoord : 5.0, yCoord : 15.0},
                Offset {xCoord : 13.0, yCoord : 13.0}];

    let seed = Color::from_rgba(255, 235, 205, 250);

    for n in 0..18 {
        draw_circle(vec[n].xCoord + x, vec[n].yCoord + y, 1.5, seed);
    }
}

async fn leaf(x : f32, y : f32) {
    let lettuce: Texture2D = load_texture("images/lettuce.png").await.unwrap();
    let letSize = Vec2 {x: 50.0, y: 40.0};

    draw_texture_ex(
        lettuce,
        x - 25.0,
        y - 20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(letSize),
            ..Default::default()
        },
    );
}

async fn tomato(x : f32, y : f32) {
    let tomato: Texture2D = load_texture("images/tomato.png").await.unwrap();
    let tomatoSize = Vec2 {x: 50.0, y: 50.0};

    draw_texture_ex(
        tomato,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tomatoSize),
            ..Default::default()
        },
    );
}

async fn tom(x : f32, y : f32) {
    let tom: Texture2D = load_texture("images/tom.png").await.unwrap();
    let tomSize = Vec2 {x: 40.0, y: 40.0};

    draw_texture_ex(
        tom,
        x - 20.0,
        y - 20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tomSize),
            ..Default::default()
        },
    );
}

async fn fries(x : f32, y : f32) {
    let fries: Texture2D = load_texture("images/fries.png").await.unwrap();
    let frySize = Vec2 {x: 50.0, y: 50.0};

    draw_texture_ex(
        fries,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(frySize),
            ..Default::default()
        },
    );
}

async fn cashier(x : f32, y : f32) {
    let cashier: Texture2D = load_texture("images/register.png").await.unwrap();
    let cashSize = Vec2 {x: 90.0, y: 90.0};

    draw_texture_ex(
        cashier,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(cashSize),
            ..Default::default()
        },
    );
}

fn burger(x : f32, y : f32) {
    let meat = Color::from_rgba(240, 128, 128, 225);
    let cooked = Color::from_rgba(160, 82, 45, 225);
    draw_circle(x, y, 10.0, meat);
}

fn boxes() {
    let grill = Color::from_rgba(220, 220, 220, 255);
    draw_rectangle(1020.0, 30.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 105.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 180.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 255.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(945.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(870.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(795.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(720.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(620.0, 330.0, 85.0, 60.0, grill);
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
    let counter = Color::from_rgba(255, 228, 196, 255);

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
    draw_rectangle(20.0, 20.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 20.0, 400.0, 300.0, burger_floor);

    draw_text("Burgers", 335.0, 40.0, 25.0, BLACK);

    //decorations
    draw_rectangle(20.0, 20.0, 105.0, 300.0, counter);
    let grill = Color::from_rgba(220, 220, 220, 255);
    let grill2 = Color::from_rgba(169, 169, 169, 200);
    draw_rectangle(20.0, 20.0, 90.0, 193.0, grill);

    for n in (32..210).step_by(8) {
        let a = n as f32;
        draw_line(20.0, a, 110.0, a, 3.0, grill2);
    }

    draw_rectangle_lines(20.0, 20.0, 90.0, 193.0, 10.0, grill2);
    for n in (28..230).step_by(30) {
        let a = n as f32;
        draw_circle(117.0, a, 3.5, RED);
    }

    for n in (43..220).step_by(30) {
        let a = n as f32;
        draw_circle(117.0, a, 3.5, BLACK);
    }

    draw_rectangle(28.0, 230.0, 90.0, 50.0, grill);
    burger(50.0, 250.0);


    //fries
    let fries_floor = Color::from_rgba(175, 238, 238, 30);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, fries_floor);
    
    draw_text("Fries", 360.0, 323.0, 25.0, BLACK);

    //decorations
    draw_rectangle(20.0, 300.0, 105.0, 300.0, counter);
    //draw_rectangle(20.0, 490.0, 400.0, 100.0, counter);
    draw_rectangle(20.0, 305.0, 90.0, 193.0, grill);
    let oil = Color::from_rgba(255, 215, 0, 150);
    draw_rectangle(25.0, 310.0, 80.0, 180.0, oil);

    //fries(30.0, 320.0).await;

    //drinks
    let drink_floor = Color::from_rgba(255, 250, 205, 50);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, WHITE);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, drink_floor);
    
    draw_text("Drinks", 525.0, 600.0, 25.0, BLACK);
    
    //decorations
    draw_rectangle(20.0, 580.0, 105.0, 300.0, counter);
    draw_rectangle(20.0, 780.0, 600.0, 100.0, counter);

    let machine = Color::from_rgba(255, 215, 0, 170);
    //tea
    draw_rectangle(30.0, 595.0, 80.0, 80.0, machine);
    draw_text("Tea", 35.0, 610.0, 20.0, BLACK);
    //coffee
    draw_rectangle(30.0, 690.0, 80.0, 80.0, machine);
    draw_text("Coffee", 35.0, 705.0, 20.0, BLACK);
    //soda
    draw_rectangle(135.0, 790.0, 150.0, 80.0, machine);
    draw_text("Soda", 140.0, 805.0, 20.0, BLACK);
    //smoothie
    draw_rectangle(300.0, 790.0, 150.0, 80.0, machine);
    draw_text("Smoothies", 305.0, 805.0, 20.0, BLACK);

    let box_crate = Color::from_rgba(160, 82, 45, 255);
    draw_rectangle(465.0, 790.0, 120.0, 80.0, box_crate);
    /*
    draw_circle(470.0, 800.0, 15.0, WHITE);
    draw_circle(520.0, 800.0, 15.0, WHITE);
    draw_circle(580.0, 800.0, 15.0, WHITE);
    draw_circle(490.0, 840.0, 15.0, WHITE);
    draw_circle(520.0, 850.0, 15.0, WHITE);
    */
    //cashier
    let cashier_floor = Color::from_rgba(255, 127, 80, 30);
    draw_rectangle(600.0, 630.0, 485.0, 250.0, WHITE);
    draw_rectangle(600.0, 630.0, 485.0, 250.0, cashier_floor);
    
    draw_text("Cashier", 610.0, 650.0, 25.0, BLACK);

    //decorations
    draw_rectangle(600.0, 780.0, 485.0, 100.0, counter);

    //cashier(990.0, 790.0).await;
    
    //assembly 
    let assembly_floor = Color::from_rgba(152, 251, 152, 30);
    draw_rectangle(610.0, 20.0, 475.0, 510.0, WHITE);
    draw_rectangle(610.0, 20.0, 475.0, 510.0, assembly_floor);
    
    draw_text("Assembly", 615.0, 45.0, 25.0, BLACK);

    //decorations
    draw_rectangle(610.0, 400.0, 475.0, 65.0, counter);
    draw_rectangle(610.0, 325.0, 475.0, 75.0, counter);
    draw_rectangle(1010.0, 20.0, 75.0, 400.0, counter);
    boxes();

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
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Hamburger") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Hamburger"));
                    }
                    println!("Hamburger added");
                }
                ui.separator();
                if ui.button(None, "Cheeseburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Cheeseburger") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Cheeseburger"));
                    }
                    println!("Cheeseburger added");
                }
                ui.separator();
                if ui.button(None, "Double Hamburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Double Hamburger") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Double Hamburger"));
                    }
                    println!("Double Hamburger added");
                    
                }
                ui.separator();
                if ui.button(None, "Double Cheeseburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Double Cheeseburger") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Double Cheeseburger"));
                    }
                    println!("Double Cheeseburger added");
                }
                ui.separator();
                if ui.button(None, "McDouble") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "McDouble") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("McDouble"));
                    }
                    println!("McDouble added");
                }
                ui.separator();
                if ui.button(None, "Big Mac") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Big Mac") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Big Mac"));
                    }
                    println!("Big Mac added");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Quarter Pounder") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Quarter Pounder"));
                    }
                    println!("Quarter Pounder added");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder with Cheese") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Quarter Pounder with Cheese") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Quarter Pounder with Cheese"));
                    }
                    println!("Quarter Pounder with Cheese added");
                    
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Double Quarter Pounder") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Double Quarter Pounder"));
                    }
                    println!("Double Quarter Pounder added");
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder with Cheese") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Double Quarter Pounder with Cheese") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Double Quarter Pounder with Cheese"));
                    }
                    println!("Double Quarter Pounder with Cheese added");
                }
                ui.separator();
            });
            ui.tree_node(hash!(), "Fries", |ui| {
                if ui.button(None, "Small Fry") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Small Fry") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Small Fry"));
                    }
                    println!("Small Fry added");
                }
                ui.separator();
                if ui.button(None, "Medium Fry") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Medium Fry") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Medium Fry"));
                    }
                    println!("Medium Fry added");
                }
                ui.separator();
                if ui.button(None, "Large Fry") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..Order.inventory.len() {
                        if (Order.inventory[i].str_name == "Large Fry") {
                            Order.inventory[i].number += 1.0;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        Order.inventory.push(Item::new("Large Fry"));
                    }
                    println!("Large Fry added");
                }
                ui.separator();
            });
            if ui.button(None, "Place Order") {
                println!("Order Placed!");
                println!("");
                println!("Here are the details of your order: ");
                for x in &Order.inventory {
                    println!("{:?}: ({:?})", x.str_name, x.number);
                }
            }
        });


        next_frame().await
    }
}