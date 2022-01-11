use druid::im;
use druid::widget::Label;
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use wordsmith::game::GuessResult;

fn build_ui() -> impl Widget<GuessResult> {
    Label::<GuessResult>::dynamic(|data, _| data.guess.to_string())
}

fn main() -> Result<(), PlatformError> {
    let data = GuessResult {
        guess: "foobar".into(),
        result: im::Vector::new(),
    };
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(data)?;
    Ok(())
}
