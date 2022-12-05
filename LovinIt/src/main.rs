use macroquad::prelude::*;
use macroquad::color::Color;
use std::thread;
use std::thread::JoinHandle;
use std::time;


use macroquad::ui::{
    hash, root_ui,
    widgets::{self, Group},
    Drag, Ui,
};

#[derive(Clone)]
pub enum ItemType{
    Hamburger,
    Cheeseburger,
    DoubleHamburger,
    DoubleCheeseburger,
    McDouble,
    BigMac,
    QuarterPounder,
    QuarterPounderWithCheese,
    DoubleQuarterPounder,
    DoubleQuarterPounderWithCheese,
    SmallFry,
    MediumFry,
    LargeFry,
    Null,
}
#[derive(Clone)]
pub struct Item {
    name: ItemType,
    customizations: Vec<String>,
    str_name: String,
    number: i32,
    cooking_time: i32,
    ingredients: Vec<String>,
}

impl Item {
    pub fn new(item_type:&str) -> Item{
        Item{
            name:match item_type{
                "Hamburger"=>ItemType::Hamburger,
                "Cheeseburger"=>ItemType::Cheeseburger,
                "Double Hamburger"=>ItemType::DoubleHamburger,
                "Double Cheeseburger"=>ItemType::DoubleCheeseburger,
                "McDouble"=>ItemType::McDouble,
                "Big Mac"=>ItemType::BigMac,
                "Quarter Pounder"=>ItemType::QuarterPounder,
                "Quarter Pounder with Cheese"=>ItemType::QuarterPounderWithCheese,
                "Double Quarter Pounder"=>ItemType::DoubleQuarterPounder,
                "Double Quarter Pounder with Cheese"=>ItemType::DoubleQuarterPounderWithCheese,
                _=> ItemType::Null
            },
            customizations:vec![],
            str_name:item_type.to_string(),
            number:1,
            cooking_time:match item_type{
                "Hamburger"=>1000,
                "Double Hamburger"=>1500,
                "Double Cheeseburger"=>1500,
                "McDouble"=>900,
                "Big Mac"=>1200,
                "Quarter Pounder"=>1500,
                "Quarter Pounder with Cheese"=>1600,
                "Double Quarter Pounder"=>1700,
                "Double Quarter Pounder with Cheese"=>1900,
                _=> 500
            },
            ingredients:match item_type{
                "Hamburger"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Cheeseburger"=>vec!["bun".to_string(), "beef patty".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Double Hamburger"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Double Cheeseburger"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "cheese".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "McDouble"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Big Mac"=>vec!["big mac bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "mac sauce".to_string(), "pickles".to_string(), "lettuce".to_string()],
                "Quarter Pounder"=>vec!["bun".to_string(), "quarter beef patty".to_string(), "ketchup".to_string(), "onions slivers".to_string(), "mustard".to_string()],
                "Quarter Pounder with Cheese"=>vec!["bun".to_string(), "quarter beef patty".to_string(), "ketchup".to_string(), "onions slivers".to_string(),"cheese".to_string(), "cheese".to_string(),"mustard".to_string()],
                "Double Quarter Pounder"=>vec!["bun".to_string(), "quarter beef patty".to_string(), "quarter beef patty".to_string(),"ketchup".to_string(), "onions slivers".to_string(), "mustard".to_string()],
                "Double Quarter Pounder with Cheese"=>vec!["bun".to_string(), "quarter beef patty".to_string(), "quarter beef patty".to_string(),"ketchup".to_string(), "onions slivers".to_string(),"cheese".to_string(), "cheese".to_string(), "mustard".to_string()],
                _=> vec![]
            },
        }
    }
    pub fn cook(&self){
        for i in range(1..self.number){
            println!("{} started cooking",self.str_name);
            thread::sleep(time::Duration::from_millis(self.cooking_time as u64)); //time to cook
            println!("{} finished cooking",self.str_name);
        }
    }
}
#[derive(Clone)]
pub struct Order {
    inventory: Vec<Item>,
}
impl Order {
    pub fn new() -> Order {
        Order {
            inventory: vec![],
        }
    }
    fn inventory(&mut self, ui: &mut Ui) {
        let mut i: usize = 0;
        for (n, item) in self.inventory.clone().iter().enumerate() {
            self.inventory.retain(|x| x.number != 0);
            let mut label = (&item.str_name).to_owned();
            label.push_str("   ");
            label.push_str(&item.number.to_string());
            let drag = Group::new(hash!("inventory", n), Vec2::new(280., 50.)) //width, height
                .draggable(true)
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &label); //left padding, upper padding
                    if ui.button(Vec2::new(260., 10.), "-") {
                        self.inventory[i].number -= 1;
                    }
                });
            i += 1;
        }
    }
    fn clear(&mut self){
        self.inventory.clear();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "WcDonalds".to_owned(),
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
        pub x_coord : f32,
        pub y_coord : f32,
    }

    let mut vec : Vec<Offset> = Vec::new();
    vec =  vec![Offset {x_coord : -18.0, y_coord : 0.0},
                Offset {x_coord : -15.0, y_coord : -7.0},
                Offset {x_coord : 5.0, y_coord : -7.0},
                Offset {x_coord : -5.0, y_coord : -2.0},
                Offset {x_coord : -10.0, y_coord : -18.0},
                Offset {x_coord : -5.0, y_coord : -13.0},
                Offset {x_coord : 7.0, y_coord : -17.0},
                Offset {x_coord : 14.0, y_coord : -12.0},
                Offset {x_coord : 15.0, y_coord : -3.0},
                Offset {x_coord : 5.0, y_coord : 4.0},
                Offset {x_coord : 17.0, y_coord : 5.0},
                Offset {x_coord : -10.0, y_coord : 5.0},
                Offset {x_coord : -17.0, y_coord : 10.0},
                Offset {x_coord : -12.0, y_coord : 16.0},
                Offset {x_coord : -3.0, y_coord : 10.0},
                Offset {x_coord : -4.0, y_coord : 18.0},
                Offset {x_coord : 5.0, y_coord : 15.0},
                Offset {x_coord : 13.0, y_coord : 13.0}];

    let seed = Color::from_rgba(255, 235, 205, 250);

    for n in 0..18 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, 1.5, seed);
    }
}

async fn leaf(x : f32, y : f32, lettuce:Texture2D) {
    let let_size = Vec2 {x: 50.0, y: 40.0};

    draw_texture_ex(
        lettuce,
        x - 25.0,
        y - 20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(let_size),
            ..Default::default()
        },
    );
}

async fn tomato(x : f32, y : f32, tomato:Texture2D) {
    let tomato_size = Vec2 {x: 50.0, y: 50.0};

    draw_texture_ex(
        tomato,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tomato_size),
            ..Default::default()
        },
    );
}

async fn tom(x : f32, y : f32, tom:Texture2D) {
    let tom_size = Vec2 {x: 40.0, y: 40.0};

    draw_texture_ex(
        tom,
        x - 20.0,
        y - 20.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tom_size),
            ..Default::default()
        },
    );
}

async fn fries(x : f32, y : f32, fries:Texture2D) {
    let fry_size = Vec2 {x: 50.0, y: 50.0};

    draw_texture_ex(
        fries,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(fry_size),
            ..Default::default()
        },
    );
}

async fn cashier(x : f32, y : f32, cashier:Texture2D) {
    let cash_size = Vec2 {x: 90.0, y: 90.0};

    draw_texture_ex(
        cashier,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(cash_size),
            ..Default::default()
        },
    );
}

fn burger(x : f32, y : f32) {
    let meat = Color::from_rgba(240, 128, 128, 225);
    //let cooked = Color::from_rgba(160, 82, 45, 225);
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
    
    let lettuce_t: Texture2D = load_texture("images/lettuce.png").await.unwrap();
    let tomato_t: Texture2D = load_texture("images/tomato.png").await.unwrap();
    let tom_t: Texture2D = load_texture("images/tom.png").await.unwrap();
    let fries_t: Texture2D = load_texture("images/fries.png").await.unwrap();
    let cashier_t: Texture2D = load_texture("images/register.png").await.unwrap();
    
    
    // let worker1: Texture2D = load_texture("images/worker1.png").await.unwrap();
    // let worker2: Texture2D = load_texture("images/worker2.png").await.unwrap();
    // let worker3: Texture2D = load_texture("images/worker3.png").await.unwrap();
    // let worker4: Texture2D = load_texture("images/worker4.png").await.unwrap();
    // let worker5: Texture2D = load_texture("images/worker5.png").await.unwrap();


    
    let worker1: Texture2D = load_texture("images/nowak.png").await.unwrap();
    let worker2: Texture2D = load_texture("images/cosman.png").await.unwrap();
    let worker3: Texture2D = load_texture("images/challen.png").await.unwrap();
    let worker4: Texture2D = load_texture("images/wade.png").await.unwrap();
    let worker5: Texture2D = load_texture("images/fleck.png").await.unwrap();
    
    
    let mut order = Order::new();

    let floor_tile = Color::from_rgba(226, 222, 221, 100);
    let counter = Color::from_rgba(255, 228, 196, 255);
    let burger_floor = Color::from_rgba(138, 43, 226, 30);
    let grill = Color::from_rgba(220, 220, 220, 255);
    let grill2 = Color::from_rgba(169, 169, 169, 200);
    let oil = Color::from_rgba(255, 215, 0, 150);
    let fries_floor = Color::from_rgba(175, 238, 238, 30);
    let drink_floor = Color::from_rgba(255, 250, 205, 50);
    let machine = Color::from_rgba(255, 215, 0, 170);
    let box_crate = Color::from_rgba(160, 82, 45, 255);
    let cashier_floor = Color::from_rgba(255, 127, 80, 30);
    let assembly_floor = Color::from_rgba(152, 251, 152, 30);

loop {
        
    clear_background(WHITE);
    
    draw_rectangle_lines(10.0, 10.0, 1085.0, 880.0, 20.0, BLACK);

    

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
    
    draw_rectangle(20.0, 20.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 20.0, 400.0, 300.0, burger_floor);

    draw_text("Grill", 360.0, 40.0, 25.0, BLACK);

    //decorations
    draw_rectangle(20.0, 20.0, 105.0, 300.0, counter);

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
    draw_rectangle(20.0, 300.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, fries_floor);
    
    draw_text("Frying", 350.0, 323.0, 25.0, BLACK);

    //decorations
    draw_rectangle(20.0, 300.0, 105.0, 300.0, counter);
    //draw_rectangle(20.0, 490.0, 400.0, 100.0, counter);
    draw_rectangle(20.0, 305.0, 90.0, 193.0, grill);
    draw_rectangle(25.0, 310.0, 80.0, 180.0, oil);

    //fries(30.0, 320.0).await;

    //drinks
    draw_rectangle(20.0, 580.0, 580.0, 300.0, WHITE);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, drink_floor);
    
    draw_text("Drinks", 525.0, 600.0, 25.0, BLACK);
    
    //decorations
    draw_rectangle(20.0, 580.0, 105.0, 300.0, counter);
    draw_rectangle(20.0, 780.0, 600.0, 100.0, counter);

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

    draw_rectangle(465.0, 790.0, 120.0, 80.0, box_crate);
    
    //cashier
    draw_rectangle(600.0, 630.0, 485.0, 250.0, WHITE);
    draw_rectangle(600.0, 630.0, 485.0, 250.0, cashier_floor);
    
    draw_text("Cashier", 610.0, 650.0, 25.0, BLACK);

    //decorations
    draw_rectangle(600.0, 780.0, 485.0, 100.0, counter);

    cashier(990.0, 790.0, cashier_t).await;
    
    //assembly 
    draw_rectangle(610.0, 20.0, 475.0, 510.0, WHITE);
    draw_rectangle(610.0, 20.0, 475.0, 510.0, assembly_floor);
    
    draw_text("Assembly", 615.0, 45.0, 25.0, BLACK);

    //decorations
    draw_rectangle(610.0, 400.0, 475.0, 65.0, counter);
    draw_rectangle(610.0, 325.0, 475.0, 75.0, counter);
    draw_rectangle(1010.0, 20.0, 75.0, 400.0, counter);
    boxes();
    leaf(700.0, 300.0, lettuce_t).await;
    ham_bot(800.0, 800.0);
    ham_bun(900.0, 300.0);
    tomato(600.0, 50.0, tomato_t).await;
    tom(900.0, 700.0, tom_t).await;
    fries(500.0, 300.0, fries_t).await;

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
            50.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker2,
            150.0,
            325.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker3,
            150.0,
            575.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker4,
            800.0,
            75.0,
            WHITE,
            DrawTextureParams {
                dest_size: None,
                ..Default::default()
            },
        );
        draw_texture_ex(
            worker5,
            800.0,
            575.0,
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
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Hamburger" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Hamburger"));
                    }
                    println!("Hamburger added");
                }
                ui.separator();
                if ui.button(None, "Cheeseburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Cheeseburger" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Cheeseburger"));
                    }
                    println!("Cheeseburger added");
                }
                ui.separator();
                if ui.button(None, "Double Hamburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Double Hamburger" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Double Hamburger"));
                    }
                    println!("Double Hamburger added");
                    
                }
                ui.separator();
                if ui.button(None, "Double Cheeseburger") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Double Cheeseburger" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Double Cheeseburger"));
                    }
                    println!("Double Cheeseburger added");
                }
                ui.separator();
                if ui.button(None, "McDouble") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "McDouble" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("McDouble"));
                    }
                    println!("McDouble added");
                }
                ui.separator();
                if ui.button(None, "Big Mac") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Big Mac" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Big Mac"));
                    }
                    println!("Big Mac added");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Quarter Pounder" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Quarter Pounder"));
                    }
                    println!("Quarter Pounder added");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder with Cheese") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Quarter Pounder with Cheese" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Quarter Pounder with Cheese"));
                    }
                    println!("Quarter Pounder with Cheese added");
                    
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Double Quarter Pounder" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Double Quarter Pounder"));
                    }
                    println!("Double Quarter Pounder added");
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder with Cheese") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Double Quarter Pounder with Cheese" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Double Quarter Pounder with Cheese"));
                    }
                    println!("Double Quarter Pounder with Cheese added");
                }
                ui.separator();
            });
            ui.tree_node(hash!(), "Sides", |ui| {
                if ui.button(None, "Small Fry") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Small Fry" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Small Fry"));
                    }
                    println!("Small Fry added");
                }
                ui.separator();
                if ui.button(None, "Medium Fry") {
                   let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Medium Fry" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Medium Fry"));
                    }
                    println!("Medium Fry added");
                }
                ui.separator();
                if ui.button(None, "Large Fry") {
                    let mut duplicate_item: bool = false; 
                    for i in 0..order.inventory.len() {
                        if order.inventory[i].str_name == "Large Fry" {
                            order.inventory[i].number += 1;
                            duplicate_item = true;
                            break;
                        }
                    }
                    if !duplicate_item {
                        order.inventory.push(Item::new("Large Fry"));
                    }
                    println!("Large Fry added");
                }
                ui.separator();
            });
        });

    widgets::Window::new(hash!(), vec2(1110., 450.), vec2(300., 400.))
    .label("Order")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(290., 380.)).ui(ui, |ui| {
            order.inventory(ui);
            if ui.button(Vec2::new(102., order.inventory.len() as f32 * 52.0 + 10 as f32), "Place Order") {
                println!("Order Placed!");
                println!("");
                println!("Here are the details of your order: ");
                let mut handles = Vec::new();
                let placed_order = order.inventory.clone();
                for x in placed_order {
                    let y = x.clone();
                    let item_cooking = thread::spawn(move|| {
                        y.cook();
                    });
                    handles.push(item_cooking);
                    println!("{}: ({:?})", x.str_name, x.number);
                }
                order.clear();
            }
        });
    });

        next_frame().await
    }
}