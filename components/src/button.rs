use crate::{
  Color,
  Size,
};
use leptos::*;
use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonColor {
  Color(Color),
  Glass,
}

impl From<Color> for ButtonColor {
  fn from(color: Color) -> Self {
    Self::Color(color)
  }
}

impl ButtonColor {
  fn into_btn_color_class(self) -> &'static str {
    match self {
      Self::Color(Color::Primary) => {
        const_format::concatcp!["btn-", Color::PRIMARY]
      }
      Self::Color(Color::Secondary) => {
        const_format::concatcp!["btn-", Color::SECONDARY]
      }
      Self::Color(Color::Accent) => {
        const_format::concatcp!["btn-", Color::ACCENT]
      }
      Self::Color(Color::Info) => const_format::concatcp!["btn-", Color::INFO],
      Self::Color(Color::Success) => {
        const_format::concatcp!["btn-", Color::SUCCESS]
      }
      Self::Color(Color::Warning) => {
        const_format::concatcp!["btn-", Color::WARNING]
      }
      Self::Color(Color::Error) => {
        const_format::concatcp!["btn-", Color::ERROR]
      }
      Self::Glass => "glass",
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonWidth {
  Wide,
  Block,
}

impl ButtonWidth {
  fn into_btn_width_class(self) -> &'static str {
    match self {
      ButtonWidth::Wide => "btn-wide",
      ButtonWidth::Block => "btn-block",
    }
  }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ButtonShape {
  Square,
  Circle,
}

impl ButtonShape {
  fn into_btn_shape_class(self) -> &'static str {
    match self {
      ButtonShape::Square => "btn-square",
      ButtonShape::Circle => "btn-circle",
    }
  }
}

#[derive(Clone, Default, Debug, Hash, PartialEq, Eq)]
pub enum ButtonKind {
  #[default]
  Button,
  Link(Cow<'static, str>),
}

impl Size {
  fn into_btn_size_class(self) -> &'static str {
    match self {
      Size::ExtraSmall => const_format::concatcp!["btn-", Size::EXTRASMALL],
      Size::Small => const_format::concatcp!["btn-", Size::SMALL],
      Size::Medium => const_format::concatcp!["btn-", Size::MEDIUM],
      Size::Large => const_format::concatcp!["btn-", Size::LARGE],
    }
  }
}

#[component]
pub fn Button(
  cx: Scope,
  #[prop(optional, into)] kind: ButtonKind,
  #[prop(optional, into)] color: Option<MaybeSignal<Option<ButtonColor>>>,
  #[prop(optional, into)] size: Option<MaybeSignal<Option<Size>>>,
  #[prop(optional, into)] outlined: MaybeSignal<bool>,
  #[prop(optional, into)] active: MaybeSignal<bool>,
  #[prop(optional, into)] disabled: MaybeSignal<bool>,
  #[prop(optional, into)] loading: MaybeSignal<bool>,
  #[prop(optional, into)] no_animation: MaybeSignal<bool>,
  #[prop(optional, into)] width: Option<MaybeSignal<Option<ButtonWidth>>>,
  #[prop(optional, into)] shape: Option<MaybeSignal<Option<ButtonShape>>>,
  #[prop(optional, into)] button_ref: Option<NodeRef<html::AnyElement>>,
  #[prop(optional, into)] on_click: Option<SignalSetter<()>>,
  #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
  let btn_ref_local = create_node_ref(cx);

  if let Some(on_click) = on_click {
    btn_ref_local.on_load(cx, move |btn: HtmlElement<html::AnyElement>| {
      btn.on(ev::click, move |_| on_click(()));
    });
  }

  if let Some(ref_) = button_ref {
    btn_ref_local.on_load(cx, move |btn| {
      ref_.load(&btn);
    });
  }

  if let Some(color) = color {
    btn_ref_local.on_load(cx, move |btn| {
      btn.dyn_classes(move || color().map(|c| c.into_btn_color_class()));
    });
  }

  if let Some(size) = size {
    btn_ref_local.on_load(cx, move |btn| {
      btn.dyn_classes(move || size().map(|s| s.into_btn_size_class()));
    });
  }

  if let Some(width) = width {
    btn_ref_local.on_load(cx, move |btn| {
      btn.dyn_classes(move || width().map(|w| w.into_btn_width_class()));
    });
  }

  if let Some(shape) = shape {
    btn_ref_local.on_load(cx, move |btn| {
      btn.dyn_classes(move || shape().map(|w| w.into_btn_shape_class()));
    });
  }

  let btn = match kind {
    ButtonKind::Button => html::button(cx).into_any(),
    ButtonKind::Link(href) => html::a(cx).attr(href, true).into_any(),
  };

  btn
    .node_ref(btn_ref_local)
    .class("btn", true)
    .class("btn-outline", outlined)
    .class("loading", loading)
    .class("btn-active", active)
    .class("no-animation", no_animation)
    .class("btn-disabled", disabled)
    .attr("disabled", disabled)
    .child(children.map(|child| child(cx)))
}
