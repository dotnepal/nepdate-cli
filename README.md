# nepdate-cli

`nepdate-cli` is a simple command-line program built using the `bikram` library to convert dates between Bikram Sambat (Nepali calendar) and Gregorian dates.

## Installation and Setup

### Prerequisites

- [Rust](https://www.rust-lang.org/) programming language
- Cargo (Rust package manager). install instruction for rust [HERE](https://www.rust-lang.org/tools/install).

### Clone the Repository

First, clone the repository:

```bash
git clone https://github.com/dotnepal/nepdate-cli
cd nepdate-cli
```

### Build the Program
- Run `cargo build --release`

This script will:
- Compile the program in release mode and store the output in the `target` directory.
- Display the path to the executable.
- Test the program by running it.

## Install using cargo 

```cargo install nepdate-cli ```
Running the above command will globally install the nepdate-cli binary.

## Usage

Get Todya Nepali date
- `nepdate-cli --today-nepali`

OUTPUT: `2 Ashad, 2082`

Get Today English date
- `nepdate-cli --today-english`

OUTPUT: `6/16/2025 (m/d/Y)`

### License

nepdate-cli is released under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html). See the LICENSE file for more details.
