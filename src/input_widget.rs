use tui_input::Input;

pub fn visual_input_text(input: &mut Input) -> String {
    let mut input_text = input.value().to_owned();
    if input_text.len() == input.cursor() {
        input_text.push('\u{2588}');
    } else {
        let cursor_pos = input.visual_cursor();
        input_text.replace_range(cursor_pos..=cursor_pos, "\u{2588}");
    };
    return input_text;
}
