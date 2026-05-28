# rubiks_cube_simulation
Rubiks Cube Simulation written in [rust](https://rust-lang.org/) with [three-d](https://docs.rs/crate/three-d).

## Controls
All rotations are made Clockwise. Hold `Shift` to make Inverse (Prime/Counterclockwise) rotation.

### Face Turns:

Supports Wide rotations while holding `Alt`.
|Keyboard Button|Action|
|-|-|
|`U`|Rotate Up Face|
|`D`|Rotate Down Face|
|`R`|Rotate Right Face|
|`L`|Rotate Left Face|
|`F`|Rotate Front Face|
|`B`|Rotate Back Face|

### Slice Moves
|Keyboard Button|Action|
|-|-|
|`M`|Rotate Middle Slice|
|`E`|Rotate Equatorian Slice|
|`S`|Rotate Standing Slice|

### Cube Rotations

Rotate whole cube.
|Keyboard Button|Action|
|-|-|
|`X`|Rotation around the X axis|
|`Y`|Rotation around the Y axis|
|`Z`|Rotation around the Z axis|

### History manipulation

Hold `Ctrl` to move to the first/last move.
|Keyboard Button|Action|
|-|-|
|`Left Arrow`|Undo last move|
|`Right Arrow`|Redo last undone move|

### Miscellaneous
|Keyboard Button|Action|
|-|-|
|`Tab`|Show axes|

Image for reference:

<img src="https://jperm.net/images/notation.png" alt="rotation.png" style="width:50%; height:auto;">

## License: [MIT](https://github.com/kerrytazi/rubiks_cube_simulation/blob/main/LICENSE)
