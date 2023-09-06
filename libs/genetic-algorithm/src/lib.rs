
use individual::Individual;
use rand::RngCore;
use selection::SelectionMethod;

mod chromosome;
mod individual;
mod selection;

#[derive(Clone, Debug)]
pub struct GeneticAlgorithm<R, S> {
    rng: R,
    selection_method: S,
}

impl<R, S> GeneticAlgorithm<R, S>
where
    R: RngCore,
    S: SelectionMethod,
{
    pub fn new(rng: R, selection_method: S) -> Self {
        Self { rng, selection_method }
    }

    pub fn evolve<I>(&mut self, rng: &mut dyn RngCore, population: & mut[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self.selection_method.select(rng, population);

                let parent_b = self.selection_method.select(rng, population);

                // TODO crossover
                // TODO mutation
                todo!()
            })
            .collect()
    }
}
