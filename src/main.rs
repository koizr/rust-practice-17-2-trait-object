extern crate gui;
use gui::{Button, Draw, Screen};

struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "<SelectBox width=\"{}\" height=\"{}\">",
            self.width, self.height
        );
        for option in self.options.iter() {
            println!("    <Option>{}</Option>", option);
        }
        println!("</SelectBox>");
    }
}

fn main() {
    // トレイトオブジェクトを型引数に持っている場合、トレイト境界を満たしていれば
    // 異なる型の値を同時に持たせることができる
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("Yes"), String::from("No")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
