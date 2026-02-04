use std::io;

fn main() {
    // 느낌표 == 매크로 == 메타프로그래밍 == 비정형 함수
    // println!("Number: {}", 4);
    // println!("Number: {}, String: {}", 4, "ABC");
    // let mut mars_weight = calculate_weight_on_mars(100.0);

    let mars_weight = calculate_weight_on_mars(100.0);
    let mut input_weight = String::new();
    let _ = io::stdin().read_line(&mut input_weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(_weight: f32) -> f32 {
    (_weight / 9.81) * 3.711
}