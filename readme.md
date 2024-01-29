# glam-traits
Traits for the vectors in [glam](https://github.com/bitshifter/glam-rs/).

There are traits for the following characteristics:
* "Any"
* Signed
* Float
* Int
* Signed int
* Unsigned int

For lengths "any", 2, 3 and 4:

|              |             |              |              |              |
| ------------ | ------------| ------------ | ------------ | ------------ |
|              | Any         | 2            | 3            | 4            |
| Any          | `GVec`      | `GVec2`      | `GVec3`      | `GVec4`      |
| Signed       | `SignedVec` | `SignedVec2` | `SignedVec3` | `SignedVec4` |
| Float        | `FloatVec`  | `FloatVec2`  | `FloatVec3`  | `FloatVec4`  |
| Int          | `IntVec`    | `IntVec2`    | `IntVec3`    | `IntVec4`    |
| Signed int   | `SIntVec`   | `SIntVec2`   | `SIntVec3`   | `SintVec4`   |
| Unsigned int | `UIntVec`   | `UIntVec2`   | `UIntVec3`   | `UIntVec4`   |

As well as for concrete types of any length:

`I16Vec`, `U16Vec`, `I32Vec`, `U32Vec`, `I64Vec`, `U64Vec`, `F32Vec`, `F64Vec`

Traits are implemented for the appropriate glam types.
