pub fn pick(el: &str, index: i32) {


    match el {
        "red" => {
            println!("{} {}", index, uppercased_text("RED".to_string()));
        }

        "green" => {
            println!("{} {}", index, uppercased_text("GREEN".to_string()));
        }

        "blue" => {

            println!("{} {}", index, uppercased_text("BLUE".to_string()));
        }

        &_ => todo!()
    }

}

pub fn uppercased_text(str: String) -> String {

    str.to_uppercase()

}