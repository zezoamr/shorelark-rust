use rand::Rng;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Clone, Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

// represents a single layer's topology for initializing the network
#[derive(Clone, Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Clone, Debug)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Network {
    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        let layers = layers
            .windows(2)
            .map(|layer| Layer::random(rng, layer[0].neurons, layer[1].neurons))
            .collect();
        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    pub fn random(rng: &mut dyn rand::RngCore, input_neurons: usize, output_neurons: usize) -> Self {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();
        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    pub fn random(rng: &mut dyn rand::RngCore, output_neurons: usize) -> Self {
        Self {
            weights: (0..output_neurons)
                .map(|_| rng.gen_range(-1.0..=1.0))
                .collect(),
            bias: rng.gen_range(-1.0..=1.0),
        }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        // represent input into a node
        assert_eq!(inputs.len(), self.weights.len());
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();
        // adding bias and doing relu activation function
        (self.bias + output).max(0.0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    mod neuron {
        use super::*;

        mod test_neuron_propagate {
            use super::*;

            #[test]
            fn test() {
                let neuron = Neuron {
                    bias: 0.5,
                    weights: vec![-0.3, 0.8],
                };
            
                // Ensures `.max()` (our ReLU) works:
                approx::assert_relative_eq!(
                    neuron.propagate(&[-10.0, -10.0]),
                    0.0,
                );
            
                // `0.5` and `1.0` chosen by a fair dice roll:
                approx::assert_relative_eq!(
                    neuron.propagate(&[0.5, 1.0]),
                    (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
                );
            
            }
        }

        // due to the random neuron initialization, this test is not deterministic and the numbers need to be reproduced by first running the test 
        // and failing it the plugging in those numbers
        mod test_neuron_random {
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use super::*;

            #[test]
            fn test() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let neuron = Neuron::random(&mut rng, 4);

                approx::assert_relative_eq!(neuron.bias, 0.5238807);

                approx::assert_relative_eq!(neuron.weights.as_slice(), [-0.6255188, 0.67383957, 0.8181262, 0.26284897].as_ref());
            }
        }
    }
}
