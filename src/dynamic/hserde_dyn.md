# Dynamic HSerde

Goal: use least space possible

## Implementation

- Text Version
- Binary Version
- Rust Enum (internally)
- JSON

(complete) conversion functions between all of these

## Representations

### Text Version

chr(32) ~ chr(126)

| Data      | Type                                              | Represents  |
|-----------|---------------------------------------------------|-------------|
| 0 ~ 2     | None -> Bool \| None                              | true, false, none  |
| 3 ~ 50    | None -> Integer                                   | an integer representing -16 ~ 31  |
| 51        | bits: [char; 1] -> Integer                        | `bits` as an Integer + 17, representing 17 ~ 111  |
| 52        | bits: [char; 2] -> Integer                        | `bits` as an Integer + 112, representing 112 ~ 9136  |
| 53        | bits: [char; 3] -> Integer                        | `bits` as an Integer + 9137, representing 9137 ~ 866511  |
| 54        | bits: [char; 4] -> Integer                        | `bits` as an Integer + 866512, representing 866512 ~ |
| 55        | bits: [char; 5] -> Integer                        | `bits` as an Integer + , representing ~ |
| 56        | bits: [char; 6] -> Integer                        | `bits` as an Integer + , representing ~ |
| 57        | n: UInt, bits: [char; (n+7)] -> Integer           | `bits` as an Integer + , representing ~ inf  |
| 58        | n: UInt -> Integer                                | `-n`  |
| 59 ~ 65   | elements: [Any; (Data-59)] -> Vector              | a Vector with (Data - 59) elements  |
| 66        | n: UInt, elements: [Any; (n+7)] -> Vector         | a Vector with `n + 7` elements   |
| [[colspan=3]] A table consists of 2n elements, odd elements for keys and even elements for values  |
| 67 ~ 73   | elements: [Any; (Data-67)*2] -> Table             | a Table with (Data - 67) elements   |
| 74        | n: UInt, elements: [Any; (n+7)*2] -> Table        | a Table with `n + 7` elements      |
| [[colspan=3]] A string is a type alias of a vector of unsigned integers  |
| 75 ~ 81   | characters: [UInt; (Data-75)] -> String           | a String with (Data - 75) characters  |
| 82        | n: UInt, characters: [UInt; (n+7)] -> String      | a String with `n + 7` characters  |
| [[colspan=3]] Todo: Real Numbers: How about giving multiple options?  |

### Text Version: `UInt`

| Data      | Type                                              | Represents  |
|-----------|---------------------------------------------------|-------------|
| 0 ~ 71    | None -> UInt                                      | a UInt representing 0 ~ 71  |
| 72        | bits: [char; 1] -> Integer                        | `bits` as an Integer + , representing ~ |
| 73        | bits: [char; 2] -> Integer                        | `bits` as an Integer + , representing ~ |
| 74        | bits: [char; 3] -> Integer                        | `bits` as an Integer + , representing ~ |
| 75        | bits: [char; 4] -> Integer                        | `bits` as an Integer + , representing ~ |
| 76        | bits: [char; 5] -> Integer                        | `bits` as an Integer + , representing ~ |
| 77        | bits: [char; 6] -> Integer                        | `bits` as an Integer + , representing ~ |
| 78        | bits: [char; 7] -> Integer                        | `bits` as an Integer + , representing ~ |
| 79        | bits: [char; 8] -> Integer                        | `bits` as an Integer + , representing ~ |
| 80        | bits: [char; 9] -> Integer                        | `bits` as an Integer + , representing ~ |
| 81        | n: UInt, bits: [char; (n+10)] -> Integer          | `bits` as an Integer + , representing ~ inf  |

### Binary Version

0 ~ 255

### Binary Version: `UInt`