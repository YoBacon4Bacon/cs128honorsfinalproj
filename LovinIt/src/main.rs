use macroquad::prelude::*;
use macroquad::color::Color;
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::sync::mpsc;
use std::time::Instant;
use std::time::Duration;


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
    McNuggets4,
    McNuggets6,
    McNuggets10,
    McNuggets20,
    RegularCoffee,
    RegularTea,
    RegularSmoothie,
    RegularSoda,
    Null,
}
#[derive(Clone)]
pub struct Item {
    name: ItemType,
    customizations: Vec<String>,
    str_name: String,
    number: i32,
    cooking_time: i32,
    assembly_time: i32,
    ingredients: Vec<String>,
    starting_station: String,
    order_num: i32,
    status: String,
}

impl Item {
    pub fn new(item_type:&str, order_number:i32) -> Item{
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
                "4 McNuggets"=>ItemType::McNuggets4,
                "6 McNuggets"=>ItemType::McNuggets6,
                "10 McNuggets"=>ItemType::McNuggets10,
                "20 McNuggets"=>ItemType::McNuggets20,
                "Regular Coffee"=>ItemType::RegularCoffee,
                "Regular Tea"=>ItemType::RegularTea,
                "Regular Smoothie"=>ItemType::RegularSmoothie,
                "Regular Soda"=>ItemType::RegularSoda,
                _=> ItemType::Null
            },
            customizations:vec![],
            str_name:item_type.to_string(),
            number:1,
            cooking_time:match item_type{
                "Hamburger"=>1700,
                "Cheeseburger"=>1700,
                "Double Hamburger"=>2500,
                "Double Cheeseburger"=>2500,
                "McDouble"=>2500,
                "Big Mac"=>2500,
                "Quarter Pounder"=>2000,
                "Quarter Pounder with Cheese"=>2000,
                "Double Quarter Pounder"=>3000,
                "Double Quarter Pounder with Cheese"=>3000,
                "Small Fry"=>2000,
                "Medium Fry"=>2200,
                "Large Fry"=>2400,
                "4 McNuggets"=>2400,
                "6 McNuggets"=>2600,
                "10 McNuggets"=>2800,
                "20 McNuggets"=>3000,
                "Regular Coffee"=>1000,
                "Regular Tea"=>1000,
                "Regular Soda"=>900,
                "Regular Smoothie"=>2000,
                _=> 500
            },
            assembly_time:match item_type{
                "Hamburger"=>1000,
                "Cheeseburger"=>1200,
                "Double Hamburger"=>1200,
                "Double Cheeseburger"=>1400,
                "McDouble"=>1200,
                "Big Mac"=>1800,
                "Quarter Pounder"=>1500,
                "Quarter Pounder with Cheese"=>1600,
                "Double Quarter Pounder"=>1700,
                "Double Quarter Pounder with Cheese"=>1800,
                "Small Fry"=>1000,
                "Medium Fry"=>1200,
                "Large Fry"=>1400,
                "4 McNuggets"=>800,
                "6 McNuggets"=>900,
                "10 McNuggets"=>1000,
                "20 McNuggets"=>1100,
                "Regular Coffee"=>800,
                "Regular Tea"=>800,
                "Regular Soda"=>700,
                "Regular Smoothie"=>900,
                _=> 500
            },
            ingredients:match item_type{
                "Hamburger"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Cheeseburger"=>vec!["bun".to_string(), "beef patty".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Double Hamburger"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Double Cheeseburger"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "cheese".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "McDouble"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "cheese".to_string(), "ketchup".to_string(), "pickles".to_string(), "onions".to_string(), "mustard".to_string()],
                "Big Mac"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(), "mac sauce".to_string(), "pickles".to_string(), "lettuce".to_string()],
                "Quarter Pounder"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "onions slivers".to_string(), "mustard".to_string()],
                "Quarter Pounder with Cheese"=>vec!["bun".to_string(), "beef patty".to_string(), "ketchup".to_string(), "onions slivers".to_string(),"cheese".to_string(), "cheese".to_string(),"mustard".to_string()],
                "Double Quarter Pounder"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(),"ketchup".to_string(), "onions slivers".to_string(), "mustard".to_string()],
                "Double Quarter Pounder with Cheese"=>vec!["bun".to_string(), "beef patty".to_string(), "beef patty".to_string(),"ketchup".to_string(), "onions slivers".to_string(),"cheese".to_string(), "cheese".to_string(), "mustard".to_string()],
                _=> vec![]
            },
            starting_station:match item_type{
                "Hamburger"=>"grill".to_string(),
                "Cheeseburger"=>"grill".to_string(),
                "Double Hamburger"=>"grill".to_string(),
                "Double Cheeseburger"=>"grill".to_string(),
                "McDouble"=>"grill".to_string(),
                "Big Mac"=>"grill".to_string(),
                "Quarter Pounder"=>"grill".to_string(),
                "Quarter Pounder with Cheese"=>"grill".to_string(),
                "Double Quarter Pounder"=>"grill".to_string(),
                "Double Quarter Pounder with Cheese"=>"grill".to_string(),
                "4 McNuggets"=>"fry".to_string(),
                "6 McNuggets"=>"fry".to_string(),
                "10 McNuggets"=>"fry".to_string(),
                "20 McNuggets"=>"fry".to_string(),
                "Small Fry"=>"fry".to_string(),
                "Medium Fry"=>"fry".to_string(),
                "Large Fry"=>"fry".to_string(),
                "Regular Coffee"=>"drink".to_string(),
                "Regular Tea"=>"drink".to_string(),
                "Regular Soda"=>"drink".to_string(),
                "Regular Smoothie"=>"drink".to_string(),
                _=> "".to_string()
            },
            order_num:order_number,
            status:"in progress".to_string(),
        }
    }
    pub fn cook(&mut self){
        for i in 0..self.clone().number{
            println!("{} started cooking - {}",self.clone().str_name, self.clone().order_num.to_string());
            thread::sleep(time::Duration::from_millis(self.clone().cooking_time as u64));
            println!("{} finished cooking",self.clone().str_name);
        }
    }
    pub fn assemble(&mut self){
        for i in 0..self.clone().number{
            println!("{} is being assembled - {}",self.clone().str_name, self.clone().order_num.to_string());
            thread::sleep(time::Duration::from_millis(self.clone().assembly_time as u64)); //time to cook
            println!("{} finished being assembled",self.clone().str_name);
        }
    }
}

#[derive(Clone)]
pub struct GrillStation {
    queue: Vec<Item>,
    total_time: i32,
}
impl GrillStation {
    pub fn new() -> GrillStation {
        GrillStation {
            queue: vec![],
            total_time: 0,
        }
    }
    pub fn cook(&mut self){
        while self.queue.clone().len() > (0 as usize) {
            let mut item = &mut self.queue[0];
            item.cook();
            self.queue.drain(0..1);
        }
    }
    fn display(&mut self, ui: &mut Ui) {
        for (n, item) in self.queue.clone().iter().enumerate() {
            let mut label = (&item.str_name).to_owned();
            label.push_str(" (");
            label.push_str(&item.number.to_string());
            label.push_str(") - ");
            label.push_str(&item.order_num.to_string());
            let drag = Group::new(hash!("grill station", n), Vec2::new(270., 50.)) //width, height
                .draggable(true)
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &label); //left padding, upper padding
                });
        }
    }
}

#[derive(Clone)]
pub struct FryStation {
    queue: Vec<Item>,
    total_time: i32,
}
impl FryStation {
    pub fn new() -> FryStation {
        FryStation {
            queue: vec![],
            total_time: 0,
        }
    }
    pub fn cook(&mut self){
        while self.queue.clone().len() > (0 as usize) {
            let mut item = &mut self.queue[0];
            item.cook();
            self.queue.drain(0..1);
        }
    }
    fn display(&mut self, ui: &mut Ui) {
        for (n, item) in self.queue.clone().iter().enumerate() {
            let mut label = (&item.str_name).to_owned();
            label.push_str(" (");
            label.push_str(&item.number.to_string());
            label.push_str(") - ");
            label.push_str(&item.order_num.to_string());
            let drag = Group::new(hash!("fry station", n), Vec2::new(270., 50.)) //width, height
                .draggable(true)
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &label); //left padding, upper padding
                });
        }
    }
}

#[derive(Clone)]
pub struct DrinkStation {
    queue: Vec<Item>,
    total_time: i32,
}
impl DrinkStation {
    pub fn new() -> DrinkStation {
        DrinkStation {
            queue: vec![],
            total_time: 0,
        }
    }
    pub fn cook(&mut self){
        while self.queue.clone().len() > (0 as usize) {
            let item = &mut self.queue[0];
            item.cook();
            self.queue.drain(0..1);
        }
    }
    fn display(&mut self, ui: &mut Ui) {
        for (n, item) in self.queue.clone().iter().enumerate() {
            let mut label = (&item.str_name).to_owned();
            label.push_str(" (");
            label.push_str(&item.number.to_string());
            label.push_str(") - ");
            label.push_str(&item.order_num.to_string());
            let drag = Group::new(hash!("drink station", n), Vec2::new(270., 50.)) //width, height
                .draggable(true)
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &label); //left padding, upper padding
                });
        }
    }
}

#[derive(Clone)]
pub struct AssemblyStation {
    queue: Vec<Item>,
    total_time: i32,
}
impl AssemblyStation {
    pub fn new() -> AssemblyStation {
        AssemblyStation {
            queue: vec![],
            total_time: 0,
        }
    }
    pub fn assemble(&mut self){
        while self.queue.clone().len() > (0 as usize) {
            let item = &mut self.queue[0];
            item.assemble();
            self.queue.drain(0..1);
        }
    }
    fn display(&mut self, ui: &mut Ui) {
        for (n, item) in self.queue.clone().iter().enumerate() {
            let mut label = (&item.str_name).to_owned();
            label.push_str(" (");
            label.push_str(&item.number.to_string());
            label.push_str(") - ");
            label.push_str(&item.order_num.to_string());
            let drag = Group::new(hash!("assembly station", n), Vec2::new(270., 50.)) //width, height
                .draggable(true)
                .ui(ui, |ui| {
                    ui.label(Vec2::new(5., 10.), &label); //left padding, upper padding
                });
        }
    }
    fn draw_items(&mut self) {
        let mut num_items: f32 = 0.;
        for item in self.queue.clone() {
            let item_name = item.str_name.as_str();
        
            match item_name{
                "Hamburger"=>hamburger(650. + (num_items * 70.), 270.),
                "Cheeseburger"=>cheeseburger(650. + (num_items * 70.), 270.),
                "Double Hamburger"=>double_hamburger(650. + (num_items * 70.), 270.),
                "Double Cheeseburger"=>double_cheeseburger(650. + (num_items * 70.), 270.),
                "McDouble"=>mcdouble(650. + (num_items * 70.), 270.),
                "Big Mac"=>mcdouble(650. + (num_items * 70.), 270.),
                "Quarter Pounder"=>quarter_pounder(650. + (num_items * 70.), 270.),
                "Quarter Pounder with Cheese"=>quarter_pounder_with_cheese(650. + (num_items * 70.), 270.),
                "Double Quarter Pounder"=>double_quarter_pounder(650. + (num_items * 70.), 270.),
                "Double Quarter Pounder with Cheese"=>double_quarter_pounder_with_cheese(650. + (num_items * 70.), 270.),
                _=> raw_meat(500., 270.)
            }
            num_items += 1.;
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
    fn add_item(&mut self, order_number:i32, item_type:&str) {
        let length: usize = self.inventory.clone().len();
        let mut duplicate_item: bool = false; 
        for i in 0..length {
            if self.inventory[i].clone().str_name == item_type {
                self.inventory[i].number += 1;
                duplicate_item = true;
                break;
            }
        }
        if !duplicate_item {
            self.inventory.push(Item::new(item_type, order_number));
        }
        println!("{} added", item_type);
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

#[allow(dead_code)]
fn ham_bot(x : f32, y : f32) {
    let burg = Color::from_rgba(244, 164, 96, 250);
    draw_circle(x, y, 26.0, burg);
    draw_circle(x + 5.5, y, 26.0, burg);
}

#[allow(dead_code)]
fn ham_bun(x : f32, y : f32) {
    ham_bot(x, y);

    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
    }

    let vec : Vec<Offset> =  vec![Offset {x_coord : -18.0, y_coord : 0.0},
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
                Offset {x_coord : 13.0, y_coord : 13.0},
                
                Offset {x_coord : 22.5, y_coord : -8.0},
                Offset {x_coord : 25.5, y_coord : 3.0},
                Offset {x_coord : 20.5, y_coord : 11.0}];
    
    let seed = Color::from_rgba(255, 235, 205, 250);

    for n in 0..21 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, 1.5, seed);
    }
}

#[allow(dead_code)]
async fn leaf(x : f32, y : f32, lettuce:Texture2D) {
    let let_size = Vec2 {x: 50.0, y: 40.0};
    
    draw_texture_ex(
        lettuce,
        x - 23.5,
        y - 19.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(let_size),
            ..Default::default()
        },
    );
}

#[allow(dead_code)]
async fn tom(x : f32, y : f32, tom:Texture2D) {
    let tom_size = Vec2 {x: 40.0, y: 40.0};
    
    draw_texture_ex(
        tom,
        x - 16.5,
        y - 19.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(tom_size),
            ..Default::default()
        },
    );
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn raw_meat(x : f32, y : f32) {
    let meat = Color::from_rgba(240, 128, 128, 225);
    //let cooked = Color::from_rgba(160, 82, 45, 225);
    draw_circle(x, y, 23.0, meat);
}

#[allow(dead_code)]
fn cooked_meat(x : f32, y : f32) {
    let cooked = Color::from_rgba(160, 82, 45, 225);
    draw_circle(x + 2.5, y, 23.0, cooked);
    //draw_circle(504.0, 599.0, 23.0, RED);
}

#[allow(dead_code)]
async fn assembly_deco(lettuce_t:Texture2D, tom_t:Texture2D) {
    let grill = Color::from_rgba(220, 220, 220, 255);
    //draw_rectangle(1020.0, 30.0, 60.0, 60.0, grill);
    //draw_rectangle(1020.0, 105.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 180.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 255.0, 60.0, 60.0, grill);
    draw_rectangle(1020.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(945.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(870.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(795.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(720.0, 330.0, 60.0, 60.0, grill);
    draw_rectangle(620.0, 330.0, 85.0, 60.0, grill);
    
    let mac = Color::from_rgba(255, 228, 196, 255);
    tom(678.0, 367.0, tom_t).await;
    tom(660.0, 365.0, tom_t).await;
    tom(640.0, 368.0, tom_t).await;
    tom(643.0, 350.0, tom_t).await;
    tom(680.0, 354.0, tom_t).await;
    tom(654.0, 354.0, tom_t).await;
    leaf(750.0, 352.0, lettuce_t).await;
    leaf(745.0, 364.0, lettuce_t).await;
    pickles(820.0, 350.0);
    pickles(817.0, 365.0);
    pickles(827.0, 360.0);
    onions(912.0, 360.0);
    onions(897.0, 352.0);
    onions(888.0, 353.0);
    onions(892.0, 367.0);
    onions(910.0, 370.0);
    cheese(967.0, 355.0);
    cheese(980.0, 365.0);
    ketchup(1044.0, 367.0);
    ketchup(1055.0, 350.0);
    draw_circle(1040.0, 344.0, 10.0, RED);
    draw_circle(1033.0, 344.0, 10.0, RED);
    draw_circle(1033.0, 353.0, 10.0, RED);
    draw_circle(1065.0, 375.0, 10.0, RED);
    mustard(1044.0, 293.0);
    mustard(1055.0, 280.0);
    draw_circle(1038.0, 280.0, 13.0, YELLOW);
    draw_circle(1043.0, 273.0, 13.0, YELLOW);
    big_mac_sauce(1045.0, 217.0);
    big_mac_sauce(1052.0, 204.0);
    draw_circle(1064.0, 220.0, 13.0, mac);
    draw_circle(1040.0, 200.0, 13.0, mac);
}

#[allow(dead_code)]
fn ketchup(x : f32, y : f32) {
    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
        pub rad : f32,
    }

    let vec : Vec<Offset> =  vec![Offset {x_coord : -11.5, y_coord : 1.0, rad : 10.0},
                Offset {x_coord : -1.5, y_coord : 9.0, rad : 10.0},
                Offset {x_coord : 8.5, y_coord : 6.0, rad : 13.0},
                Offset {x_coord : -1.5, y_coord : -9.0, rad : 8.0},
                Offset {x_coord : 11.5, y_coord : -9.0, rad : 10.0}];

    for n in 0..5 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, vec[n].rad, RED);
    }
}

#[allow(dead_code)]
fn mustard(x : f32, y : f32) {
    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
        pub rad : f32,
    }

    let vec : Vec<Offset> =  vec![Offset {x_coord : -11.5, y_coord : 1.0, rad : 10.0},
                Offset {x_coord : -1.5, y_coord : 9.0, rad : 10.0},
                Offset {x_coord : 8.5, y_coord : 6.0, rad : 13.0},
                Offset {x_coord : -1.5, y_coord : -9.0, rad : 8.0},
                Offset {x_coord : 11.5, y_coord : -9.0, rad : 10.0}];

    for n in 0..5 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, vec[n].rad, YELLOW);
    }
}

#[allow(dead_code)]
fn big_mac_sauce(x : f32, y : f32) {
    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
        pub rad : f32,
    }

    let mac = Color::from_rgba(255, 228, 196, 255);

    let vec : Vec<Offset> =  vec![Offset {x_coord : -11.5, y_coord : 1.0, rad : 10.0},
                Offset {x_coord : -1.5, y_coord : 9.0, rad : 10.0},
                Offset {x_coord : 8.5, y_coord : 6.0, rad : 13.0},
                Offset {x_coord : -1.5, y_coord : -9.0, rad : 8.0},
                Offset {x_coord : 11.5, y_coord : -9.0, rad : 10.0}];

    for n in 0..5 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, vec[n].rad, mac);
    }
}

#[allow(dead_code)]
fn pickles(x : f32, y : f32) {
    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
    }

    let pick = Color::from_rgba(143, 188, 143, 255);
    
    let vec : Vec<Offset> =  vec![Offset {x_coord : 13.5, y_coord : -9.0},
                Offset {x_coord : -11.5, y_coord : 0.0},
                Offset {x_coord : 8.5, y_coord : 11.0}];
    
    for n in 0..3 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, 8.75, DARKGREEN);
    }

    for n in 0..3 {
        draw_circle(vec[n].x_coord + x, vec[n].y_coord + y, 7.0, pick);
    }
}

#[allow(dead_code)]
fn onions(x : f32, y : f32) {
    pub struct Offset {
        pub x_coord : f32,
        pub y_coord : f32,
        pub rad : f32,
    }
    
    let oni = Color::from_rgba(255, 255, 240, 255);
    
    let vec : Vec<Offset> =  vec![Offset {x_coord : -7.5, y_coord : 3.0, rad : 10.0},
                Offset {x_coord : 2.5, y_coord : -11.0, rad : 13.0},
                Offset {x_coord : 8.5, y_coord : -1.0, rad : 8.0},
                Offset {x_coord : 1.0, y_coord : 10.0, rad : 10.0}];

    for n in 0..4 {
        draw_circle_lines(vec[n].x_coord + x, vec[n].y_coord + y, 8.0, 3.0, oni);
    }
}

#[allow(dead_code)]
fn cheese(x : f32, y : f32) {
    let chee = Color::from_rgba(255, 215, 0, 255);
    draw_rectangle(x - 18.5, y - 19.0, 40.0, 40.0, chee);
}

#[allow(dead_code)]
fn hamburger(x : f32, y : f32) {
    ham_bot(x - 1.5, y + 14.0);
    cooked_meat(x - 1.5, y + 11.6);
    ketchup(x - 1.5, y + 9.5);
    pickles(x - 1.5, y + 7.0);
    onions(x - 1.5, y + 4.5);
    mustard(x - 1.5, y - 2.5);
    ham_bun(x - 1.5, y - 15.0);
}

#[allow(dead_code)]
fn cheeseburger(x : f32, y : f32) {
    ham_bot(x - 1.5, y + 19.0);
    cooked_meat(x - 1.5, y + 16.6);
    cheese(x - 1.5, y + 13.0);
    ketchup(x - 1.5, y + 11.0);
    pickles(x - 1.5, y + 8.0);
    onions(x - 1.5, y + 5.0);
    mustard(x - 1.5, y + 1.0);
    ham_bun(x - 1.5, y - 11.0);
}

#[allow(dead_code)]
fn double_hamburger(x : f32, y : f32) {
    ham_bot(x - 3.0, y + 15.0);
    cooked_meat(x - 3.0, y + 13.0);
    ketchup(x - 3.0, y + 11.0);
    pickles(x - 3.0, y + 9.0);
    onions(x - 3.0, y + 7.0);
    cooked_meat(x - 3.0, y);
    mustard(x - 3.0, y - 3.0);
    ham_bun(x - 3.0, y - 14.0);
}

#[allow(dead_code)]
fn double_cheeseburger(x : f32, y : f32) {
    ham_bot(x - 3.0, y + 20.0);
    cooked_meat(x - 3.0, y + 18.0);
    cheese(x - 3.0, y + 16.6);
    ketchup(x - 3.0, y + 15.0);
    pickles(x - 3.0, y + 14.0);
    onions(x - 3.0, y + 13.0);
    mustard(x - 3.0, y + 8.0);
    cheese(x - 3.0, y + 2.0);
    ham_bun(x - 3.0, y - 7.0);
}

#[allow(dead_code)]
fn mcdouble(x : f32, y : f32) {
    ham_bot(x - 3.0, y + 20.0);
    cooked_meat(x - 3.0, y + 18.0);
    cheese(x - 3.0, y + 16.6);
    ketchup(x - 3.0, y + 15.0);
    pickles(x - 3.0, y + 14.0);
    onions(x - 3.0, y + 13.0);
    mustard(x - 3.0, y + 8.0);
    cooked_meat(x - 3.0, y + 2.0);
    ham_bun(x - 3.0, y - 7.0);
}

#[allow(dead_code)]
fn quarter_pounder(x : f32, y : f32) {
    ham_bot(x - 1.5, y + 14.0);
    cooked_meat(x - 1.5, y + 11.6);
    ketchup(x - 1.5, y + 9.5);
    pickles(x - 1.5, y + 7.0);
    onions(x - 1.5, y + 4.5);
    mustard(x - 1.5, y - 2.5);
    ham_bun(x - 1.5, y - 15.0);
}

#[allow(dead_code)]
fn quarter_pounder_with_cheese(x : f32, y : f32) {
    ham_bot(x - 1.5, y + 19.0);
    cooked_meat(x - 1.5, y + 16.6);
    cheese(x - 1.5, y + 13.0);
    ketchup(x - 1.5, y + 11.0);
    pickles(x - 1.5, y + 8.0);
    onions(x - 1.5, y + 5.0);
    mustard(x - 1.5, y + 1.0);
    ham_bun(x - 1.5, y - 11.0);
}

#[allow(dead_code)]
fn double_quarter_pounder(x : f32, y : f32) {
    ham_bot(x - 3.0, y + 15.0);
    cooked_meat(x - 3.0, y + 13.0);
    ketchup(x - 3.0, y + 11.0);
    pickles(x - 3.0, y + 9.0);
    onions(x - 3.0, y + 7.0);
    cooked_meat(x - 3.0, y);
    mustard(x - 3.0, y - 3.0);
    ham_bun(x - 3.0, y - 14.0);
}

#[allow(dead_code)]
fn double_quarter_pounder_with_cheese(x : f32, y : f32) {
    ham_bot(x - 3.0, y + 20.0);
    cooked_meat(x - 3.0, y + 18.0);
    cheese(x - 3.0, y + 16.6);
    ketchup(x - 3.0, y + 15.0);
    pickles(x - 3.0, y + 14.0);
    onions(x - 3.0, y + 13.0);
    mustard(x - 3.0, y + 8.0);
    cheese(x - 3.0, y + 2.0);
    ham_bun(x - 3.0, y - 7.0);
}

#[allow(dead_code)]
fn bag(x : f32, y : f32) {
    draw_rectangle(x, y, 60., 90., Color::from_rgba(160, 82, 45, 255));
}

#[allow(dead_code)]
fn fries() {
    let fr = Color::from_rgba(245, 245, 220, 225);
    draw_line(80.0, 340.0, 90.0, 355.0, 3.0, fr);
    draw_line(55.0, 340.0, 35.0, 345.0, 3.0, fr);
    draw_line(66.0, 350.0, 70.0, 365.0, 3.0, fr);
    draw_line(40.0, 360.0, 55.0, 362.0, 3.0, fr);
    draw_line(57.0, 370.0, 43.0, 380.0, 3.0, fr);
    draw_line(70.0, 380.0, 86.0, 380.0, 3.0, fr);
    draw_line(90.0, 360.0, 80.0, 375.0, 3.0, fr);
}

#[allow(dead_code)]
fn nuggets() {
    let nug = Color::from_rgba(218, 165, 32, 225);
    draw_circle(50.0, 448.0, 5.5, nug);
    draw_circle(65.0, 435.0, 5.5, nug);
    draw_circle(43.0, 430.0, 5.5, nug);
    draw_circle(47.0, 410.0, 5.5, nug);
    draw_circle(63.0, 416.0, 5.5, nug);
    draw_circle(84.0, 447.0, 5.5, nug);
    draw_circle(90.0, 430.0, 5.5, nug);
    draw_circle(82.0, 413.0, 5.5, nug);
}

#[allow(dead_code)]
fn done_fries(x : f32, y : f32) {
    pub struct Offset {
        pub x1 : f32,
        pub y1 : f32,
        pub x2 : f32,
        pub y2 : f32,
    }

    let fri = Color::from_rgba(255, 165, 0, 225);
    
    let vec : Vec<Offset> =  vec![Offset {x1 : -12.0, y1 : -27.0, x2 : -8.0, y2 : -10.0},
                        Offset {x1 : -5.0, y1 : -25.0, x2 : -5.0, y2 : -10.0},
                        Offset {x1 : 1.0, y1 : -25.0, x2 : -2.0, y2 : -10.0},
                        Offset {x1 : 3.0, y1 : -28.0, x2 : 5.0, y2 : -10.0},
                        Offset {x1 : 9.0, y1 : -25.0, x2 : 5.0, y2 : -10.0}];

    for n in 0..5 {
        draw_line(vec[n].x1 + x, vec[n].y1 + y, vec[n].x2 + x, vec[n].y2 + y, 3.0, fri);
    }

    draw_rectangle(x - 13.0, y - 16.0, 23.0, 25.0, RED);
    draw_text("W", x - 7.0, y + 3.0, 25.0, YELLOW);
}

//#[macroquad::main("lovin_it")]
#[macroquad::main(window_conf)]
async fn main() {

    let (tx, rx) = mpsc::channel();
    let mut grill_empty = true;
    let mut fry_empty = true;
    let mut drink_empty = true;
    let mut assembly_empty = true;

    let mut order_ready = false;
    let mut order_ready_num = 1;

    let mut contains_fries = false;
    let mut contains_nuggets = false;

    let mut order_number: i32 = 1;
    let mut grill_station = GrillStation::new();
    let mut fry_station = FryStation::new();
    let mut drink_station = DrinkStation::new();
    let mut assembly_station = AssemblyStation::new();

    let mut grill_now = Instant::now();
    let mut fry_now = Instant::now();
    let mut drink_now = Instant::now();
    let mut assembly_now = Instant::now();

    //adding image into program
    
    let lettuce_t: Texture2D = load_texture("images/lettuce.png").await.unwrap();
    let tom_t: Texture2D = load_texture("images/tom.png").await.unwrap();
    let cashier_t: Texture2D = load_texture("images/register.png").await.unwrap();
    
    
    // let worker1: Texture2D = load_texture("images/worker1.png").await.unwrap();
    // let worker2: Texture2D = load_texture("images/worker2.png").await.unwrap();
    // let worker3: Texture2D = load_texture("images/worker3.png").await.unwrap();
    // let worker4: Texture2D = load_texture("images/worker4.png").await.unwrap();
    // let worker5: Texture2D = load_texture("images/worker5.png").await.unwrap();


    
    // let worker1: Texture2D = load_texture("images/nowak.png").await.unwrap();
    // let worker2: Texture2D = load_texture("images/cosman.png").await.unwrap();
    // let worker3: Texture2D = load_texture("images/challen.png").await.unwrap();
    // let worker4: Texture2D = load_texture("images/wade.png").await.unwrap();
    let worker5: Texture2D = load_texture("images/fleck.png").await.unwrap();
    
    
    let mut order = Order::new();
    let mut orders:Vec<Order> = vec![];
    let mut grill_orders:Vec<Order> = vec![];
    let mut fry_orders:Vec<Order> = vec![];
    let mut drink_orders:Vec<Order> = vec![];
    let mut assembly_orders:Vec<Order> = vec![];

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
    raw_meat(68.0, 254.5);

    //fries
    draw_rectangle(20.0, 300.0, 400.0, 300.0, WHITE);
    draw_rectangle(20.0, 300.0, 400.0, 300.0, fries_floor);
    
    draw_text("Frying", 350.0, 323.0, 25.0, BLACK);

    //decorations
    draw_rectangle(20.0, 300.0, 105.0, 300.0, counter);
    //draw_rectangle(20.0, 490.0, 400.0, 100.0, counter);
    draw_rectangle(20.0, 305.0, 90.0, 193.0, grill);
    
    draw_rectangle(25.0, 310.0, 80.0, 180.0, oil);

    draw_rectangle_lines(30.0, 330.0, 70.0, 60.0, 5.0, GRAY); //fries fryer
    draw_rectangle_lines(100.0, 355.0, 30.0, 5.0, 5.0, GRAY); //fries fryer handle
    draw_rectangle_lines(30.0, 400.0, 70.0, 60.0, 5.0, GRAY); //nuggets fryer
    draw_rectangle_lines(100.0, 425.0, 30.0, 5.0, 5.0, GRAY); //nuggets fryer handle

    //drinks
    draw_rectangle(20.0, 580.0, 580.0, 300.0, WHITE);
    draw_rectangle(20.0, 580.0, 580.0, 300.0, drink_floor);
    
    draw_text("Drinks", 350.0, 615.0, 25.0, BLACK);

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
    
    draw_text("Assembly", 650.0, 40.0, 25.0, BLACK);

    //decorations
    draw_rectangle(610.0, 400.0, 475.0, 65.0, counter);
    draw_rectangle(610.0, 325.0, 475.0, 75.0, counter);
    draw_rectangle(1010.0, 20.0, 75.0, 400.0, counter);
    assembly_deco(lettuce_t, tom_t).await;

    //partitions/walls
    draw_line(20.0, 300.0, 420.0, 300.0, 10.0, BLACK);
    draw_line(20.0, 580.0, 420.0, 580.0, 10.0, BLACK);
    draw_line(610.0, 400.0, 1085.0, 400.0, 10.0, BLACK);
    draw_line(600.0, 575.0, 600.0, 880.0, 10.0, BLACK);

    //progress bars
    draw_rectangle_lines(130.0, 25.0, 200.0, 20.0, 5.0, GRAY); //grill
    draw_rectangle_lines(130.0, 310.0, 200.0, 20.0, 5.0, GRAY);
    draw_rectangle_lines(130.0, 600.0, 200.0, 20.0, 5.0, GRAY);
    draw_rectangle_lines(740.0, 25.0, 200.0, 20.0, 5.0, GRAY); //assembly

    //drawing the image
    //texture methods for image manipulation
    
    //https://github.com/not-fl3/macroquad/blob/master/src/texture.rs
        // draw_texture_ex(
        //     worker1,
        //     150.0,
        //     50.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: None,
        //         ..Default::default()
        //     },
        // );
        // draw_texture_ex(
        //     worker2,
        //     150.0,
        //     325.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: None,
        //         ..Default::default()
        //     },
        // );
        // draw_texture_ex(
        //     worker3,
        //     150.0,
        //     575.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: None,
        //         ..Default::default()
        //     },
        // );
        // draw_texture_ex(
        //     worker4,
        //     800.0,
        //     75.0,
        //     WHITE,
        //     DrawTextureParams {
        //         dest_size: None,
        //         ..Default::default()
        //     },
        // );
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
                    order.add_item(order_number, "Hamburger");
                }
                ui.separator();
                if ui.button(None, "Cheeseburger") {
                    order.add_item(order_number, "Cheeseburger");
                }
                ui.separator();
                if ui.button(None, "Double Hamburger") {
                    order.add_item(order_number, "Double Hamburger");
                }
                ui.separator();
                if ui.button(None, "Double Cheeseburger") {
                    order.add_item(order_number, "Double Cheeseburger");
                }
                ui.separator();
                if ui.button(None, "McDouble") {
                    order.add_item(order_number, "McDouble");
                }
                ui.separator();
                if ui.button(None, "Big Mac") {
                    order.add_item(order_number, "Big Mac");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder") {
                    order.add_item(order_number, "Quarter Pounder");
                }
                ui.separator();
                if ui.button(None, "Quarter Pounder with Cheese") {
                    order.add_item(order_number, "Quarter Pounder with Cheese");
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder") {
                    order.add_item(order_number, "Double Quarter Pounder");
                }
                ui.separator();
                if ui.button(None, "Double Quarter Pounder with Cheese") {
                    order.add_item(order_number, "Double Quarter Pounder with Cheese");
                }
                ui.separator();
            });
            ui.tree_node(hash!(), "Sides", |ui| {
                if ui.button(None, "Small Fry") {
                    order.add_item(order_number, "Small Fry");
                }
                ui.separator();
                if ui.button(None, "Medium Fry") {
                    order.add_item(order_number, "Medium Fry");
                }
                ui.separator();
                if ui.button(None, "Large Fry") {
                    order.add_item(order_number, "Large Fry");
                }
                ui.separator();
                if ui.button(None, "4 McNuggets") {
                    order.add_item(order_number, "4 McNuggets");
                }
                ui.separator();
                if ui.button(None, "6 McNuggets") {
                    order.add_item(order_number, "6 McNuggets");
                }
                ui.separator();
                if ui.button(None, "10 McNuggets") {
                    order.add_item(order_number, "10 McNuggets");
                }
                ui.separator();
                if ui.button(None, "20 McNuggets") {
                    order.add_item(order_number, "20 McNuggets");
                }
                ui.separator();
            });
            ui.tree_node(hash!(), "Drinks", |ui| {
                if ui.button(None, "Regular Coffee") {
                    order.add_item(order_number, "Regular Coffee");
                }
                ui.separator();
                if ui.button(None, "Regular Tea") {
                    order.add_item(order_number, "Regular Tea");
                }
                ui.separator();
                if ui.button(None, "Regular Smoothie") {
                    order.add_item(order_number, "Regular Smoothie");
                }
                ui.separator();
                if ui.button(None, "Regular Soda") {
                    order.add_item(order_number, "Regular Soda");
                }
            });
        });

    let mut label = "Order ".to_owned();
    label.push_str(&order_number.to_string());

    widgets::Window::new(hash!(), vec2(1110., 450.), vec2(300., 400.))
    .label(&label)
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(290., 380.)).ui(ui, |ui| {
            order.inventory(ui);
            if ui.button(Vec2::new(102., order.inventory.len() as f32 * 52.0 + 10 as f32), "Place Order") {
                println!("Order Placed!");

                //cache orders according to stations
                //orders.push(order.clone());

                if order.inventory.len() != 0 {
                    orders.push(order.clone());

                    for item in order.clone().inventory { //add to grilling station queue
                        let final_item = item.clone();
                        if final_item.starting_station == "grill" {
                            grill_orders.push(order.clone());
                            break;
                        }
                    }

                    for item in order.clone().inventory { //add to frying station queue
                        let final_item = item.clone();
                        if final_item.starting_station == "fry" {
                            fry_orders.push(order.clone());
                            break;
                        }
                    }

                    for item in order.clone().inventory { //add to drink station queue
                        let final_item = item.clone();
                        if final_item.starting_station == "drink" {
                            drink_orders.push(order.clone());
                            break;
                        }
                    }

                    order.clear();
                    order_number += 1;

                }
            }
        });
    });

    //if first==true or rx has been received (no processes running in background) 
    //pop from orders vector to get next order in line
    //separate into different starting stations
    //run thread

    let txg = tx.clone(); //grill
    let txf = tx.clone(); //fry
    let txd = tx.clone(); //drink
    let txa = tx.clone(); //assembly
    let received = rx.try_iter().next();

    let mut assembly_ready = true;
    //find all orders ready for assembly
    let orders_cloned = orders.clone();
    for i in 0..orders_cloned.len() {
        assembly_ready = true;
        let order_ready = &orders_cloned[i];

        if order_ready.inventory.len() == 0 {
            break;
        }

        let order_ready_num = order_ready.inventory[0].order_num;

        //check if order is still getting ready at grill
        for grill_order in &grill_orders.clone() {
            let num = grill_order.inventory[0].order_num;
            if num == order_ready_num {
                assembly_ready = false;
                break;
            }
        }
        //check if order still getting ready at fry
        if assembly_ready {
            for fry_order in &fry_orders.clone() {
                let num = fry_order.inventory[0].order_num;
                if num == order_ready_num {
                    assembly_ready = false;
                    break;
                }
            }
        }
        //check if order still getting ready at drink
        if assembly_ready {
            for drink_order in &drink_orders.clone() {
                let num = drink_order.inventory[0].order_num;
                if num == order_ready_num {
                    assembly_ready = false;
                    break;
                }
            }
        }

        if (assembly_ready) {
            assembly_orders.push(order_ready.clone());
            orders.drain(i..(i + 1));
        }
    }

    let received_assembly1 = received.clone();
    let received_assembly2 = received.clone();
    let received_assembly3 = received.clone();
    //check if order has been assembled
    if (!received_assembly1.is_none() && received_assembly1.unwrap() == "assembly") {
        order_ready = true;
        order_ready_num = assembly_orders[0].inventory[0].order_num;
        assembly_orders.drain(0..1);
        assembly_station.total_time = 0;
    }
    //all orders in assembly queue have been assembled  
    if assembly_orders.len() == 0 && (!received_assembly2.is_none() && received_assembly2.unwrap() == "assembly") {
        assembly_station.queue.clear();
        assembly_empty = true;
        assembly_station.total_time = 0;
    }
    //there are assembly orders ready to process and no assembly orders running in background
    if assembly_orders.clone().len() > 0 && (assembly_empty || (!received_assembly3.is_none() && received_assembly3.unwrap() == "assembly")) { 
        assembly_station.queue.clear();
        let placed_order = assembly_orders[0].clone().inventory; //get order
        assembly_empty = false;
        assembly_station.total_time = 0;

        for item in placed_order { //add to grilling station queue
            let final_item = item.clone();
            assembly_station.queue.push(final_item.clone());
            assembly_station.total_time += (final_item.assembly_time * final_item.number);
        }
        let mut y = assembly_station.clone();
        assembly_now = Instant::now();
        let item_cooking = thread::spawn(move || {
            y.assemble();
            if y.queue.clone().len() == (0 as usize) {
                let val = String::from("assembly");
                txa.send(val).unwrap();
            }
        });
    }

    let received_grill1 = received.clone();
    let received_grill2 = received.clone();
    let received_grill3 = received.clone();

    //check if order has been completed
    if (!received_grill3.is_none() && received_grill3.unwrap() == "grill") {
        grill_orders.drain(0..1);
        grill_station.total_time = 0;
    }

    //all grill orders have been processed    
    if grill_orders.len() == 0 && (!received_grill1.is_none() && received_grill1.unwrap() == "grill") {
        grill_station.queue.clear();
        grill_empty = true;
        grill_station.total_time = 0;
    }
    
    //there are grill orders ready to process and no grill orders running in background
    if grill_orders.clone().len() > 0 && (grill_empty || (!received_grill2.is_none() && received_grill2.unwrap() == "grill")) { 
        grill_station.total_time = 0;
        grill_station.queue.clear();
        let placed_order = grill_orders[0].clone().inventory; //get order
        grill_empty = false;

        for item in placed_order { //add to grilling station queue
            let final_item = item.clone();
            if final_item.starting_station == "grill" {
                grill_station.queue.push(final_item.clone());
                grill_station.total_time += (final_item.cooking_time * final_item.number);
            }
        }
        let mut y = grill_station.clone();

        grill_now = Instant::now();

        let item_cooking = thread::spawn(move || {
            y.cook();
            if y.queue.clone().len() == (0 as usize) {
                let val = String::from("grill");
                txg.send(val).unwrap();
            }
        });
    }

    //all fry orders have been processed  
    let received_fry1 = received.clone();  
    let received_fry2 = received.clone();  
    let received_fry3 = received.clone();  

    //check if order has been completed
    if (!received_fry3.is_none() && received_fry3.unwrap() == "fry") {
        fry_orders.drain(0..1);
        fry_station.total_time = 0;
        contains_fries = false;
        contains_nuggets = false;
    }

    if fry_orders.len() == 0 && (!received_fry1.is_none() && received_fry1.unwrap() == "fry") {
        fry_station.queue.clear();
        fry_empty = true;
        fry_station.total_time = 0;
        contains_fries = false;
        contains_nuggets = false;
    }
    
    //there are fry orders ready to process and no fry orders running in background
    if fry_orders.clone().len() > 0 && (fry_empty || (!received_fry2.is_none() && received_fry2.unwrap() == "fry")) { 
        fry_station.queue.clear();
        fry_empty = false;
        fry_station.total_time = 0;
        let placed_order = fry_orders[0].clone().inventory; //get order

        for item in placed_order { //add to grilling station queue
            let final_item = item.clone();
            if final_item.starting_station == "fry" {
                if final_item.str_name == "Small Fry".to_string() || final_item.str_name == "Medium Fry".to_string() || final_item.str_name == "Large Fry".to_string() {
                    contains_fries = true;
                } else {
                    contains_nuggets = true;
                }
                fry_station.queue.push(final_item.clone());
                fry_station.total_time += (final_item.cooking_time * final_item.number);
            }
        }

        let mut y = fry_station.clone();
        fry_now = Instant::now();
        let item_cooking = thread::spawn(move || {
            y.cook();
            if y.queue.clone().len() == (0 as usize) {
                let val = String::from("fry");
                txf.send(val).unwrap();
            }
        });
    }

    //all drink orders have been processed  
    let received_drink1 = received.clone();  
    let received_drink2 = received.clone();  
    let received_drink3 = received.clone();  

    //check if order has been completed
    if (!received_drink3.is_none() && received_drink3.unwrap() == "drink") {
        drink_orders.drain(0..1);
        drink_station.total_time = 0;
    }

    if drink_orders.len() == 0 && (!received_drink1.is_none() && received_drink1.unwrap() == "drink") {
        drink_station.queue.clear();
        drink_empty = true;
        drink_station.total_time = 0;
    }
    
    //there are drink orders ready to process and no drink orders running in background
    if drink_orders.clone().len() > 0 && (drink_empty || (!received_drink2.is_none() && received_drink2.unwrap() == "drink")) { 
        drink_station.queue.clear();
        drink_empty = false;
        drink_station.total_time = 0;
        let placed_order = drink_orders[0].clone().inventory; //get order

        for item in placed_order { //add to grilling station queue
            let final_item = item.clone();
            if final_item.starting_station == "drink" {
                drink_station.queue.push(final_item.clone());
                drink_station.total_time += (final_item.cooking_time * final_item.number);
            }
        }

        let mut y = drink_station.clone();
        drink_now = Instant::now();
        let item_cooking = thread::spawn(move || {
            y.cook();
            if y.queue.clone().len() == (0 as usize) {
                let val = String::from("drink");
                txd.send(val).unwrap();
            }
        });
    }

    widgets::Window::new(hash!(), vec2(130., 50.), vec2(290., 200.))
    .label("Grill Station")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(280., 180.)).ui(ui, |ui| {
            grill_station.display(ui);
        });
    });

    widgets::Window::new(hash!(), vec2(130., 340.), vec2(290., 200.))
    .label("Fry Station")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(280., 180.)).ui(ui, |ui| {
            fry_station.display(ui);
        });
    });

    widgets::Window::new(hash!(), vec2(130., 625.), vec2(290., 150.))
    .label("Drink Station")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(280., 140.)).ui(ui, |ui| {
            drink_station.display(ui);
        });
    });

    widgets::Window::new(hash!(), vec2(650., 50.), vec2(290., 150.))
    .label("Assembly Station")
    .titlebar(true)
    .ui(&mut *root_ui(), |ui| {
        Group::new(hash!(), Vec2::new(280., 140.)).ui(ui, |ui| {
            assembly_station.display(ui);
        });
    });

    if !grill_empty {
        cooked_meat(68.0, 100.);
        let new_now = Instant::now();
        let x = new_now.duration_since(grill_now).as_millis() as f32;
        let y = grill_station.total_time as f32;
        draw_rectangle(130.0, 25.0, 200.0 * (x / y), 20.0, Color::from_rgba(50, 205, 50, 250));
    }

    if !fry_empty {
        if contains_fries {
            fries();
        }
        if contains_nuggets {
            nuggets();
        }
        let new_now = Instant::now();
        let x = new_now.duration_since(fry_now).as_millis() as f32;
        let y = fry_station.total_time as f32;
        draw_rectangle(130.0, 310.0, 200.0 * (x / y), 20.0, Color::from_rgba(50, 205, 50, 250));
    }

    if !drink_empty {
        let new_now = Instant::now();
        let x = new_now.duration_since(drink_now).as_millis() as f32;
        let y = drink_station.total_time as f32;
        draw_rectangle(130.0, 600.0, 200.0 * (x / y), 20.0, Color::from_rgba(50, 205, 50, 250));
    }

    if !assembly_empty {
        assembly_station.draw_items();
        let new_now = Instant::now();
        let x = new_now.duration_since(assembly_now).as_millis() as f32;
        let y = assembly_station.total_time as f32;
        draw_rectangle(740.0, 25.0, 200.0 * (x / y), 20.0, Color::from_rgba(50, 205, 50, 250));
    }

    let txb = tx.clone();
    let received_bag = received.clone();

    if (!received_bag.is_none() && received_bag.unwrap() == "done") {
        order_ready = false;
    }

    if order_ready {
        bag(650., 420.);
        draw_text(&order_ready_num.to_string(), 670.0, 460.0, 25.0, BLACK);
        let show_bag = thread::spawn(move || {
            thread::sleep(time::Duration::from_millis(3000));
            let val = String::from("done");
            txb.send(val).unwrap();
        });
    }

        next_frame().await
    }
}