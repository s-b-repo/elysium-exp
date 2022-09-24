use super::Overlay;
use iced_native::widget::tree::Tree;
use iced_native::widget::{container, tree};
use iced_native::widget::{Container, Widget};
use iced_native::{event, layout, mouse, overlay, renderer};
use iced_native::{Clipboard, Event, Layout, Length, Point, Rectangle, Shell};
use std::cell::UnsafeCell;

/// The entire window.
///
/// Provides a means to overlay a menu over everything else.
pub struct Surface<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    pub ui: Container<'a, Message, Renderer>,
    pub overlay: UnsafeCell<Container<'a, Message, Renderer>>,
}

#[derive(Debug)]
pub struct SurfaceState {
    overlay: Tree,
}

impl SurfaceState {
    pub fn new() -> Self {
        let overlay = Tree::empty();

        Self { overlay }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Surface<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Renderer::Theme: container::StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        self.ui.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.ui.diff(tree)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        self.ui.draw(
            tree,
            renderer,
            theme,
            style,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn height(&self) -> Length {
        Widget::height(&self.ui)
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.ui.layout(renderer, limits)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.ui
            .mouse_interaction(tree, layout, cursor_position, viewport, renderer)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        self.ui.on_event(
            tree,
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            shell,
        )
    }

    fn overlay<'b>(
        &'b self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        let state = crate::State::get();

        if !state.menu_open.0 {
            return None;
        }

        let state = tree.state.downcast_mut::<SurfaceState>();
        let overlay: &'b mut Container<'b, Message, Renderer> =
            unsafe { &mut *self.overlay.get().cast() };

        state.overlay.diff(overlay as &dyn Widget<_, _>);

        Some(overlay::Element::new(
            layout.position(),
            Box::new(Overlay {
                overlay,
                state: &mut state.overlay,
            }),
        ))
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<SurfaceState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(SurfaceState::new())
    }

    fn width(&self) -> Length {
        Widget::width(&self.ui)
    }
}
