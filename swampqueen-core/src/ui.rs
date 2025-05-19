//! User interface functionality.

use cli_prompts::{
    DisplayPrompt,
    prompts::{AbortReason, Input, Selection},
    style::{Formatting, InputStyle},
};

/// Text input prompt. Performs the validation and transformation function, continuing the prompt
/// until a valid input is given.
///
/// # Errors
///
/// This function returns an [`AbortReason`] when the input is unexpectedly terminated.
pub fn input<F, T>(prompt: &str, f: F) -> Result<T, AbortReason>
where
    F: Fn(&str) -> Result<T, String>,
{
    Input::new(prompt, f)
        .style(InputStyle::default().default_value_formatting(Formatting::default().bold()))
        .display()
}

/// Allow the user to select an item from the given array of options.
///
/// # Errors
///
/// This function returns an [`AbortReason`] when the selection is unexpectedly terminated.
pub fn select<S>(prompt: &str, options: &[S]) -> Result<S, AbortReason>
where
    S: Clone,
    for<'a> &'a S: Into<String>,
{
    let prompt = Selection::new(prompt, options.iter());
    prompt.display().cloned()
}
