use gui::{Screen, Button, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Big Sad"),
            }),
        ],
    };

    // Even with the dynamic dispatch, we still have compile time check. For instance, this wouldn't compile:
    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hello world!"))],
    // };

    // "the trait 'Draw' is not implemented for 'String'". Yet again, pretty explicit

    screen.render();
}