
use rand::{seq::SliceRandom, RngCore};

use crate::*;

pub trait SelectionMethod {
    fn select<'a, I>(&mut self, rng: &mut dyn RngCore, population: &'a[I]) -> &'a I
    where
        I: Individual;

    fn sort<'a, I>(&mut self, population: &mut [I]) -> &Self
    where
        I: Individual;
    fn set_sorted_population(&mut self) -> &Self;
    fn set_not_sorted_population(&mut self) -> &Self;
}

#[derive(Clone, Debug)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {

    fn select<'a, I>(&mut self, rng: &mut dyn RngCore, population: &'a[I]) -> &'a I
    where
        I: Individual,
    {
            population
                .choose_weighted(rng, |individual| individual.fitness())
                .expect("got an empty population")
    }
    fn sort<'a, I>(&mut self, population: &mut [I]) -> &Self
        where
            I: Individual {
                self
    }
    fn set_sorted_population(&mut self) -> &Self {
        self
    }
    fn set_not_sorted_population(&mut self) -> &Self {
        self
    }
}

#[derive(Clone, Debug)]
pub struct RankSelection{
    is_sorted: bool,
}

impl RankSelection {
    pub fn new() -> Self {
        Self {
            is_sorted: false
        }
    }
}

impl SelectionMethod for RankSelection {

    fn select<'a, I>(&mut self, rng: &mut dyn RngCore, population: &'a[I]) -> &'a I
    where
        I: Individual,
        {
            // assuming used sort method to sort population before here
            let total_fitness: f64 = (1..=population.len()).sum::<usize>() as f64; 
            // Calculate the total fitness of the population as the sum of all ranks
            // Choose an individual from the population using weighted random selection,
            // where the weight of each individual is its rank divided by the total fitness of the population
            population
                .choose_weighted(rng, |individual| {
                    let rank = population
                        .iter()
                        .position(|x| x.fitness() == individual.fitness())
                        .unwrap() + 1; // Find the rank of the individual by finding its position in the sorted population and adding 1
                    // The + 1 is added to the result of the position method to convert the zero-based index returned by the method into a one-based rank. ranks are typically assigned starting from 1
                    rank as f64 / total_fitness // Calculate the selection probability of the individual as its rank divided by the total fitness of the population
                })
                .expect("got an empty population")
        }

        fn sort<'a, I>(&mut self, population: &mut [I]) -> &Self
        where
                I: Individual,
            {
                if !self.is_sorted {
                    population.sort_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap()); // Sort the population by fitness in ascending order
                    self.is_sorted = true; // Set is_sorted to true
                }
                self.set_sorted_population();
                self
            }
        fn set_sorted_population(&mut self) -> &Self {
            self.is_sorted = true;
            self
        }
        fn set_not_sorted_population(&mut self) -> &Self {
            self.is_sorted = false;
            self
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod roulette_wheel_selection {

        mod test_selection_with_histogram_nondeterminstic {
            use std::collections::BTreeMap;

            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use crate::{selection::{RouletteWheelSelection, SelectionMethod}, individual::{TestIndividual, Individual}};

            #[test]
            fn test() {
                let mut method: RouletteWheelSelection = RouletteWheelSelection::new();
                method.set_not_sorted_population();
                let mut rng = ChaCha8Rng::from_seed(Default::default());

                let mut population = vec![
                    TestIndividual::new(2.0),
                    TestIndividual::new(1.0),
                    TestIndividual::new(4.0),
                    TestIndividual::new(3.0),
                ];

                let mut actual_histogram = BTreeMap::new();


                for _ in 0..1000 {
                    method.sort(&mut population);
                    let fitness = method.select(&mut rng, &population)
                        .fitness() as i32;

                    *actual_histogram
                        .entry(fitness)
                        .or_insert(0) += 1;
                }

                let expected_histogram = BTreeMap::from_iter(vec![
                    // (fitness, how many times this fitness has been chosen)
                    (1, 98),
                    (2, 202),
                    (3, 278),
                    (4, 422),
                ]);

                assert_eq!(actual_histogram, expected_histogram);
            }
        }
    }

    mod rank_selection {

        mod test_selection_with_histogram_nondeterminstic {
            use std::collections::BTreeMap;

            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use crate::{selection::{SelectionMethod, RankSelection}, individual::{TestIndividual, Individual}};

            #[test]
            fn test() {
                let mut method: RankSelection = RankSelection::new();
                method.set_not_sorted_population();
                let mut rng = ChaCha8Rng::from_seed(Default::default());

                let mut population = vec![
                    TestIndividual::new(2.0),
                    TestIndividual::new(1.0),
                    TestIndividual::new(4.0),
                    TestIndividual::new(3.0),
                ];

                let mut actual_histogram = BTreeMap::new();

                for _ in 0..1000 {
                    method.sort(&mut population);
                    let fitness = method.select(&mut rng, &population)
                        .fitness() as i32;

                    *actual_histogram
                        .entry(fitness)
                        .or_insert(0) += 1;
                }

                let expected_histogram = BTreeMap::from_iter(vec![
                    // (fitness, how many times this fitness has been chosen)
                    (1, 95),
                    (2, 194),
                    (3, 310),
                    (4, 401),
                ]);

                assert_eq!(actual_histogram, expected_histogram);
            }
        }
    }
    
}
