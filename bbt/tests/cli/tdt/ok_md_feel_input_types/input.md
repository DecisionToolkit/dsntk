| F | Age      | Status            | Active | Value  | Result      |
|:-:|:--------:|:-----------------:|:------:|:------:|:-----------:|
|   | `in`     | `in`              | `in`   | `in`   | `out`       |
| 1 | [18..65] | "Gold","Platinum" | true   | -      | "Premium"   |
| 2 | [18..65] | not("Cancelled")  | true   | >0     | "Standard"  |
| 3 | <18      | -                 | -      | -      | "Minor"     |
| 4 | >65      | -                 | true   | -      | "Senior"    |
| 5 | -        | "Cancelled"       | -      | -      | "Cancelled" |
| 6 | -        | -                 | false  | -      | "Inactive"  |
| 7 | -        | -                 | -      | null   | "No Value"  |
