use std::io::{self, Write};

//摂氏 celsius 華氏 fahrenheit

//to_celsius(華氏から摂氏)の逆のことをするだけ
fn to_fahrenheit(celsius: f32) -> f32 {
    return (celsius * 1.8) + 32.0;
}

fn to_celsius(fahrenheit: f32) -> f32 {
    return (fahrenheit - 32.0) / 1.8;
}

fn main() {
    println!("摂氏から華氏または華氏から摂氏に温度を変換します。\n");

    loop {
        print!("温度を入力してください : ");

        io::stdout().flush().unwrap();

        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line!");

        let temperature: f32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("数字を入力してください");
                continue;
            }
        };

        let fahrenheit = to_fahrenheit(temperature);
        let celsius = to_celsius(temperature);

        println!("\n");
        colour::green!("華氏: {}°F => 摂氏: {}°C\n", temperature, celsius);
        colour::blue!("摂氏: {}°C => 華氏: {}°F", temperature, fahrenheit);
        break;
    }
}
