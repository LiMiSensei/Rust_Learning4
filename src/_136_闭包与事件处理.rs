use std::arch::x86_64::_bextr_u32;

enum ButtonData {
    Count(i32),
    Message(String),
}

struct Button<F>
where
    F: Fn(&mut ButtonData),
{
    click_handler: F,
    button_data: ButtonData,
}

impl<F> Button<F>
where
    F: Fn(&mut ButtonData),
{
    fn new(click_handler: F, data: ButtonData) -> Button<F> {
        Button {
            click_handler,
            button_data: data,
        }
    }

    fn click(&mut self) {
        (self.click_handler)(&mut self.button_data);
    }

    fn set_message(& mut self, message: String) {
        self.button_data = ButtonData::Message(message);
    }
}
#[allow(dead_code)]
pub fn main() {
    let mut subscribe_btn = Button::new(
        |btn| {
            if let ButtonData::Count(sub_count) = btn {
                *sub_count += 1;
                println!("{}", sub_count)
            }
        },
        ButtonData::Count(0),
    );

    //let send_btn = Button::new();

    subscribe_btn.click();
    subscribe_btn.click();
    subscribe_btn.click();


    let mut send_btn = Button::new(
        |btn| {
            if let ButtonData::Message(msg) = btn {
                println!("msg:{}", msg)
            }
        },
        ButtonData::Message(String::new()),
    );

    send_btn.set_message("Hi".to_string());
    
    send_btn.click();
    send_btn.click();
    send_btn.click();
}
