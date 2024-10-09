enum DayTime<T>{
      
    Morning(T),
    Evening(T)
}
 
fn main(){
     
    let morning = DayTime::Morning("Доброе утро".to_string());  // параметр T представляет тип String
    if let DayTime::Morning(morning_value)= morning { println!("Morning: {}", morning_value);}
     
      
    let evening = DayTime::Evening(16);     // параметр T представляет тип i32
    if let DayTime::Evening(evening_value)= evening { println!("Evening: {}", evening_value);}
}