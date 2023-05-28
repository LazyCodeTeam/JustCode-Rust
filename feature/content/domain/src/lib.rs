use std::collections::HashMap;

use model::historical_answer::HistoricalAnswer;

mod detect_modifications;
mod into_modification;
pub mod model;

pub trait Personalize<TOut> {
    fn personalize(self, correct_historical_answers: &HashMap<String, HistoricalAnswer>) -> TOut;
}

impl<TIn, TOut> Personalize<Vec<TOut>> for Vec<TIn>
where
    TIn: Personalize<TOut>,
{
    fn personalize(
        self,
        correct_historical_answers: &HashMap<String, HistoricalAnswer>,
    ) -> Vec<TOut> {
        self.into_iter()
            .map(|item| item.personalize(correct_historical_answers))
            .collect()
    }
}
