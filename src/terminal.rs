use iced::widget::{button, scrollable, text, text_input, Scrollable};
use iced::widget::{column, container};
use iced::{Alignment, Element, Length};

use crate::Message;

#[derive(Debug, Clone)]
pub enum TerminalMessage {
    InputChanged(String),
    Submit,
}

#[derive(Default)]
pub struct Terminal {
    pub input_value: String,
    pub display_value: String,
    line_num: u32,
}

impl Terminal {
    /// push data to display_value
    /// if line_num is less than 10, add data to display_value
    /// if line_num is greater than 10, remove all characters up to the first newline
    fn reg_data(&mut self, new_data: &String) {
        if self.line_num < 5 {
            self.display_value.push_str(new_data);
            self.display_value.push('\n');
            self.line_num += 1;
        } else {
            let mut first_newline = self.display_value.chars().position(|c| c == '\n').unwrap();
            first_newline += 1;
            self.display_value = self.display_value[first_newline..].to_string();
            self.display_value.push_str(new_data);
            self.display_value.push('\n');
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let input_row = text_input(">", &self.input_value)
            .on_input(|value| Message::TerminalMessage(TerminalMessage::InputChanged(value)))
            .on_submit(Message::TerminalMessage(TerminalMessage::Submit))
            .width(Length::Fill)
            .line_height(2.0)
            .align_x(Alignment::Start);

        let scroll: Scrollable<'_, Message, iced::Theme, iced::Renderer> = scrollable(
            column![text(&self.display_value)]
                .width(Length::Fill)
                .align_x(Alignment::Start),
        )
        .height(Length::Fill)
        .direction(scrollable::Direction::Vertical(
            scrollable::Scrollbar::default().width(5).scroller_width(5),
        ));

        let input_row_w_button = button(container(text("btn")))
            .on_press(Message::TerminalMessage(TerminalMessage::Submit));

        container(column![scroll, input_row, input_row_w_button]).into()
    }

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::InputChanged(value) => {
                self.input_value = value;
            }
            TerminalMessage::Submit => {
                self.reg_data(&(self.input_value.clone()));
                self.input_value.clear();
            }
        }
    }
}
