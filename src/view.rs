use iced::widget::{button, column, text, radio, row, Column, Button, Scrollable};
use iced::Length;
use crate::model::{Model, DistanceSelection, DefenceSelection, FaintSelection, BodySelection};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Reset,
    Next,
    Previous,
    Reload,
    DistanceSelected(DistanceSelection),
    DefenceSelected(DefenceSelection),
    FaintSelected(FaintSelection),
    BodySelected(BodySelection),
    ItemSelected(usize),
}

pub fn view(model: &Model) -> Column<Message> {
    let mut column: Column<Message> = Column::new();
    for (index, item) in model.combinations().iter().enumerate() {
        let button: Button<Message> = button(item.description.as_str())
            .on_press(Message::ItemSelected(index))
            .width(Length::Fill);
        column = column.push(button);
    }
    let scrollable: Scrollable<Message> = Scrollable::new(column).into();

    let button_row_spacing = 5;
    let row_spacing = 20;
    let radio_label_length = 65;
    let radio_length = 65;
    column![
        text(model.number())
            .size(70)
            .width(Length::Fill),
        text(model.combination())
            .size(100)
            .width(Length::Fill),
        text("")
            .size(10)
            .width(Length::Fill),    
        row![    
            button("Next")
                .on_press(Message::Next)
                .width(Length::Fill),
            button("Previous")
                .on_press(Message::Previous)
                .width(Length::Fill),    
            button("Reset")
                .on_press(Message::Reset)
                .width(Length::Fill),
            button("Reload")
                .on_press(Message::Reload)
                .width(Length::Fill),  
        ].spacing(button_row_spacing),       
        row![
            text("Distance:").width(radio_label_length),
            radio("All", DistanceSelection::All, model.distance_selection(), Message::DistanceSelected)
                .width(radio_length),
            radio("Long", DistanceSelection::Long, model.distance_selection(), Message::DistanceSelected)
                .width(radio_length),
            radio("Short", DistanceSelection::Short, model.distance_selection(), Message::DistanceSelected)
                .width(radio_length),
        ].spacing(row_spacing),      
        row![ 
            text("Defence:").width(radio_label_length),
            radio("All", DefenceSelection::All, model.defence_selection(), Message::DefenceSelected)
                .width(radio_length),
            radio("Yes", DefenceSelection::Yes, model.defence_selection(), Message::DefenceSelected)
                .width(radio_length),
            radio("No", DefenceSelection::No, model.defence_selection(), Message::DefenceSelected)
                .width(radio_length),
        ].spacing(row_spacing),         
        row![ 
            text("Faint:").width(radio_label_length),
            radio("All", FaintSelection::All, model.faint_selection(), Message::FaintSelected)
                .width(radio_length),
            radio("Yes", FaintSelection::Yes, model.faint_selection(), Message::FaintSelected)
                .width(radio_length),
            radio("No", FaintSelection::No, model.faint_selection(), Message::FaintSelected)
                .width(radio_length),  
        ].spacing(row_spacing),  
        row![ 
            text("Body:").width(radio_label_length),    
            radio("All", BodySelection::All, model.body_selection(), Message::BodySelected)
                .width(radio_length),
            radio("Yes", BodySelection::Yes, model.body_selection(), Message::BodySelected)
                .width(radio_length),
            radio("No", BodySelection::No, model.body_selection(), Message::BodySelected)
                .width(radio_length),   
        ].spacing(row_spacing), 
         scrollable,
    ]
    .into()
}

pub fn update(model: &mut Model, message: Message) {
    match message {
        Message::Next => {
            model.next();
        }
        Message::Previous => {
            model.previous();
        }
        Message::Reset => {
            model.reset();
        }
        Message::Reload => {
            model.reload();
        }
        Message::DistanceSelected(option) => {
            model.set_distance_selection(option);
        }
        Message::DefenceSelected(option) => {
            model.set_defence_selection(option);
        }
        Message::FaintSelected(option) => {
            model.set_faint_selection(option);
        }
        Message::BodySelected(option) => {
            model.set_body_selection(option);
        }
        Message::ItemSelected(index) => {
            model.set(index);
        }
    }
}

