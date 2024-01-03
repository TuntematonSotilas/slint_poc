fn main() {
    MainWindow::new().unwrap().run().unwrap();
}


slint::slint! {

    export struct Palette {
        gradient: brush
    }

    export global Theme {
        in property <Palette> palette: {
            gradient: @linear-gradient(135deg, #80BCBD 0%, #AAD9BB 100%),
        };
    }

    component Tile inherits Rectangle {
        background: Theme.palette.gradient;
        Text {
            text: "hello world";
            color: #F9F7C9;
        }
    }

    export component MainWindow inherits Window {
        width: 400px;
        height: 400px;
        Tile {}
    }
}