# solar_crowd_funding

This is a solar crowdfunding Rust Smart Contract(Solana Blockchain).
It aims to provide institutions that want to acquire expensive solar projects with affordable financing through individual investors.
The funds from the investors are pulled together and assigned to a specific solar project and institution. 
Once the solar project is successfully completed, the institution is expected to be paying a certain amount on monthly basis for given period.
The amount paid by the institution will be shared accross the individual investors of the project on monthly basis.

Below are some features contained in the program:

- Register investor
- Register institution
- Invest in project(done by investor)
- Pay solar bill by institution

## Getting started

In order to run this example program you will need to install Rust and
Solana. Information about installing Rust can be found
[here](https://rustup.rs/) and information about installing Solana can
be found [here](https://docs.solana.com/cli/install-solana-cli-tools).

Once you've completed the Solana installation run the following
commands to configure you machine for local development:

```
solana config set --url localhost
solana-keygen new
```

These two commands create Solana config files in `~/.config/solana/`
which solana command line tools will read in to determine what cluster
to connect to and what keypair to use.

Having done that run a local Solana validator by running:

```
solana-test-validator
```

This program must be left running in the background.

## Deploying the Solana program

To deploy the Solana program in this repository to the Solana cluster
that you have configured run:

```
anchor deploy
```

## Running the test program

To run the test program you must have already deployed the Solana
program. The test program sends a transaction to the Solana
blockchain asking it to execute the deployed program and reports the
results.

```
anchor test --skip-local-validator
```
