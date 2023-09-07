
use crossover::CrossoverMethod;
use individual::Individual;
use rand::RngCore;
use selection::SelectionMethod;

mod chromosome;
mod crossover;
mod individual;
mod selection;
mod mutation;


pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static) -> Self {
        Self { selection_method, crossover_method: Box::new(crossover_method), }
    }

    pub fn evolve<I>(&mut self, rng: &mut dyn RngCore, mut population: &mut [I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());
        self.selection_method.set_not_sorted_population();

        (0..population.len())
            .map(|_| {
                self.selection_method.sort(&mut population);

                let parent_a = self.selection_method.select(rng, population).chromosome();

                let parent_b = self.selection_method.select(rng, population).chromosome();

                let child = self.crossover_method.crossover(rng, parent_a, parent_b);
                
                // TODO mutation
                // TODO convert `Chromosome` back into `Individual`
                todo!()
            })
            .collect()
    }
}
