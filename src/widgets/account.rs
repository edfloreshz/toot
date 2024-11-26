use capitalize::Capitalize;
use cosmic::{
    iced::{alignment::Horizontal, Length},
    widget::{self, image::Handle},
    Apply, Element, Task,
};
use mastodon_async::prelude::Account;
use std::collections::HashMap;

use crate::app;

#[derive(Debug, Clone)]
pub enum Message {
    Open(String),
}

pub fn account<'a>(
    account: &'a Account,
    handles: &'a HashMap<String, Handle>,
) -> Element<'a, Message> {
    let spacing = cosmic::theme::active().cosmic().spacing;

    let header = handles
        .get(&account.header)
        .map(|handle| widget::image(handle));
    let avatar = handles
        .get(&account.avatar)
        .map(|handle| widget::image(handle).width(100));
    let display_name = widget::text(&account.display_name).size(18);
    let username = widget::button::link(format!("@{}", account.username))
        .on_press(Message::Open(account.url.clone()));
    let bio = (!account.note.is_empty()).then_some(widget::text(
        html2text::config::rich()
            .string_from_read(account.note.as_bytes(), 700)
            .unwrap(),
    ));
    let joined = widget::text::caption(format!(
        "Joined on {}",
        account
            .created_at
            .format(&time::format_description::parse("[day] [month repr:short] [year]").unwrap())
            .unwrap()
    ));
    let fields: Option<Vec<Element<_>>> = account.fields.as_ref().map(|fields| {
        fields
            .iter()
            .map(|field| {
                let value = html2text::config::rich()
                    .string_from_read(field.value.as_bytes(), 700)
                    .unwrap();
                widget::column()
                    .push(widget::text(field.name.capitalize()))
                    .push(widget::text(value.clone()).class(cosmic::style::Text::Accent))
                    .width(Length::Fill)
                    .apply(widget::button::custom)
                    .class(cosmic::style::Button::Icon)
                    .on_press(Message::Open(value))
                    .into()
            })
            .collect()
    });
    let followers = widget::column()
        .push(widget::text::text("Followers"))
        .push(widget::text::title3(account.followers_count.to_string()))
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Center);
    let following = widget::column()
        .push(widget::text::text("Following"))
        .push(widget::text::title3(account.following_count.to_string()))
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Center);
    let statuses = widget::column()
        .push(widget::text::text("Posts"))
        .push(widget::text::title3(account.statuses_count.to_string()))
        .width(Length::FillPortion(1))
        .align_x(Horizontal::Center);

    let info = widget::container(
        widget::row()
            .push(followers)
            .push(widget::divider::vertical::light().height(Length::Fixed(50.)))
            .push(following)
            .push(widget::divider::vertical::light().height(Length::Fixed(50.)))
            .push(statuses)
            .padding(spacing.space_xs)
            .spacing(spacing.space_xs),
    )
    .class(cosmic::style::Container::Card);

    let content = widget::column()
        .push_maybe(header)
        .push_maybe(avatar)
        .push(display_name)
        .push(username)
        .push_maybe(bio)
        .push(joined)
        .push(info)
        .push_maybe(fields.map(|fields| widget::settings::section().extend(fields)))
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .spacing(spacing.space_xs);

    widget::settings::flex_item_row(vec![content.into()])
        .padding(spacing.space_xs)
        .into()
}

pub fn update(message: Message) -> Task<app::Message> {
    let tasks = vec![];
    match message {
        Message::Open(url) => {
            if let Err(err) = open::that_detached(url) {
                tracing::error!("{err}");
            }
        }
    }
    Task::batch(tasks)
}