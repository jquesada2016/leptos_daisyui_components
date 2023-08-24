use crate::{Color, Size};
use leptos::*;
use wasm_bindgen::JsCast;

impl Color {
  const fn into_text_input_class(self) -> &'static str {
    match self {
      Self::None => "",
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
      Self::None => "",
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
  #[prop(optional, into)] id: Option<String>,
  #[prop(optional, into)] type_: Option<TextInputType>,
  #[prop(optional, into)] name: Option<String>,
  #[prop(optional, into)] label: Option<MaybeSignal<String>>,
  #[prop(optional, into)] label_alt: Option<MaybeSignal<String>>,
  #[prop(optional, into)] helper: Option<MaybeSignal<String>>,
  #[prop(optional, into)] helper_alt: Option<MaybeSignal<String>>,
  #[prop(optional, into)] placeholder: Option<MaybeSignal<String>>,
  #[prop(optional, into)] value: Option<MaybeSignal<String>>,
  #[prop(optional, into)] value_as_date: Option<MaybeSignal<js_sys::Date>>,

  #[prop(optional, into)] focus: Option<MaybeSignal<bool>>,
  #[prop(optional, into)] bordered: MaybeSignal<bool>,
  #[prop(optional, into)] color: Option<MaybeSignal<Color>>,
  #[prop(optional, into)] size: Option<MaybeSignal<Size>>,
  #[prop(optional, into)] ghost: MaybeSignal<bool>,
  #[prop(optional, into)] input_ref: Option<NodeRef<html::Input>>,
  #[prop(optional)] on_value: Option<SignalSetter<String>>,
  #[prop(optional)] on_value_number: Option<SignalSetter<f64>>,
  #[prop(optional)] on_value_date: Option<SignalSetter<Option<js_sys::Date>>>,
  #[prop(optional)] on_focus: Option<SignalSetter<()>>,
  #[prop(optional)] on_blur: Option<SignalSetter<()>>,
) -> impl IntoView {
  let input_ref_local = create_node_ref::<html::Input>();

  let id = id.unwrap_or_else(|| rand::random::<usize>().to_string());

  if let Some(input_ref) = input_ref {
    input_ref_local.on_load(move |input| {
      input_ref.load(&input);
    });
  }

  if let Some(focus) = focus {
    input_ref_local.on_load(move |input| {
      create_effect(move |later_run| {
        if focus.get() {
          if later_run.is_none() {
            input.clone().on_mount(|input| {
              let _ = input.focus();
            });
          }

          let _ = input.focus();
        }
      });
    });
  }

  if let Some(color) = color {
    input_ref_local.on_load(move |input| {
      queue_microtask(move || {
        input.dyn_classes(move || Some(color.get().into_text_input_class()));
      });
    });
  }

  if let Some(size) = size {
    input_ref_local.on_load(move |input| {
      queue_microtask(move || {
        input.dyn_classes(move || Some(size.get().into_text_input_size()));
      });
    });
  }

  if let Some(on_value) = on_value {
    input_ref_local.on_load(move |input| {
      input.on(ev::input, move |e| {
        let value = event_target_value(&e);

        on_value.set(value);
      });
    });
  }

  if let Some(on_value_number) = on_value_number {
    input_ref_local.on_load(move |input| {
      input.on(ev::input, move |e| {
        let el = event_target::<web_sys::HtmlInputElement>(&e);

        on_value_number.set(el.value_as_number())
      });
    });
  }

  if let Some(on_value_date) = on_value_date {
    input_ref_local.on_load(move |input| {
      input.on(ev::input, move |e| {
        let el = event_target::<web_sys::HtmlInputElement>(&e);

        let date = js_sys::Reflect::get(&el, &"valueAsDate".into())
          .unwrap()
          .unchecked_into::<js_sys::Date>();

        let date = date.is_truthy().then_some(date);

        on_value_date.set(date);
      });
    });
  }

  if let Some(on_focus) = on_focus {
    input_ref_local.on_load(move |input| {
      input.on(ev::focus, move |_| on_focus.set(()));
    });
  }

  if let Some(on_blur) = on_blur {
    input_ref_local.on_load(move |input| {
      input.on(ev::blur, move |_| on_blur.set(()));
    });
  }

  let label = {
    let id = id.clone();

    (label.is_some() || label_alt.is_some()).then(|| {
      let label = label.map(|label| {
        view! {
          <span class="label-text">{label}</span>
        }
      });
      let label_alt = label_alt.map(|label_alt| {
        view! {
          <span class="label-text-alt">{label_alt}</span>
        }
      });

      view! {
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
        view! {
          <span class="label-text">{helper}</span>
        }
      });
      let helper_alt = helper_alt.map(|helper_alt| {
        view! {
          <span class="label-text-alt">{helper_alt}</span>
        }
      });

      view! {
        <label class="label" for=id>
          {helper}
          {helper_alt}
        </label>
      }
    })
  };

  if let Some(value_as_date) = value_as_date {
    input_ref_local.on_load(move |input| {
      input.on_mount(move |input| {
        create_effect(move |_| {
          js_sys::Reflect::set(
            &input,
            &"valueAsDate".into(),
            &value_as_date.get(),
          )
          .unwrap();
        });
      });
    });
  }

  view! {
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
        placeholder=placeholder.map(|placeholder| placeholder.get())
        prop:value=value.unwrap_or_default()
      />
      {helper}
    </div>
  }
}
