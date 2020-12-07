pub trait Draw {
    // トレイトのすべてのメソッドが以下の条件を満たすことでトレイトオブジェクトにできる
    // - 戻り値の型に Self がない（戻り値の型が Self になるとコンパイル時にメモリサイズを決定できない）
    // - 型引数が出現しない（型引数が出現するとコンパイル時にメモリサイズを決定できない）
    fn draw(&self);
}

pub struct Screen {
    // Box<dyn Draw> がトレイトオブジェクト（だと思う）
    // コンパイル時にオブジェクトのメモリサイズを決定できるようにスマートポインタにする
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            // 多態を利用したメソッド呼び出しが可能
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "<Button width=\"{}\" height=\"{}\">{}</Button>",
            self.width, self.height, self.label
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
