# Dynamic HSerde

Goal: use least space possible

## Implementation

- Text Version
- Binary Version
- Rust Enum (internally)
  - Raw Rust type
  - ex) `Vec<u16>` to `HSObj::Vec`
- JSON

(complete) conversion functions between all of these

## Representations

### Text Version

chr(32) ~ chr(126)

| Data      | Type                                              | Represents  |
|-----------|---------------------------------------------------|-------------|
| 0 ~ 2     | () -> Bool \| None                                | true, false, none  |
| 3 ~ 50    | () -> Integer                                     | Data - 19, an integer representing -16 ~ 31 |
| 51        | bits: [Char; 1] -> UInt                           | `to_uint(bits) + 32`, representing 32 ~ 126 |
| 52        | bits: [Char; 2] -> UInt                           | `to_uint(bits) + 127`, representing 127 ~ 9151 |
| 53        | bits: [Char; 3] -> UInt                           | `to_uint(bits) + 9152`, representing 9152 ~ |
| 54        | bits: [Char; 4] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 55        | bits: [Char; 5] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 56        | bits: [Char; 6] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 57        | n: UInt, bits: [Char; (n+7)] -> UInt              | `to_uint(bits) + `, representing ~ inf  |
| 58        | n: UInt -> Integer                                | `-n`  |
| 59 ~ 66   | elements: [Any; (Data-59)] -> Vector              | a Vector with (Data - 59) elements  |
| 67        | n: UInt, elements: [Any; (n+8)] -> Vector         | a Vector with `n + 8` elements   |
| [[colspan=3]] A table consists of 2n elements, odd elements for keys and even elements for values  |
| 68 ~ 75   | elements: [Any; (Data-68)*2] -> Table             | a Table with (Data - 68) elements   |
| 76        | n: UInt, elements: [Any; (n+8)*2] -> Table        | a Table with `n + 8` elements      |
| [[colspan=3]] A string is a type alias of a vector of unsigned integers  |
| 77 ~ 84   | characters: [UInt; (Data-77)] -> String           | a String with (Data - 77) characters  |
| 85        | n: UInt, characters: [UInt; (n+8)] -> String      | a String with `n + 8` characters  |
| [[colspan=3]] Todo: Real Numbers: How about giving multiple options?  |

### Text Version: `UInt`

| Data      | Type                                              | Represents  |
|-----------|---------------------------------------------------|-------------|
| 0 ~ 86    | () -> UInt                                        | a UInt representing 0 ~ 86  |
| 87        | bits: [Char; 1] -> UInt                           | `to_uint(bits) + 87`, representing 87 ~ 181 |
| 88        | bits: [Char; 2] -> UInt                           | `to_uint(bits) + 182`, representing 182 ~ 9206 |
| 89        | bits: [Char; 3] -> UInt                           | `to_uint(bits) + 9207`, representing 9207 ~ 866581 |
| 90        | bits: [Char; 4] -> UInt                           | `to_uint(bits) + 866582`, representing 866582 ~ |
| 91        | bits: [Char; 5] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 92        | bits: [Char; 6] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 93        | bits: [Char; 7] -> UInt                           | `to_uint(bits) + `, representing ~ |
| 94        | n: UInt, bits: [Char; (n+8)] -> UInt              | `to_uint(bits) + `, representing ~ inf  |

`UInt` always represents an internal data.

### Binary Version

0 ~ 255

### Binary Version: `UInt`