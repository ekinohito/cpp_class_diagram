# C++ Class Diagrams

> Утилита анализа кода C++ (hpp, cpp) с формированием в формате opendoc/visio:
> схемы классов (с указанием наследования),
> схем алгоритмов каждого метода.

В качестве формата генерации схем я выбрал разметку `mermaid.js`,
так как это открытый легковесный текстовый формат описания диаграмм, который поддерживается множеством
редакторов `markdown` файлов, а также может рендериться в изображения с использованием библиотеки.

Для получения AST я использовал ast-dump компилятора clang, так как парсинг языка C++ очень сложен и только у clang он организован
более-менее логично, так что с синтаксическими деревьями можно работать.

В качестве языка программирования я выбрал Rust™️, так как его система типов позволяет быть уверенным в правильности работы программы,
а компилятор и отсутствие сборщика мусора позволяет осуществлять работу с большими файлами, такими как AST относительно быстро

Запуск:

```sh
cargo run -- <список файлов для анализа кода>
```

Генерация AST в формате JSON для создания моковых данных:

```sh
cargo run --bin generate_mock -- <имя файла для перевода в AST>
```

Сборка:

```sh
cargo build -r
```

Интерфейс:

```sh
target/release/cpp_class_diagrams <список файлов для анализа кода>
```

# Примеры работы:

## Схема наследования

```sh
# Наследование
cargo run src/mocks/inheritance.cpp
```
[Исходный код](src/mocks/inheritance.cpp)

> ## src/mocks/inheritance.cpp:
> 
> ### Inheritance:
> 
> ```mermaid
> classDiagram
> x
> class Animal
> Animal : +int age
> Animal : +char[20] gender
> Animal : +isMammal()
> Animal : +mate()
> class Duck
> Animal <|-- Duck
> Duck : +char[20] beakColor
> Duck : +swim()
> Duck : +quack()
> class Fish
> Animal <|-- Fish
> Fish : -int sizeInFeet
> Fish : -canEat()
> class Zebra
> Animal <|-- Zebra
> Zebra : -bool is_wild
> Zebra : -run()
> ```
> ### Methods:
> 
> #### Animal:
> 
> ```mermaid
> flowchart TB
> 0x7fd3c380c278["isMammal"]
> 0x7fd3c380c278 --> 0x7fd3c380c278_out
> 0x7fd3c380c278_out["end"]
> ```
> ```mermaid
> flowchart TB
> 0x7fd3c380c368["mate"]
> 0x7fd3c380c368 --> 0x7fd3c380c368_out
> 0x7fd3c380c368_out["end"]
> ```
> #### Duck:
> 
> ```mermaid
> flowchart TB
> 0x7fd3c380c750["swim"]
> 0x7fd3c380c750 --> 0x7fd3c380c750_out
> 0x7fd3c380c750_out["end"]
> ```
> ```mermaid
> flowchart TB
> 0x7fd3c380c810["quack"]
> 0x7fd3c380c810 --> 0x7fd3c380c810_out
> 0x7fd3c380c810_out["end"]
> ```
> #### Fish:
> 
> ```mermaid
> flowchart TB
> 0x7fd3c380cbb8["canEat"]
> 0x7fd3c380cbb8 --> 0x7fd3c380cbb8_out
> 0x7fd3c380cbb8_out["end"]
> ```
> #### Zebra:
> 
> ```mermaid
> flowchart TB
> 0x7fd3c2833f50["run"]
> 0x7fd3c2833f50 --> 0x7fd3c2833f50_out
> 0x7fd3c2833f50_out["end"]
> ```

## Схема алгоритма

```sh
# Схема алгоритма
cargo run src/mocks/rich_method.cpp
```

[Исходный код](src/mocks/rich_method.cpp)

> ## src/mocks/rich_method.cpp:
> 
> ### Inheritance:
> 
> ```mermaid
> classDiagram
> x
> class ClassA
> ClassA : -rich_method()
> ```
> ### Methods:
> 
> #### ClassA:
> 
> ```mermaid
> flowchart TB
> 0x7f815c882100["rich_method"]
> 0x7f815c882100 --> 0x7f815c882290
> 0x7f815c882290["int a = 2;"]
> 0x7f815c882348["int b = 3;"]
> 0x7f815c882290 --> 0x7f815c882348
> 0x7f815c882490["b = b * 5 - 6 * a"]
> 0x7f815c882348 --> 0x7f815c882490
> 0x7f815c8825c0["int c = a + b;"]
> 0x7f815c882490 --> 0x7f815c8825c0
> 0x7f815c8826c8["bool d = c > 4;"]
> 0x7f815c8825c0 --> 0x7f815c8826c8
> 0x7f815c8827a8_con(( ))
> 0x7f815c8827a8_brk(( ))
> 0x7f815c8827a8{"a > 0"}
> 0x7f815c8827a8_con --> 0x7f815c8827a8
> 0x7f815c8827a8 -->|"No"| 0x7f815c8827a8_brk
> 0x7f815c8827a8 -->|"Yes"| 0x7f815c882778
> 0x7f815c882778["a--"]
> 0x7f815c882778 --> 0x7f815c8827a8_con
> 0x7f815c8826c8 --> 0x7f815c8827a8_con
> 0x7f815c8827f8_con(( ))
> 0x7f815c8827f8_brk(( ))
> 0x7f815c8827f8{"true"}
> 0x7f815c8827f8 -->|"Yes"| 0x7f815c8827f8_con
> 0x7f815c8827f8 -->|"No"| 0x7f815c8827f8_brk
> 0x7f815c8827f8_con --> 0x7f815c8827c8
> 0x7f815c8827c8["break"]
> 0x7f815c8827c8 --> 0x7f815c8827f8_brk
> 0x7f815c8827a8_brk --> 0x7f815c8827f8_con
> 0x7f815c8829d0_con(( ))
> 0x7f815c8829d0_brk(( ))
> 0x7f815c8829d0{{"int i = 1; i < 8; ++i"}}
> 0x7f815c8829d0_con --> 0x7f815c8829d0
> 0x7f815c8829d0 --> 0x7f815c8829d0_brk
> 0x7f815c8829d0 --> 0x7f815c8829a0
> 0x7f815c8829a0["c++"]
> 0x7f815c8829a0 --> 0x7f815c8829d0_con
> 0x7f815c8827f8_brk --> 0x7f815c8829d0_con
> 0x7f815c882cf0{"a < c"}
> 0x7f815c882cf0_out(( ))
> 0x7f815c882cf0 -->|"Yes"| 0x7f815c882ad8
> 0x7f815c882ad8["b += 2"]
> 0x7f815c882ad8 --> 0x7f815c882cf0_out
> 0x7f815c882cf0 -->|"No"| 0x7f815c882cc0
> 0x7f815c882cc0{"a == c"}
> 0x7f815c882cc0_out(( ))
> 0x7f815c882cc0 -->|"Yes"| 0x7f815c882bf0
> 0x7f815c882bf0["b += 1"]
> 0x7f815c882bf0 --> 0x7f815c882cc0_out
> 0x7f815c882cc0 -->|"No"| 0x7f815c882c78
> 0x7f815c882c78["b -= 1"]
> 0x7f815c882c78 --> 0x7f815c882cc0_out
> 0x7f815c882cc0_out --> 0x7f815c882cf0_out
> 0x7f815c8829d0_brk --> 0x7f815c882cf0
> 0x7f815c882d58_brk(( ))
> 0x7f815c882d58{"a"}
> 0x7f815c882db8(( ))
> 0x7f815c882d58 -->|"1"| 0x7f815c882db8
> 0x7f815c882db8 --> 0x7f815c89cc00
> 0x7f815c89cc00["a++"]
> 0x7f815c89cc18["break"]
> 0x7f815c89cc18 --> 0x7f815c882d58_brk
> 0x7f815c89cc00 --> 0x7f815c89cc18
> 0x7f815c89cc58(( ))
> 0x7f815c882d58 -->|"2"| 0x7f815c89cc58
> 0x7f815c89cc58 --> 0x7f815c89cca0
> 0x7f815c89cca0["b++"]
> 0x7f815c89ccf0(( ))
> 0x7f815c882d58 -->|"Default"| 0x7f815c89ccf0
> 0x7f815c89ccf0 --> 0x7f815c89ccd8
> 0x7f815c89ccd8["c--"]
> 0x7f815c89cca0 --> 0x7f815c89ccf0
> 0x7f815c89cd10["break"]
> 0x7f815c89cd10 --> 0x7f815c882d58_brk
> 0x7f815c89ccd8 --> 0x7f815c89cd10
> 0x7f815c882cf0_out --> 0x7f815c882d58
> 0x7f815c89cd88["return c"]
> 0x7f815c89cd88 --> 0x7f815c882100_out
> 0x7f815c882d58_brk --> 0x7f815c89cd88
> 0x7f815c882100_out["end"]
> ```
