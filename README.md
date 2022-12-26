# monthly-rust

A budget app designed to detect your recurring expenses. Created as an educational tool, the goal is to answer: What transactions repeat?

Instructions:

- Download credit card or bank statements as a CSV files (1 year)
- Download the app from the releases tab in GitHub
- Open the app and point at a directory or choose a file

## TODO:

[x] Commit to Github
[x] read from path
[x] read from dynamic path
[x] get a list
[x] map over to format our info
[x] write some matching cases
[x] filter by those cases into:
[x] final list
[x] calc the sum
[x] naive matching
[] Bug: Quarterly like ClearDefense is an issue......... auto insurance, break these up into "Suggestions"?
[] Items that recurr only x may need to be suggested, or if they occur 4 times,
[] measure first & last date, to determine months span?? Does this mean to detect over yearly...
[] mark as quartlery, half yearly, yearly, etc. based on frequency
[] can't assume that 2 transactions in 6 months will repeat every year
[] ORDER BY ( date, amount, desc, etc.)

### TODO GUI:

[] make an import function
[] Make a Slider on how many must match ( 2 - 6)
[] Move things out into modules
[] Scrub the doc for anything like a regex of "Desc, Descript, etc. all variation"

### Keyboard TODO:

[] Add | bar on the keyboard on the same place as shift
[] Add & to help with borrowing
[] add key for accepting copilot suggestiosn !!
