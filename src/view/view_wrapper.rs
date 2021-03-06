use Printer;

use direction::Direction;
use event::{Event, EventResult};
use std::any::Any;
use vec::Vec2;
use view::{Selector, View};

/// Generic wrapper around a view.
///
/// Default implementation forwards all calls to the child view.
/// Overrides some methods as desired.
///
/// You can use the [`wrap_impl!`] macro to define `with_view` and
/// `with_view_mut` for you.
///
/// [`wrap_impl!`]: ../macro.wrap_impl.html
pub trait ViewWrapper {
    /// Type that this view wraps.
    type V: View;

    /// Get an immutable reference to the wrapped view.
    fn with_view<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&Self::V) -> R;

    /// Get a mutable reference to the wrapped view.
    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
        where F: FnOnce(&mut Self::V) -> R;

    /// Wraps the `draw` method.
    fn wrap_draw(&self, printer: &Printer) {
        self.with_view(|v| v.draw(printer));
    }

    /// Wraps the `required_size` method.
    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        self.with_view_mut(|v| v.required_size(req)).unwrap_or_else(Vec2::zero)
    }

    /// Wraps the `on_event` method.
    fn wrap_on_event(&mut self, ch: Event) -> EventResult {
        self.with_view_mut(|v| v.on_event(ch)).unwrap_or(EventResult::Ignored)
    }

    /// Wraps the `layout` method.
    fn wrap_layout(&mut self, size: Vec2) {
        self.with_view_mut(|v| v.layout(size));
    }

    /// Wraps the `take_focus` method.
    fn wrap_take_focus(&mut self, source: Direction) -> bool {
        self.with_view_mut(|v| v.take_focus(source)).unwrap_or(false)
    }

    /// Wraps the `find` method.
    fn wrap_call_on_any<'a>(&mut self, selector: &Selector,
                         callback: Box<FnMut(&mut Any) + 'a>) {
        self.with_view_mut(|v| v.call_on_any(selector, callback));
    }

    /// Wraps the `focus_view` method.
    fn wrap_focus_view(&mut self, selector: &Selector) -> Result<(), ()> {
        self.with_view_mut(|v| v.focus_view(selector)).unwrap_or(Err(()))
    }

    /// Wraps the `needs_relayout` method.
    fn wrap_needs_relayout(&self) -> bool {
        self.with_view(|v| v.needs_relayout()).unwrap_or(true)
    }
}

impl<T: ViewWrapper> View for T {
    fn draw(&self, printer: &Printer) {
        self.wrap_draw(printer);
    }

    fn required_size(&mut self, req: Vec2) -> Vec2 {
        self.wrap_required_size(req)
    }

    fn on_event(&mut self, ch: Event) -> EventResult {
        self.wrap_on_event(ch)
    }

    fn layout(&mut self, size: Vec2) {
        self.wrap_layout(size);
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.wrap_take_focus(source)
    }

    fn call_on_any<'a>(&mut self, selector: &Selector,
                    callback: Box<FnMut(&mut Any) + 'a>) {
        self.wrap_call_on_any(selector, callback)
    }

    fn needs_relayout(&self) -> bool {
        self.wrap_needs_relayout()
    }

    fn focus_view(&mut self, selector: &Selector) -> Result<(), ()> {
        self.wrap_focus_view(selector)
    }
}

/// Convenient macro to implement the [`ViewWrapper`] trait.
///
/// It defines the `with_view` and `with_view_mut` implementations,
/// as well as the `type V` declaration.
///
/// [`ViewWrapper`]: view/trait.ViewWrapper.html
///
/// # Examples
///
/// ```no_run
/// # #[macro_use] extern crate cursive;
/// # use cursive::view::{View,ViewWrapper};
/// struct FooView<T: View> {
///     view: T,
/// }
///
/// impl <T: View> ViewWrapper for FooView<T> {
///     wrap_impl!(self.view: T);
/// }
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! wrap_impl {
    (self.$v:ident: $t:ty) => {
        type V = $t;

        fn with_view<F, R>(&self, f: F) -> Option<R>
            where F: FnOnce(&Self::V) -> R
        {
            Some(f(&self.$v))
        }

        fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
            where F: FnOnce(&mut Self::V) -> R
        {
            Some(f(&mut self.$v))
        }
    };
}
