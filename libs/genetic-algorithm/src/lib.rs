use std::cmp::Ordering;

pub use self::{
    chromosome::*, crossover::*, individual::*, mutation::*, selection::*,
};

use rand::RngCore;

mod chromosome;
mod crossover;
mod individual;
mod selection;
mod mutation;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(selection_method: S, crossover_method: impl CrossoverMethod + 'static, mutation_method: impl MutationMethod + 'static) -> Self {
        Self { selection_method, crossover_method: Box::new(crossover_method), mutation_method: Box::new(mutation_method), }
    }

    pub fn evolve<I>(&mut self, rng: &mut dyn RngCore, mut population: &mut [I]) -> (Vec<I>, Statistics)
    where
        I: Individual + Clone,
    {
        assert!(!population.is_empty());
        self.selection_method.set_not_sorted_population();

        let new_population = (0..population.len())
            .map(|_| {
                self.selection_method.sort(&mut population);

                let parent_a = self.selection_method.select(rng, population).chromosome();

                let parent_b = self.selection_method.select(rng, population).chromosome();

                let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
                
                self.mutation_method.mutate(rng, &mut child);

                I::create(child)

            })
            .collect();
        let stats = Statistics::new(population);

        (new_population, stats)
    }
}

#[derive(Clone, Debug)]
pub struct Statistics {
    min_fitness: f32,
    max_fitness: f32,
    avg_fitness: f32,
    median_fitness: f32,
}

impl Statistics {
    fn new<I>(population: &[I]) -> Self
    where
        I: Individual + Clone,
    {
        assert!(!population.is_empty());

        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;
        
        let mut sorted_population = population.clone().to_vec();
        sorted_population.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap_or(Ordering::Equal));

        let median_fitness = if sorted_population.len() % 2 == 0 {
            (sorted_population[sorted_population.len() / 2].fitness() + sorted_population[sorted_population.len() / 2 - 1].fitness()) / 2.0
        } else {
            sorted_population[sorted_population.len() / 2].fitness()
        };

        for individual in population {
            let fitness = individual.fitness();

            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        Self {
            min_fitness,
            max_fitness,
            avg_fitness: sum_fitness / (population.len() as f32),
            median_fitness,
        }
    }

    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }
    pub fn median_fitness(&self) -> f32 {
        self.median_fitness
    }
}

#[cfg(test)]
mod tests {
    use crate::{selection::{RouletteWheelSelection, RankSelection}, crossover::UniformCrossover, mutation::GaussianMutation, individual::TestIndividual};

    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        TestIndividual::create(chromosome)
    }

    #[test]
    fn test_with_roulette_wheel_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let mut ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];
        

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particular reason for a number of 10 
        // that'd change is the *magnitude* of difference between
        // initial and output population.
        for _ in 0..10 {
            (population, _) = ga.evolve(&mut rng, &mut population);
        }

        let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ]; // for Roulette Wheel Selection

        assert_eq!(population, expected_population);
    }
    
    #[test]
    fn test_with_rank_selection() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let mut ga = GeneticAlgorithm::new(
            RankSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        // We're running `.evolve()` a few times, so that the
        // differences between initial and output population are
        // easier to spot.
        //
        // No particular reason for a number of 10 
        // that'd change is the *magnitude* of difference between
        // initial and output population.
        for _ in 0..10 {
            (population, _) = ga.evolve(&mut rng, &mut population);
        }

        //let expected_population = vec![
        //    individual(&[0.44769490, 2.0648358, 4.3058133]),
        //    individual(&[1.21268670, 1.5538777, 2.8869110]),
        //    individual(&[1.06176780, 2.2657390, 4.4287640]),
        //    individual(&[0.95909685, 2.4618788, 4.0247330]),
        //]; // for Roulette Wheel Selection

        let expected_population = vec![
            individual(&[0.9437746, 2.7132483, 4.413993]),
            individual(&[0.8460895, 2.1113086, 4.568947]),
            individual(&[0.5653255, 2.326864, 4.568947]),
            individual(&[0.5033445, 2.326864, 4.413993] ),
        ]; // for rank selection

        assert_eq!(population, expected_population);
    }
}
