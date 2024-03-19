# Monty-Carlos CLI

A CLI interface for the [`Monty-Carlos`](https://github.com/necrosovereign/monty_carlos) library.

## Usage
```
monty_carlos_cli [OPTIONS] <--test-statistic <TEST_STATISTIC>|--make-distribution> <SAMPLES> <TEST>

Arguments:
  <SAMPLES>
          The size of simulated datasets of Kolmogorov-Smirnov or Lilliefors test

  <TEST>
          The statistical test to be simulated

          Possible values:
          - kolmogorov-smirnov: Run Kolmogorov-Smirnov test
          - lilliefors:         Run Lilliefors test

Options:
      --iterations <ITERATIONS>
          Number of iterations of the simulation

      --test-statistic <TEST_STATISTIC>
          Calculate the probability that the statistic is less than the given value

      --make-distribution
          Output the distribution of statistics in the simulation

  -h, --help
          Print help (see a summary with '-h')
```
