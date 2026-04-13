[dsntk] | [ÐecisionToolkit]

[ÐecisionToolkit]: https://github.com/DecisionToolkit

[dsntk]: https://crates.io/crates/dsntk

[DMN 1.3]: https://www.omg.org/spec/DMN/1.3/PDF

# FEEL grammar in [DMN 1.3]

1. expression =
    - a. boxed expression |
    - b. textual expression ;

2. textual expression =
    - a. for expression | if expression | quantified expression |
    - b. disjunction |
    - c. conjunction |
    - d. comparison |
    - e. arithmetic expression |
    - f. instance of |
    - g. path expression | filter expression | function invocation |
    - h. literal | simple positive unary test | name | `(` expression `)` ;

3. textual expressions = textual expression { `,` textual expression } ;

4. arithmetic expression =
    - a. addition | subtraction |
    - b. multiplication | division |
    - c. exponentiation |
    - d. arithmetic negation ;

5. simple expression = arithmetic expression | simple value ;

6. simple expressions = simple expression { `,` simple expression } ;

7. simple positive unary test =
    - a. [ `<` | `<=` | `>` | `>=` ] endpoint |
    - b. interval ;

8. interval = ( open interval start | closed interval start ) endpoint `..` endpoint ( open interval end | closed interval end ) ;

9. open interval start = `(` | `]` ;

10. closed interval start = `[` ;

11. open interval end = `)` | `[` ;

12. closed interval end = `]` ;

13. positive unary test = expression ;

14. positive unary tests = positive unary test { `,` positive unary test } ;

15. unary tests =
    - a. positive unary tests |
    - b. `not` `(` positive unary tests `)` |
    - c. `-` ;

16. endpoint = simple value ;

17. simple value = qualified name | simple literal ;

18. qualified name = name { `.` name } ;

19. addition = expression `+` expression ;

20. subtraction = expression `-` expression ;

21. multiplication = expression `*` expression ;

22. division = expression `/` expression ;

23. exponentiation = expression `**` expression ;

24. arithmetic negation = `-` expression ;

25. name = name start { name part | additional name symbols } ;

26. name start = name start char { name part char } ;

27. name part = name part char { name part char } ;

28. name start char = `?` | `[A-Z]` | `_` | `[a-z]`
    | `[\uC0-\uD6]` | `[\uD8-\uF6]` | `[\uF8-\u2FF]` | `[\u370-\u37D]`
    | `[\u37F-\u1FFF]` | `[\u200C-\u200D]` | `[\u2070-\u218F]` | `[\u2C00-\u2FEF]`
    | `[\u3001-\uD7FF]` | `[\uF900-\uFDCF]` | `[\uFDF0-\uFFFD]` | `[\u10000-\uEFFFF]` ;

29. name part char = name start char | digit | `\uB7` | `[\u0300-\u036F]` | `[\u203F-\u2040]` ;

30. additional name symbols = `.` | `/` | `-` | `’` | `+` | `*` ;

31. literal = simple literal | `null` ;

32. simple literal = numeric literal | string literal | boolean literal | date time literal ;

33. string literal = `"` { character – (`"` | vertical space) | string escape sequence} `"` ;

34. boolean literal = `true` | `false` ;

35. numeric literal = [ `-` ] ( digits [ `.` digits ] | `.` digits ) ;

36. digit = [0-9] ;

37. digits = digit { digit } ;

38. function invocation = expression parameters ;

39. parameters = `(` ( named parameters | positional parameters ) `)` ;

40. named parameters = parameter name `:` expression { `,` parameter name `:` expression } ;

41. parameter name = name ;

42. positional parameters = [ expression { `,` expression } ] ;

43. path expression = expression `.` name ;

44. for expression = `for` name `in` iteration context { `,` name `in` iteration context } `return` expression ;

45. if expression = `if` expression `then` expression `else` expression ;

46. quantified expression = (`some` | `every`) name `in` expression { `,` name `in` expression } `satisfies` expression ;

47. disjunction = expression `or` expression ;

48. conjunction = expression `and` expression ;

49. comparison =
    - a. expression ( `=` | `!=` | `<` | `<=` | `>` | `>=` ) expression |
    - b. expression `between` expression `and` expression |
    - c. expression `in` positive unary test |
    - d. expression `in` `(` positive unary tests `)` ;

50. filter expression = expression `[` expression `]` ;

51. instance of = expression `instance` `of` type ;

52. type =
    - a. qualified name |
    - b. `list` `<` type `>` |
    - c. `context` `<` name `:` type { `,` name `:` type } `>` |
    - d. `function` `<` [ type { `,` type } ] `>` `->` type ;

53. boxed expression = list | function definition | context ;

54. list = `[` [ expression { `,` expression } ] `]` ;

55. function definition = `function` `(` [ formal parameter { `,` formal parameter } ] `)` [ `external` ] expression ;

56. formal parameter = parameter name [`:` type ] ;

57. context = `{` [ context entry { `,` context entry } ] `}` ;

58. context entry = key `:` expression ;

59. key = name | string literal ;

60. date time literal = at literal | function invocation ;

61. white space = vertical space | `\u0009` | `\u0020` | `\u0085` | `\u00A0` | `\u1680` | `\u180E` | `[\u2000-\u200B]`
    | `\u2028` | `\u2029` | `\u202F` | `\u205F` | `\u3000` | `\uFEFF` ;

62. vertical space = `[\u000A-\u000D]` ;

63. iteration context = expression [ `..` expression ] ;

64. string escape sequence = `\'` | `\"` | `\\` | `\n` | `\r` | `\t` | code point;

65. at literal = `@` string literal
