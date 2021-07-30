//第一题 交通信号灯
trait Time{
    fn light_time(&self)->u32{
        0
    }
}

enum TrafficLight{
    Red,
    Green,
    Yellow,
}

impl Time for TrafficLight{
    fn light_time(&self)->u32{
        match self{
            TrafficLight::Red => 30 ,
            TrafficLight::Green => 60 ,
            TrafficLight::Yellow => 3 ,
        }
    }
}


//第二题 整数集合求和
fn get_sum(x:Vec<u32>)->Option<u32>{
    let mut result:u32 = 0;
    for i in &x{
        match result.checked_add(*i) {
            None => {
                println!("overflow!");}
            _    => {
                result = result + i;}
        }
    }
        Some(result)
}

//第三题
//use std::ops::Mul;
//use std::convert::{Into};

struct rectangle{
    x:f32,
    y:f32
}

struct circle{
    r:f32,
}

struct triangle{
    d:f32,
    h:f32,
}

pub trait Area{
    fn get_area(&self)->f32{
        println!("get the area ...");
        0.0
    }
}

/*
impl<T,U> rectangle<T,U>{
    fn get_area(&self)->T
        where T: Mul<Output = T> + Copy,
              U: Into<T> + Copy 
        {
        self.x.mul(self.y.into())
        }
}
*/

fn cal_Area<T:Area> (shape:T)->f32{
    shape.get_area()
}

impl Area for rectangle{
    fn get_area(&self)->f32
        {
        self.x * self.y
        }
}

impl Area for circle{
    fn get_area(&self)->f32
        {
        self.r * self.r * 3.1415 * 0.5
        }
}

impl Area for triangle{
    fn get_area(&self)->f32
        {
        self.d * self.h * 0.5
        }
}

fn main(){

    //第一题
    let light_red = TrafficLight::Red;
    let light_green = TrafficLight::Green;
    let light_yellow = TrafficLight::Yellow;
    println!("第一题：");
    println!("Time of light_red: {}",light_red.light_time());
    println!("Time of light_green: {}",light_green.light_time());
    println!("Time of light_yellow: {}",light_yellow.light_time());

    //第二题
    let number_list = [34,50,25,100,65];
    let res = get_sum((&number_list).to_vec());
    println!("第二题：");
    println!("numbers:{:?}",number_list);
    println!("sum:{:?}",res);

    //第三题
    println!("第三题：");

    let mut rec = rectangle{x:8.0,y:6.0};
    println!("rectangle area:{}",cal_Area(rec));

    let mut cir = circle{r:1.0};
    println!("circle area:{}",cal_Area(cir));

    let mut tri = triangle{d:3.0,h:2.0};
    println!("triangle area:{}",cal_Area(tri));
}