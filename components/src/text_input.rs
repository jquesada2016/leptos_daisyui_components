use crate::{
  Color,
  Size,
};
use leptos::*;

impl Color {
  const fn into_text_input_class(self) -> &'static str {
    match self {
      Self::Primary => const_format::concatcp!["input-", Color::PRIMARY],
      Self::Secondary => const_format::concatcp!["input-", Color::SECONDARY],
      Self::Accent => const_format::concatcp!["input-", Color::ACCENT],
      Self::Info => const_format::concatcp!["input-", Color::INFO],
      Self::Success => const_format::concatcp!["input-", Color::SUCCESS],
      Self::Warning => const_format::concatcp!["input-", Color::WARNING],
      Self::Error => const_format::concatcp!["input-", Color::ERROR],
    }
  }
}

impl Size {
  const fn into_text_input_size(self) -> &'static str {
    match self {
      Size::Large => const_format::concatcp!["input-", "lg"],
      Size::Medium => const_format::concatcp!["input-", "md"],
      Size::Small => const_format::concatcp!["input-", "sm"],
      Size::ExtraSmall => const_format::concatcp!["input-", "xs"],
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextInputType {
  Text,
  Number,
  Color,
  Date,
  DateTimeLocal,
  Email,
  Month,
  Password,
  Search,
  Tel,
  Time,
  Url,
  Week,
}

impl TextInputType {
  fn into_type_str(self) -> &'static str {
    match self {
      Self::Text => "text",
      Self::Number => "number",
      Self::Color => "color",
      Self::Date => "date",
      Self::DateTimeLocal => "datetime-local",
      Self::Email => "email",
      Self::Month => "month",
      Self::Password => "password",
      Self::Search => "search",
      Self::Tel => "tel",
      Self::Time => "time",
      Self::Url => "url",
      Self::Week => "week",
    }
  }
}

#[component]
pub fn TextInput(
  cx: Scope,
  #[prop(optional, into)] id: Option<String>,
  #[prop(optional, into)] type_: Option<TextInputType>,
  #[prop(optional, into)] name: Option<String>,
  #[prop(optional, into)] label: Option<MaybeSignal<String>>,
  #[prop(optional, into)] label_alt: Option<MaybeSignal<String>>,
  #[prop(optional, into)] helper: Option<MaybeSignal<String>>,
  #[prop(optional, into)] helper_alt: Option<MaybeSignal<String>>,
  #[prop(optional, into)] placeholder: Option<MaybeSignal<String>>,
  #[prop(optional, into)] value: Option<MaybeSignal<String>>,
  #[prop(optional, into)] focus: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] bordered: MaybeSignal<bool>,
  #[prop(optional, into)] color: Option<MaybeSignal<Option<Color>>>,
  #[prop(optional, into)] size: Option<MaybeSignal<Option<Size>>>,
  #[prop(optional, into)] ghost: MaybeSignal<bool>,
  #[prop(optional, into)] input_ref: Option<NodeRef<html::Input>>,
  #[prop(optional)] on_value: Option<SignalSetter<String>>,
) -> impl IntoView {
  let input_ref_local = create_node_ref::<html::Input>(cx);

  let id = id.unwrap_or_else(|| rand::random::<usize>().to_string());

  if let Some(input_ref) = input_ref {
    input_ref_local.on_load(cx, move |input| {
      input_ref.load(&input);
    });
  }

  if let Some(focus) = focus {
    input_ref_local.on_load(cx, move |input| {
      create_effect(cx, move |later_run| {
        if focus() {
          if later_run.is_none() {
            input.clone().on_mount(|input| {
              let _ = input.focus();
            });
          }

          let _ = input.focus();
        }
      })
    });
  }

  if let Some(color) = color {
    input_ref_local.on_load(cx, move |input| {
      input.dyn_classes(move || color().map(|c| c.into_text_input_class()));
    });
  }

  if let Some(size) = size {
    input_ref_local.on_load(cx, move |input| {
      input.dyn_classes(move || size().map(|s| s.into_text_input_size()));
    });
  }

  if let Some(on_value) = on_value {
    input_ref_local.on_load(cx, move |input| {
      input.on(ev::input, move |e| {
        let value = event_target_value(&e);

        on_value(value);
      });
    });
  }

  let label = {
    let id = id.clone();

    (label.is_some() || label_alt.is_some()).then(|| {
      let label = label.map(|label| {
        view! { cx,
          <span class="label-text">{label}</span>
        }
      });
      let label_alt = label_alt.map(|label_alt| {
        view! { cx,
          <span class="label-text-alt">{label_alt}</span>
        }
      });

      view! { cx,
        <label class="label" for=id>
          {label}
          {label_alt}
        </label>
      }
    })
  };

  let helper = {
    let id = id.clone();

    (helper.is_some() || helper_alt.is_some()).then(|| {
      let helper = helper.map(|helper| {
        view! { cx,
          <span class="label-text">{helper}</span>
        }
      });
      let helper_alt = helper_alt.map(|helper_alt| {
        view! { cx,
          <span class="label-text-alt">{helper_alt}</span>
        }
      });

      view! { cx,
        <label class="label" for=id>
          {helper}
          {helper_alt}
        </label>
      }
    })
  };

  view! { cx,
    <div class="form-control w-full max-w-xs">
      {label}
      <input
        ref=input_ref_local
        id=id
        name=name
        type=type_.map(|type_| type_.into_type_str())
        class="input input-bordered w-full max-w-xs"
        class:input-bordered=bordered
        class:input-ghost=ghost
        placeholder=placeholder.map(|placeholder| placeholder())
        prop:value=value.unwrap_or_default()
      />
      {helper}
    </div>
  }
}
