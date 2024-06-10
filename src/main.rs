use chrono::prelude::*;
use fltk::{app, button, frame, input, misc, prelude::*, window};
use fltk_calendar::calendar;
use fltk_flex::Flex;
use fltk_theme::{ThemeType, WidgetTheme};
use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

fn main() {
    let a = app::App::default().with_scheme(app::Scheme::Gtk);

    let mut win = window::Window::new(100, 100, 1024, 768, "GPU Reservation");
    let mut flex = Flex::default()
        .with_size(500, 200)
        .center_of_parent()
        .column();
    let mut start_date_btn = button::Button::new(160, 200, 80, 40, "Select Start Date");
    let mut start_date_label = frame::Frame::default().with_label("Start Date");
    let mut end_date_btn = button::Button::new(160, 200, 80, 40, "Select End Date");
    end_date_btn.deactivate();

    let mut end_date_label = frame::Frame::default().with_label("End Date");
    //let cal = calendar::Calendar::default();
    let mut user_names = misc::InputChoice::default();
    user_names.set_label("Please Select the name of the user");
    user_names.add("Ird");
    user_names.add("Oz");
    user_names.add("Ootake");
    user_names.add("Isuru");

    let mut flex_box_gpus = Flex::default().with_size(100, 100);
    let gpu0 = button::CheckButton::default().with_label("gpu 0");
    let gpu1 = button::CheckButton::default().with_label("gpu 1");
    let gpu2 = button::CheckButton::default().with_label("gpu 2");
    let gpu3 = button::CheckButton::default().with_label("gpu 3");
    flex_box_gpus.end();
    let theme = WidgetTheme::new(ThemeType::Aero);
    theme.apply();

    let mut btn = button::Button::default()
        .with_size(80, 30)
        .center_of_parent()
        .with_label("Book Appointment");

    win.end();
    flex.end();
    win.show();

    // define the behavior of the button
    let end_date_btn = Rc::new(RefCell::new(end_date_btn));
    let end_date_btn_clone = Rc::clone(&end_date_btn);
    let start_date_label = Rc::new(RefCell::new(start_date_label));
    let end_date_label = Rc::new(RefCell::new(end_date_label));
    let start_date_label_clone = Rc::clone(&start_date_label);
    let end_date_label_clone = Rc::clone(&end_date_label);

    start_date_btn.set_callback(move |_| {
        let cal = calendar::Calendar::default(); // Assuming calendar module provides this
        let date = cal.get_date();
        if let Some(date) = date {
            let date_str = format!("{}/{}/{}", date.year(), date.month(), date.day());
            println!("{:?}", date_str);
            start_date_label_clone.borrow_mut().set_label(&date_str);
            end_date_btn_clone.borrow_mut().activate();
        }
    });
    let start_date_label_clone = Rc::clone(&start_date_label);
    end_date_btn.borrow_mut().set_callback(move |_| {
        let cal = calendar::Calendar::default(); // Assuming calendar module provides this
        let date = cal.get_date();
        if let Some(date) = date {
            let date_str = format!("{}/{}/{}", date.year(), date.month(), date.day());
            println!("{:?}", date_str);
            let start_date = start_date_label_clone.borrow().label();
            match verify_dates(&start_date, &date_str) {
                Ok(valid) => {
                    if valid {
                        end_date_label_clone.borrow_mut().set_label(&date_str);
                    } else {
                        end_date_label_clone.borrow_mut().set_label(
                            "Invalid date selected. Please make sure end date is after start date.",
                        );
                    }
                }
                Err(e) => println!("Error verifying dates: {}", e),
            }
        }
    });
    btn.set_callback(move |b| {
        //label_box.set_label(&user_name.value());
        //dates_correct = verify_dates(&start_date, &end_date);
        //if !dates_correct{
        //  //handle the error
        //}
        win.set_label("Button Clicked")
    });
    a.run().unwrap();
}

fn verify_dates(start_date_str: &str, end_date_str: &str) -> Result<bool, Box<dyn Error>> {
    let start_date = NaiveDate::parse_from_str(start_date_str, "%Y/%m/%d")?;
    let end_date = NaiveDate::parse_from_str(end_date_str, "%Y/%m/%d")?;

    Ok(end_date >= start_date)
}

fn create_label(arg: &str) -> frame::Frame {
    let mut label_box = frame::Frame::default();
    label_box.set_label(arg);
    label_box
}
