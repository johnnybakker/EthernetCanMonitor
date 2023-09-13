mod event;

use std::any::TypeId;

pub use event::*;
use iced::{widget::{PaneGrid, pane_grid::{self}, Container}, Application, Length, Element};
use iced::executor;
use iced::theme::Theme;
use iced::Command;

use crate::{can::{self, CanUdpSocket}, widget::{WidgetHandle, TableWidget, GraphWidget}};

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
        let (panes, _) = pane_grid::State::new(WidgetHandle::new(0, GraphWidget::default()));

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
						WidgetHandle::new(self.panes_created, GraphWidget::default()),
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
							WidgetHandle::new(self.panes_created, GraphWidget::default()),
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

        let pane_grid = PaneGrid::<EventBox, _>::new(&self.panes, |pane, handle, is_maximized| {
			handle.view(pane, Some(pane) == focus, total_panes, is_maximized).into()
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

