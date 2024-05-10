use std::{cell::RefCell, rc::Rc, any::TypeId};
use iced::{Element, Theme, widget::{Row, button, text, pane_grid::{self, Pane, Content}, scrollable, container, row, column, Column}, Color, Length, alignment, theme, Alignment};
use crate::{Event, EventBox, AppMessage};
use super::Widget;


pub struct WidgetHandle {  
	pub id: usize,
	pub is_pinned: bool,
	widget: Rc<RefCell<dyn Widget>>,
	subscriptions: Vec<(TypeId, Box<dyn Fn(&mut dyn Widget, &dyn Event)>)>
}

impl WidgetHandle {

	pub fn new(id: usize, widget: impl Widget + 'static) -> Self {
	
		let widget = Rc::new(RefCell::new(widget));

		let mut handle = Self {
			id: id,
			is_pinned: true,
			widget: widget.clone(),
			subscriptions: Vec::default()
		};

		widget.borrow_mut().setup(&mut handle);
		handle
	}

	fn get_mut_widget(&self) -> &mut dyn Widget {
		unsafe { &mut *self.widget.as_ptr() }
	}

	pub fn update(&mut self, type_id: TypeId, e: &dyn Event) -> Option<iced::Command<EventBox>> {
		
		for subscription in self.subscriptions.iter() {
			if subscription.0 == type_id {
				subscription.1(self.get_mut_widget(), e);
			}
		}

		None
	}

	pub fn view(&self, pane: Pane, focus: bool, total_panes: usize, is_maximized: bool) -> Content<'static, EventBox, iced::Renderer<Theme>> {


		let pin_button = button(
			text(if self.is_pinned { "Unpin" } else { "Pin" }).size(14),
		)
		.on_press(AppMessage::TogglePin(pane).into())
		.padding(3);

		let title = row![
			pin_button,
			text("Messages").style(if focus {
				PANE_ID_COLOR_FOCUSED
			} else {
				PANE_ID_COLOR_UNFOCUSED
			}),
		]
		.spacing(5);

		let title_bar = pane_grid::TitleBar::new(title)
			.controls(view_controls(
				pane,
				total_panes,
				self.is_pinned,
				is_maximized,
			))
			.padding(10)
			.style(if focus {
				style::title_bar_focused
			} else {
				style::title_bar_active
			}).into();

		pane_grid::Content::new(
			view_content(pane, &self, total_panes)
		)
		.title_bar(title_bar)
		.style(
			if focus {
				style::pane_focused
			} else {
				style::pane_active
			}
		)
	

	}
	
	pub fn subscribe<W: Widget + 'static, T: Event + 'static>(&mut self, handler: fn(&mut W, &T)) {

		let type_id = TypeId::of::<T>();
		
		let handler = move |w: &mut dyn Widget, e: &dyn Event| unsafe { 
			handler(
				&mut *(w as *mut dyn Widget as *mut W),
				&*(e as *const dyn Event as *const T)
			) 
		};

		self.subscriptions.push((type_id, Box::new(handler)));
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

    let mut controls = Column::new()
    .spacing(5)
    .width(Length::Fill);

	if !handle.is_pinned {
    
		controls = controls.push(
			button(
				"Split horizontally",
				AppMessage::Split(pane_grid::Axis::Horizontal, pane).into(),
			)
		).push(
			button(
				"Split vertically",
				AppMessage::Split(pane_grid::Axis::Vertical, pane).into(),
			)
		);

		if total_panes > 1 {
			controls = controls.push(
				button("Close", AppMessage::Close(pane).into())
					.style(theme::Button::Destructive),
			);
		} 
	} else {
		controls = controls.push(
			Row::new().push(handle.get_mut_widget().view())
		);
	}

    let content = column![
        controls,
    ]
    .width(Length::Fill)
    .spacing(10)
    .align_items(Alignment::Center);

    container(content)
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
    use iced::{Theme, Color};

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
            background: Some(Color::WHITE.into()),
            border_width: 2.0,
            border_color: palette.background.strong.color,
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Appearance {
        let palette = theme.extended_palette();

        container::Appearance {
            background: Some(Color::WHITE.into()),
            border_width: 2.0,
            border_color: palette.primary.strong.color,
            ..Default::default()
        }
    }
}
