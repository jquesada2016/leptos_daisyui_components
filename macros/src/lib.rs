#[macro_use]
extern crate proc_macro_error;

use std::io::Write;

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn include_component_classes(
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input = proc_macro2::TokenStream::from(input);

  if !input.is_empty() {
    abort!(input, "this macro doesn't accept any input");
  }

  let mut file = std::io::BufWriter::new(
    std::fs::File::create("./daisyui_component_classes.txt").unwrap_or_else(
      |err| {
        abort_call_site!(
          "failed to create `./daisy_ui_component_classes.txt` file, \
           error:\n{:#?}",
          err
        )
      },
    ),
  );

  if cfg!(feature = "text-input") {
    file.write_all(classes::TEXT_INPUT.as_ref()).unwrap();
  }

  if cfg!(feature = "button") {
    file.write_all(classes::BUTTON.as_ref()).unwrap();
  }

  proc_macro::TokenStream::default()
}

mod classes {
  #[rustfmt::skip]
  pub const TEXT_INPUT: &str =
    "form-control\n\
    label\n\
    input\n\
    input-bordered\n\
    input-ghost\n\
    input-primary\n\
    input-secondary\n\
    input-accent\n\
    input-info\n\
    input-success\n\
    input-warning\n\
    input-error\n\
    input-lg\n\
    input-md\n\
    input-sm\n\
    input-xs\n";

  #[rustfmt::skip]
  pub const BUTTON: &str =
    "btn\n\
     btn-primary\n\
     btn-secondary\n\
     btn-accent\n\
     btn-info\n\
     btn-success\n\
     btn-warning\n\
     btn-error\n\
     btn-ghost\n\
     btn-link\n\
     btn-outline\n\
     btn-active\n\
     btn-disabled\n\
     glass\n\
     loading\n\
     no-animation\n\
     btn-lg\n\
     btn-md\n\
     btn-sm\n\
     btn-xs\n\
     btn-wide\n\
     btn-block\n\
     btn-circle\n\
     btn-square\n";
}
