mod event;

use std::any::TypeId;

pub use event::*;
use iced::{widget::{container, PaneGrid, pane_grid::{self}, Container}, Application, Length, Element, Color};

use iced::alignment::{self, Alignment};
use iced::executor;
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, row, scrollable, text,
};
use iced::Command;

use crate::{widget::{WidgetHandle, CanMessageWidget}, can::{self, CanUdpSocket}};

pub struct App {
    panes: pane_grid::State<WidgetHandle>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    TogglePin(pane_grid::Pane),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
}

impl Event for AppMessage {}

impl Application for App {
    type Message = EventBox;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<EventBox>) {
        let (panes, _) = pane_grid::State::new(WidgetHandle::new(0, CanMessageWidget::default()));

        (
            App {
                panes,
                panes_created: 1,
                focus: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pane grid - Iced")
    }

    fn update(&mut self, message: EventBox) -> Command<EventBox> {

		match message.unbox::<CanUdpSocket>() {
			Some(_) => {
				println!("Received sender");
			}
			None => {
				for widget in self.panes.iter_mut() {
					widget.1.update(message.type_id(), message.event());
				}
			}
		}


		if message.is::<AppMessage>() {
			let message = message.unbox_unchecked();

			match message {
				AppMessage::Split(axis, pane) => {
					let result = self.panes.split(
						*axis,
						&pane,
						WidgetHandle::new(self.panes_created, CanMessageWidget::default()),
					);

					if let Some((pane, _)) = result {
						self.focus = Some(pane);
					}

					self.panes_created += 1;
				}
				AppMessage::SplitFocused(axis) => {
					if let Some(pane) = self.focus {
						let result = self.panes.split(
							*axis,
							&pane,
							WidgetHandle::new(self.panes_created, CanMessageWidget::default()),
						);

						if let Some((pane, _)) = result {
							self.focus = Some(pane);
						}

						self.panes_created += 1;
					}
				}
				AppMessage::FocusAdjacent(direction) => {
					if let Some(pane) = self.focus {
						if let Some(adjacent) =
							self.panes.adjacent(&pane, *direction)
						{
							self.focus = Some(adjacent);
						}
					}
				}
				AppMessage::Clicked(pane) => {
					self.focus = Some(*pane);
				}
				AppMessage::Resized(pane_grid::ResizeEvent { split, ratio }) => {
					self.panes.resize(&split, *ratio);
				}
				AppMessage::Dragged(pane_grid::DragEvent::Dropped {
					pane,
					target,
				}) => {
					self.panes.drop(&pane, *target);
				}
				AppMessage::Dragged(_) => {}
				AppMessage::TogglePin(pane) => {
					if let Some(WidgetHandle { is_pinned, .. }) = self.panes.get_mut(&pane)
					{
						*is_pinned = !*is_pinned;
					}
				}
				AppMessage::Maximize(pane) => self.panes.maximize(&pane),
				AppMessage::Restore => {
					self.panes.restore();
				}
				AppMessage::Close(pane) => {
					if let Some((_, sibling)) = self.panes.close(&pane) {
						self.focus = Some(sibling);
					}
				}
				AppMessage::CloseFocused => {
					if let Some(pane) = self.focus {
						if let Some(WidgetHandle { is_pinned, .. }) = self.panes.get(&pane)
						{
							if !is_pinned {
								if let Some((_, sibling)) = self.panes.close(&pane)
								{
									self.focus = Some(sibling);
								}
							}
						}
					}
				}
			}

		}

      

        Command::none()
    }

	fn subscription(&self) -> iced::Subscription<Self::Message> {
		struct CanSocket;
		iced::subscription::channel(TypeId::of::<CanSocket>(), 0, can::udp_socket)
	}

    fn view(&self) -> Element<Self::Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::<EventBox, _>::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = focus == Some(id);

            let pin_button = button(
                text(if pane.is_pinned { "Unpin" } else { "Pin" }).size(14),
            )
            .on_press(AppMessage::TogglePin(id).into())
            .padding(3);

            let title = row![
                pin_button,
                "Pane",
                text(pane.id.to_string()).style(if is_focused {
                    PANE_ID_COLOR_FOCUSED
                } else {
                    PANE_ID_COLOR_UNFOCUSED
                }),
            ]
            .spacing(5);

            let title_bar = pane_grid::TitleBar::new(title)
                .controls(view_controls(
                    id,
                    total_panes,
                    pane.is_pinned,
                    is_maximized,
                ))
                .padding(10)
                .style(if is_focused {
                    style::title_bar_focused
                } else {
                    style::title_bar_active
                }).into();

			pane_grid::Content::new(
				view_content(id, pane, total_panes)
			)
			.title_bar(title_bar)
			.style(
				if is_focused {
					style::pane_focused
				} else {
				style::pane_active
				}
			)
			.into()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
		.on_click(|e|AppMessage::Clicked(e).into())
        .on_drag(|e|AppMessage::Dragged(e).into())
        .on_resize(10, |e|AppMessage::Resized(e).into());
	

		Container::new(pane_grid)
			.width(Length::Fill)
			.height(Length::Fill)
			.padding(10)
			.into()
    }
}

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

fn view_content<'a>(
    pane: pane_grid::Pane,
	handle: &WidgetHandle,
    total_panes: usize,
) -> Element<'a, EventBox> {
    let button = |label, message| {
        button(
            text(label)
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(16),
        )
        .width(Length::Fill)
        .padding(8)
        .on_press(message)
    };

    let mut controls = column![
        button(
            "Split horizontally",
            AppMessage::Split(pane_grid::Axis::Horizontal, pane).into(),
        ),
        button(
            "Split vertically",
            AppMessage::Split(pane_grid::Axis::Vertical, pane).into(),
        )
    ]
    .spacing(5)
    .width(Length::Fill);

    if total_panes > 1 && !handle.is_pinned {
        controls = controls.push(
            button("Close", AppMessage::Close(pane).into())
                .style(theme::Button::Destructive),
        );
    } else {
		controls = controls.push(
			handle.view()
		)
	}

    let content = column![
        controls,
    ]
    .width(Length::Fill)
    .spacing(10)
    .align_items(Alignment::Center);

    container(scrollable(content).width(Length::Fill))
        .width(Length::Fill)
        .padding(5)
        .into()
}

fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, EventBox> {
    let mut row = row![].spacing(5);

    if total_panes > 1 {
        let toggle = {
            let (content, message) = if is_maximized {
                ("Restore", AppMessage::Restore.into())
            } else {
                ("Maximize", AppMessage::Maximize(pane).into())
            };
            button(text(content).size(14))
                .style(theme::Button::Secondary)
                .padding(3)
                .on_press(message)
        };

        row = row.push(toggle);
    }

    let mut close = button(text("Close").size(14))
        .style(theme::Button::Destructive)
        .padding(3);

    if total_panes > 1 && !is_pinned {
        close = close.on_press(AppMessage::Close(pane).into());
    }

    row.push(close).into()
}

mod style {
    use iced::widget::container;
    use iced::Theme;

    pub fn title_bar_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border_width: 2.0,
            border_color: palette.background.strong.color,
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(palette.background.weak.color.into()),
            border_width: 2.0,
            border_color: palette.primary.strong.color,
            ..Default::default()
        }
    }
}

// pub struct App {
// 	panes: pane_grid::State<WidgetHandle>,
// 	widgets: Vec<WidgetHandle>,
// }

// impl App {

// 	pub fn add_widget(&mut self, widget: impl Widget + 'static) {
// 		self.widgets.push(WidgetHandle::new(widget));
// 	}

// }

// impl Application for App {
    
// 	type Executor = iced::executor::Default;
//     type Message = EventBox;
//     type Theme = iced::Theme;
//     type Flags = ();

//     fn new(_: Self::Flags) -> (Self, iced::Command<Self::Message>) {

// 		let (state, pane) = pane_grid::State::new(
// 			WidgetHandle::new(HeartbeatWidget::default())
// 		);

// 		let mut app = Self {
// 			panes: state, 
// 			widgets: Default::default() 
// 		};
	
// 		app.panes.split(pane_grid::Axis::Horizontal, &pane, WidgetHandle::new(HeartbeatWidget::default()));

// 		(app, iced::Command::none())
//     }

//     fn title(&self) -> String {
//         "CanMonitor".to_owned()
//     }

// 	fn subscription(&self) -> iced::Subscription<Self::Message> {
// 		struct CanSocket;
// 		iced::subscription::channel(TypeId::of::<CanSocket>(), 0, can::udp_socket)
// 	}

//     fn update(&mut self, e: Self::Message) -> iced::Command<Self::Message> {

// 		match e.unbox::<CanUdpSocket>() {
// 			Some(_) => {
// 				println!("Received sender");
// 			}
// 			None => {
// 				for widget in self.widgets.iter_mut() {
// 					widget.update(e.type_id(), e.event());
// 				}
// 			}
// 		}

// 		iced::Command::none()
// 	}

//     fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {

// 		let widgets = self.widgets.iter()
// 			.map(|w|w.view())
// 			.collect();

// 		let body: Element<_,_> = Row::with_children(widgets)
// 			.width(Length::Fill)
// 			.height(Length::Fill)
// 			.into();

// 		let grid = PaneGrid::new(&self.panes, |id, pane, is_maximized|{
// 			Text::new("Pane").into()
// 		});

// 		container(grid)
// 			.width(Length::Fill)
// 			.height(Length::Fill)
// 			.padding(10)
// 			.style(|theme: &iced::Theme|container::Appearance {
// 				background: Some(
// 					iced::Background::Color(
// 						Color::from_rgb(255.0, 0.0, 0.0)
// 					)
// 				),
// 				..Default::default()
// 			})
// 			.into()
//     }
// }
