## src/mocks/class.cpp:

### Inheritance:

```mermaid
classDiagram
x
class ClassA
ClassA : +public_method()
ClassA : -private_method()
```
### Methods:

#### ClassA:

```mermaid
flowchart TB
0x7f9350081720["public_method"]
0x7f9350081720 --> 0x7f93500819c0
0x7f93500819c0["int x = 43;"]
0x7f93500819c0 --> 0x7f9350081720_out
0x7f9350081720_out["end"]
```
```mermaid
flowchart TB
0x7f9350081838["private_method"]
0x7f9350081838 --> 0x7f9350081838_out
0x7f9350081838_out["end"]
```
## src/mocks/class_struct_union.cpp:

### Inheritance:

```mermaid
classDiagram
x
class ClassShouldBeIncluded
```
### Methods:

#### ClassShouldBeIncluded:

## src/mocks/hello_world.cpp:

### Inheritance:

```mermaid
classDiagram
x
```
### Methods:

## src/mocks/incomplete_class.cpp:

### Inheritance:

```mermaid
classDiagram
x
class IncompleteClassIncludedOnce
```
### Methods:

#### IncompleteClassIncludedOnce:

## src/mocks/inheritance.cpp:

### Inheritance:

```mermaid
classDiagram
x
class Animal
Animal : +int age
Animal : +char[20] gender
Animal : +isMammal()
Animal : +mate()
class Duck
Animal <|-- Duck
Duck : +char[20] beakColor
Duck : +swim()
Duck : +quack()
class Fish
Animal <|-- Fish
Fish : -int sizeInFeet
Fish : -canEat()
class Zebra
Animal <|-- Zebra
Zebra : -bool is_wild
Zebra : -run()
```
### Methods:

#### Animal:

```mermaid
flowchart TB
0x7f966401f478["isMammal"]
0x7f966401f478 --> 0x7f966401f478_out
0x7f966401f478_out["end"]
```
```mermaid
flowchart TB
0x7f966401f568["mate"]
0x7f966401f568 --> 0x7f966401f568_out
0x7f966401f568_out["end"]
```
#### Duck:

```mermaid
flowchart TB
0x7f966401f950["swim"]
0x7f966401f950 --> 0x7f966401f950_out
0x7f966401f950_out["end"]
```
```mermaid
flowchart TB
0x7f966401fa10["quack"]
0x7f966401fa10 --> 0x7f966401fa10_out
0x7f966401fa10_out["end"]
```
#### Fish:

```mermaid
flowchart TB
0x7f966401fdb8["canEat"]
0x7f966401fdb8 --> 0x7f966401fdb8_out
0x7f966401fdb8_out["end"]
```
#### Zebra:

```mermaid
flowchart TB
0x7f966403eb50["run"]
0x7f966403eb50 --> 0x7f966403eb50_out
0x7f966403eb50_out["end"]
```
## src/mocks/inner_class.cpp:

### Inheritance:

```mermaid
classDiagram
x
class innerClass
```
### Methods:

#### innerClass:

## src/mocks/rich_method.cpp:

### Inheritance:

```mermaid
classDiagram
x
class ClassA
ClassA : -rich_method()
```
### Methods:

#### ClassA:

```mermaid
flowchart TB
0x7fbb3000cd00["rich_method"]
0x7fbb3000cd00 --> 0x7fbb3000ce90
0x7fbb3000ce90["int a = 2;"]
0x7fbb3000cf48["int b = 3;"]
0x7fbb3000ce90 --> 0x7fbb3000cf48
0x7fbb3000d090["b = b * 5 - 6 * a"]
0x7fbb3000cf48 --> 0x7fbb3000d090
0x7fbb3000d1c0["int c = a + b;"]
0x7fbb3000d090 --> 0x7fbb3000d1c0
0x7fbb3000d2c8["bool d = c > 4;"]
0x7fbb3000d1c0 --> 0x7fbb3000d2c8
0x7fbb3000d3a8_con(( ))
0x7fbb3000d3a8_brk(( ))
0x7fbb3000d3a8{"a > 0"}
0x7fbb3000d3a8_con --> 0x7fbb3000d3a8
0x7fbb3000d3a8 -->|"No"| 0x7fbb3000d3a8_brk
0x7fbb3000d3a8 -->|"Yes"| 0x7fbb3000d378
0x7fbb3000d378["a--"]
0x7fbb3000d378 --> 0x7fbb3000d3a8_con
0x7fbb3000d2c8 --> 0x7fbb3000d3a8_con
0x7fbb3000d3f8_con(( ))
0x7fbb3000d3f8_brk(( ))
0x7fbb3000d3f8{"true"}
0x7fbb3000d3f8 -->|"Yes"| 0x7fbb3000d3f8_con
0x7fbb3000d3f8 -->|"No"| 0x7fbb3000d3f8_brk
0x7fbb3000d3f8_con --> 0x7fbb3000d3c8
0x7fbb3000d3c8["break"]
0x7fbb3000d3c8 --> 0x7fbb3000d3f8_brk
0x7fbb3000d3a8_brk --> 0x7fbb3000d3f8_con
0x7fbb3000d5d0_con(( ))
0x7fbb3000d5d0_brk(( ))
0x7fbb3000d5d0{{"int i = 1; i < 8; ++i"}}
0x7fbb3000d5d0_con --> 0x7fbb3000d5d0
0x7fbb3000d5d0 --> 0x7fbb3000d5d0_brk
0x7fbb3000d5d0 --> 0x7fbb3000d5a0
0x7fbb3000d5a0["c++"]
0x7fbb3000d5a0 --> 0x7fbb3000d5d0_con
0x7fbb3000d3f8_brk --> 0x7fbb3000d5d0_con
0x7fbb3000d8f0{"a < c"}
0x7fbb3000d8f0_out(( ))
0x7fbb3000d8f0 -->|"Yes"| 0x7fbb3000d6d8
0x7fbb3000d6d8["b += 2"]
0x7fbb3000d6d8 --> 0x7fbb3000d8f0_out
0x7fbb3000d8f0 -->|"No"| 0x7fbb3000d8c0
0x7fbb3000d8c0{"a == c"}
0x7fbb3000d8c0_out(( ))
0x7fbb3000d8c0 -->|"Yes"| 0x7fbb3000d7f0
0x7fbb3000d7f0["b += 1"]
0x7fbb3000d7f0 --> 0x7fbb3000d8c0_out
0x7fbb3000d8c0 -->|"No"| 0x7fbb3000d878
0x7fbb3000d878["b -= 1"]
0x7fbb3000d878 --> 0x7fbb3000d8c0_out
0x7fbb3000d8c0_out --> 0x7fbb3000d8f0_out
0x7fbb3000d5d0_brk --> 0x7fbb3000d8f0
0x7fbb3000d958["return c"]
0x7fbb3000d958 --> 0x7fbb3000cd00_out
0x7fbb3000d8f0_out --> 0x7fbb3000d958
0x7fbb3000cd00_out["end"]
```
