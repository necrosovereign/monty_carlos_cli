// Copyright 2024 Vladimir Kharchev

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A simple CLI program that uses [`monty_carlos`] crate to run Monte-Carlo simulations.
#![warn(clippy::pedantic)]
use clap::{Args, Parser, ValueEnum};

use monty_carlos::sample::{KSSample, LillieforsSample, Sample};
use monty_carlos::MonteCarlo;
use statrs::distribution::Normal;

/// Enum for the CLI option to choose Kolmogorov-Smirnov or Lilliefors test
#[derive(ValueEnum, Clone, Copy)]
enum Test {
    /// Run Kolmogorov-Smirnov test.
    KolmogorovSmirnov,
    /// Run Lilliefors test.
    Lilliefors,
}

/// The enum [`SimulationType`] in the form required by [clap].
///
/// [clap] doesn't allow enums with non-unit variants, so the argument for the [`SimulationType`]
/// is represented by a quasi-isomorphic struct. The attribute `#[group(required = true, multiple =
/// false]` ensures that any instance of [`SimulationTypeArg`] created by [clap] is a member of a
/// subtype, that is isomorphic to [`SimulationType`]. [`SimulationTypeArg::condence`] is this
/// isomorphism.
#[derive(Args, Clone, Copy)]
#[group(required = true, multiple = false)]
struct SimulationTypeArg {
    #[arg(long)]
    /// Calculate the probability that the statistic is less than the given value.
    test_statistic: Option<f64>,
    #[arg(long)]
    /// Output the distribution of statistics in the simulation.
    make_distribution: bool,
}

impl SimulationTypeArg {
    /// The isomorphism from the valid subtype of [`SimulationTypeArg`] to [`SimulationType`]
    fn condence(self) -> SimulationType {
        if let Some(test_statistic) = self.test_statistic {
            SimulationType::TestStatistic(test_statistic)
        } else {
            SimulationType::MakeDistribution
        }
    }
}

/// Describes which result is requested from the simulation.
enum SimulationType {
    /// Probability that the test statistic is less than the value.
    TestStatistic(f64),
    /// The distribution of statistics in the simulation.
    MakeDistribution,
}

/// The struct for command-line arguments.
#[derive(Parser)]
#[command(about = "Runs Monte-Carlo simulations", long_about = None)]
struct Cli {
    /// The size of simulated datasets of Kolmogorov-Smirnov or Lilliefors test.
    samples: usize,
    /// Number of iterations of the simulation.
    #[arg(long)]
    iterations: Option<usize>,
    /// Which result should be produced.
    #[command(flatten)]
    simulation_type: SimulationTypeArg,
    /// The statistical test to be simulated.
    #[arg(value_enum)]
    test: Test,
}

/// Part of [main] that is independent of the type of `S`.
#[allow(clippy::needless_pass_by_value)] // This function acts as `main` after it has been called.
fn run_with_sample<S: Sample>(cli: Cli, sample: S) {
    let mut simulator = MonteCarlo::new(sample);
    if let Some(iterations) = cli.iterations {
        simulator.iterations = iterations;
    }

    match cli.simulation_type.condence() {
        SimulationType::TestStatistic(test_statistic) => {
            let pvalue = simulator.simulate_pvalue(test_statistic);
            println!("pvalue = {pvalue}");
        }
        SimulationType::MakeDistribution => {
            let distr = simulator.simulate_distribution();
            println!("{distr:?}");
        }
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.test {
        Test::KolmogorovSmirnov => {
            let sample = KSSample::new(Normal::new(0.0, 1.0).unwrap(), cli.samples).unwrap();
            run_with_sample(cli, sample);
        }
        Test::Lilliefors => {
            let sample =
                LillieforsSample::new(Normal::new(0.0, 1.0).unwrap(), cli.samples).unwrap();
            run_with_sample(cli, sample);
        }
    }
}
