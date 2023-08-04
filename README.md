# ConcreteSTV to raire-rs file format converter

This project converts ConcreteSTV .stv files to the input format to raire-rs. 

# Compiling

ConcreteSTVToRaire is written in [Rust](https://www.rust-lang.org/). Install Rust (latest stable version
recommended). Then:
* Clone this repository, and also the ConcreteSTV and raire-rs repositories. All three repositories should
  be at the same position in the directory hierarchy. 
* Compile this project with `cargo build --release`

An example under Linux, assuming you have Rust already installed would be
```bash
git clone https://github.com/AndrewConway/ConcreteSTV.git
git clone https://github.com/AndrewConway/ConcreteSTVToRaire.git
git clone https://github.com/DemocracyDevelopers/raire-rs.git
cd ConcreteSTVToRaire
cargo build --release
```

# Running

```bash
./target/release/concrete_stv_to_raire path/to/your/ConcreteSTV/file.stv
```


## Copyright

This program is Copyright 2023 Andrew Conway.

This file is part of ConcreteSTVToRaire.

ConcreteSTVToRaire is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

ConcreteSTVToRaire is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with ConcreteSTV.  If not, see <https://www.gnu.org/licenses/>.
