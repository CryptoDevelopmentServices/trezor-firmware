use crate::ui::{
    component::{text::paragraphs::Paragraphs, Child, Component, Event, EventCtx, Image, Never},
    geometry::{Insets, LinearPlacement, Rect},
};

use super::theme;

pub enum DialogMsg<T, U> {
    Content(T),
    Controls(U),
}

pub struct Dialog<T, U> {
    content: Child<T>,
    controls: Child<U>,
}

impl<T, U> Dialog<T, U>
where
    T: Component,
    U: Component,
{
    pub fn new(content: T, controls: U) -> Self {
        Self {
            content: Child::new(content),
            controls: Child::new(controls),
        }
    }

    pub fn inner(&self) -> &T {
        self.content.inner()
    }
}

impl<T, U> Component for Dialog<T, U>
where
    T: Component,
    U: Component,
{
    type Msg = DialogMsg<T::Msg, U::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        let controls_area = self.controls.place(bounds);
        let content_area = bounds
            .inset(Insets::bottom(controls_area.height()))
            .inset(Insets::bottom(theme::BUTTON_SPACING))
            .inset(Insets::left(theme::CONTENT_BORDER));
        self.content.place(content_area);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.content
            .event(ctx, event)
            .map(Self::Msg::Content)
            .or_else(|| self.controls.event(ctx, event).map(Self::Msg::Controls))
    }

    fn paint(&mut self) {
        self.content.paint();
        self.controls.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.content.bounds(sink);
        self.controls.bounds(sink);
    }
}

#[cfg(feature = "ui_debug")]
impl<T, U> crate::trace::Trace for Dialog<T, U>
where
    T: crate::trace::Trace,
    U: crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("Dialog");
        t.field("content", &self.content);
        t.field("controls", &self.controls);
        t.close();
    }
}

pub struct IconDialog<T, U> {
    image: Child<Image>,
    paragraphs: Paragraphs<T>,
    controls: Child<U>,
}

impl<T, U> IconDialog<T, U>
where
    T: AsRef<str>,
    U: Component,
{
    pub fn new(icon: &'static [u8], title: T, controls: U) -> Self {
        Self {
            image: Child::new(Image::new(icon)),
            paragraphs: Paragraphs::new()
                .with_placement(
                    LinearPlacement::vertical()
                        .align_at_center()
                        .with_spacing(Self::VALUE_SPACE),
                )
                .add(theme::TEXT_MEDIUM, title)
                .centered(),
            controls: Child::new(controls),
        }
    }

    pub fn with_description(mut self, description: T) -> Self {
        if !description.as_ref().is_empty() {
            self.paragraphs = self
                .paragraphs
                .add_color(theme::TEXT_NORMAL, theme::OFF_WHITE, description)
                .centered();
        }
        self
    }

    pub fn new_shares(lines: [T; 4], controls: U) -> Self {
        let [l0, l1, l2, l3] = lines;
        Self {
            image: Child::new(Image::new(theme::IMAGE_SUCCESS)),
            paragraphs: Paragraphs::new()
                .with_placement(LinearPlacement::vertical().align_at_center())
                .add_color(theme::TEXT_NORMAL, theme::OFF_WHITE, l0)
                .centered()
                .add(theme::TEXT_MEDIUM, l1)
                .centered()
                .add_color(theme::TEXT_NORMAL, theme::OFF_WHITE, l2)
                .centered()
                .add(theme::TEXT_MEDIUM, l3)
                .centered(),
            controls: Child::new(controls),
        }
    }

    pub const ICON_AREA_PADDING: i16 = 2;
    pub const ICON_AREA_HEIGHT: i16 = 60;
    pub const VALUE_SPACE: i16 = 5;
}

impl<T, U> Component for IconDialog<T, U>
where
    T: AsRef<str>,
    U: Component,
{
    type Msg = DialogMsg<Never, U::Msg>;

    fn place(&mut self, bounds: Rect) -> Rect {
        let bounds = bounds
            .inset(theme::borders())
            .inset(Insets::top(Self::ICON_AREA_PADDING));

        let controls_area = self.controls.place(bounds);
        let content_area = bounds.inset(Insets::bottom(controls_area.height()));

        let (image_area, content_area) = content_area.split_top(Self::ICON_AREA_HEIGHT);

        self.image.place(image_area);
        self.paragraphs.place(content_area);
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        self.paragraphs.event(ctx, event);
        self.controls.event(ctx, event).map(Self::Msg::Controls)
    }

    fn paint(&mut self) {
        self.image.paint();
        self.paragraphs.paint();
        self.controls.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.image.bounds(sink);
        self.paragraphs.bounds(sink);
        self.controls.bounds(sink);
    }
}

#[cfg(feature = "ui_debug")]
impl<T, U> crate::trace::Trace for IconDialog<T, U>
where
    T: AsRef<str>,
    U: crate::trace::Trace,
{
    fn trace(&self, t: &mut dyn crate::trace::Tracer) {
        t.open("IconDialog");
        t.field("content", &self.paragraphs);
        t.field("controls", &self.controls);
        t.close();
    }
}
