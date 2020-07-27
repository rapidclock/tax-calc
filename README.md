# tax-calc
CLI Tax Calculator

This is a Command Line Tax Calculator.

Given a config file (included in the resources folder), one can estimate the taxes owed based on if the income and if
the individual is single, married or head-of-household.

The config file values can be adjusted to alter the calculations.

We display the breakdown by tax bracket (bracket 0 is the standard deduction) and also
list the total tax owed and the effective tax rate.

## Usage
```shell script
❯ ./tax_calc --help
Tax Calculator 0.1.0

USAGE:
    tax_calc --cfg <CFG> --income <INCOME> <-s|-m|-o>

FLAGS:
    -h, --help       Prints help information
    -o               Indicates Tax Calculation as the head of household
    -m               Indicates Tax Calculation as a Married Couple
    -s               Indicates Tax Calculation as a Single Individuals
    -V, --version    Prints version information

OPTIONS:
    -c, --cfg <CFG>          The file that contains the tax bracket definitions
    -i, --income <INCOME>
```

### Examples

#### Single, Income = 100k

```shell script
$ ./tax_calc -c ./resources/2020.json -i 100000 -s
Tax Analysis for the income : 100000
Rate: 0, Cap: 12400, Bracket Fill: 12400, Tax Amount: 0
Rate: 10, Cap: 9875, Bracket Fill: 9875, Tax Amount: 987.5
Rate: 12, Cap: 40125, Bracket Fill: 40125, Tax Amount: 4815
Rate: 22, Cap: 85525, Bracket Fill: 37600, Tax Amount: 8272
Total Tax : 14074.5, Effective Tax Rate: 14.0745
```

Here,
 - Rate : The rate of Taxation (0 is Standard Deduction).
 - Cap : The maximum amount that can fall in this tax-bracket.
 - Bracket Fill : This is the total amount of the cap that you have utilized for this tax-bracket.
 Usually same as the cap, except for the last bracket.
 - Tax Amount : The amount of tax owed in the specific tax-bracket.
 - Total Tax : The total tax owed based on the income and all tax-brackets.
 - Effective Tax Rate: The percentage tax owed on all income. [(Total Tax / Income) * 100]

#### Married, Income = 250k

```shell script
$ ./tax_calc -c ./resources/2020.json -i 250000 -m
Tax Analysis for the income : 250000
Rate: 0, Cap: 24800, Bracket Fill: 24800, Tax Amount: 0
Rate: 10, Cap: 19750, Bracket Fill: 19750, Tax Amount: 1975
Rate: 12, Cap: 80250, Bracket Fill: 80250, Tax Amount: 9630
Rate: 22, Cap: 171050, Bracket Fill: 125200, Tax Amount: 27544
Total Tax : 39149, Effective Tax Rate: 15.659600000000001
```