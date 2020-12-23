use teloxide::prelude::*;
use teloxide_macros::teloxide;

use super::states::*;

#[teloxide(subtransition)]
async fn setup(state: SetupState, cx: TransitionIn, ans: String) -> TransitionOut<Dialogue> {
    next(state)
}

#[teloxide(subtransition)]
async fn ready(state: ReadyState, cx: TransitionIn, ans: String) -> TransitionOut<Dialogue> {
    next(state)
}

use thiserror::Error;
#[derive(Debug, Error)]
enum Error2 {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
}

//use teloxide::prelude::*;
//use teloxide_macros::teloxide;
//
//use super::states::*;
//
//#[teloxide(subtransition)]
//async fn start(state: StartState, cx: TransitionIn, ans: String) -> TransitionOut<Dialogue> {
//    if let Ok(number) = ans.parse() {
//        cx.answer_str(format!(
//            "Remembered number {}. Now use /get or /reset",
//            number
//        ))
//        .await?;
//        next(HaveNumberState { number })
//    } else {
//        cx.answer_str("Please, send me a number").await?;
//        next(state)
//    }
//}
//
//#[teloxide(subtransition)]
//async fn have_number(
//    state: HaveNumberState,
//    cx: TransitionIn,
//    ans: String,
//) -> TransitionOut<Dialogue> {
//    let num = state.number;
//
//    if ans.starts_with("/get") {
//        cx.answer_str(format!("Here is your number: {}", num))
//            .await?;
//        next(state)
//    } else if ans.starts_with("/reset") {
//        cx.answer_str("Resetted number").await?;
//        next(StartState)
//    } else {
//        cx.answer_str("Please, send /get or /reset").await?;
//        next(state)
//    }
//}
