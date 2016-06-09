# budget
A simple command line budgeting tool

## Paradigm
`budget` is based on the idea of a monthly budget. Many expenses, from
utilities to credit card payments to rent, are monthly, and most people
are paid each month. Therefore, `budget` works on the concept of a
"pay day" on which your budget resets. _Even if you are paid weekly, 
you have to set a monthly payday._

As a user, you tell `budget` when you are paid, when your bills are due,
and how much you want to save each month. `budget` will then tell you 
approximately how much you can spend weekly or daily.

You can log one-off expenses, as well, which will allow `budget` to tell you
how much money you have left for this month or week.

`budget` is just a helpful tool, and is not designed for keeping track of
money on a longer than monthly basis.


## Features
The tool can be called from the command line by typing `budget <command>`.

+ `budget setup <amount> <day>` will set up your budget with your payday on day <day> with amount <amount>
+ `budget status` (or just `budget`) will tell you how long until your next payday, next expense, and how much you have left.
+ `budget spend <amount>` will add a one-time expense, reducing the amount of money you have left.
+ `budget tip <amount>` will add a one-time income, increasing the amount of money you have left.
+ `budget expense <amount> <name> <day of the month>` will add a recurring expense.
+ `budget income <amount> <name> <day of the month>` will add a recurring income, _in addition_ to your payday income.
+ `budget remove <id>` will stop a recurring income or expense.
+ `budget list` will list all the recurring incomes and expenses you've set up.

