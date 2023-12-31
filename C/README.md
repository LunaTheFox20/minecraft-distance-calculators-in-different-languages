# Point Distance Calculator

This C program calculates the distance between two 3D points using either the Euclidean distance or the Manhattan distance method. It prompts the user to enter the coordinates of three points and the distance calculation method.

## Table of Contents

- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Method Selection](#method-selection)
- [Examples](#examples)
- [License](#license)
- [Author](#author)

## Getting Started

### Prerequisites

Make sure you have a C compiler installed on your system. You can use GCC (GNU Compiler Collection) on Unix-based systems or MinGW on Windows.

### Installation

1. Clone the repository or download the C code.

   ```bash
   git clone https://github.com/LunaTheFox20/minecraft-distance-calculators-in-different-languages.git
   ```

2. Navigate to the project directory.

   ```bash
   cd minecraft-distance-calculators-in-different-languages/C
   ```

3. Compile the C code.

   - Using GCC:

     ```bash
     gcc -o main main.c
     ```

   - Using MinGW on Windows:

     ```bash
     gcc -o main.exe main.c
     ```

## Usage

Run the program by executing the compiled binary:

```bash
./main   # On Unix-based systems
main.exe # On Windows
```

## Method Selection

You can choose one of two distance calculation methods:

- **Euclidean**: Calculates the [Euclidean distance](https://github.com/LunaTheFox20/minecraft-distance-calculator#euclidean) between two points in 3D space.
- **Manhattan**: Calculates the [Manhattan distance](https://github.com/LunaTheFox20/minecraft-distance-calculator#manhattan) between two points in 3D space.

## Examples

1. Calculate the Euclidean distance between two points:

   ```
   Enter coordinates for Point 1 in the format 'x y z': 1.0 2.0 3.0
   Enter coordinates for Point 2 in the format 'x y z': 4.0 5.0 6.0
   Enter the distance calculation method (euclidean or manhattan): euclidean
   The euclidean distance between the two points is: 5
   ```

2. Calculate the Manhattan distance between two points:

   ```
   Enter coordinates for Point 1 in the format 'x y z': 1.0 2.0 3.0
   Enter coordinates for Point 2 in the format 'x y z': 4.0 5.0 6.0
   Enter the distance calculation method (euclidean or manhattan): manhattan
   The manhattan distance between the two points is: 9
   ```

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/LunaTheFox20/minecraft-distance-calculators-in-different-languages/blob/main/LICENSE) file for details.

## Author

- **LunaTheFox20**
- GitHub: [LunaTheFox20](https://github.com/LunaTheFox20)

Feel free to contribute to the project or report issues if you encounter any problems. Enjoy calculating distances!
