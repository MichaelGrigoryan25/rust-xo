# Rust ❌⭕

An imperfect tic-tac-toe game made completely with Rust as a learning project made completely from scratch.

## Getting Started

- Clone the repo

```bash
git clone https://github.com/MichaelGrigoryan25/rust-xo.git
cd rust-xo
```

- Run following commands

```bash
cargo build
cargo check
cargo run
```

## Playing the game

You are player `O`. You make a move after the computer(`X`).

After the computer has made a move your command line will look like this

```bash
CPU Chose Position: (1, 1)
_ | _ | _
_ | X | _
_ | _ | _

Enter the position(y, x):

```

The first box has the position of (0 <- Y axis, 0 <- X axis), thus all the positions are limited to have the maximum value of `2`.

Let's say that we want to move to the center. In that case we'll just enter

```bash
Enter the position(y, x):
1,1
```

The positions must be separated with a comma(`,`) otherwise the game won't work

## License

```
MIT License

Copyright (c) 2021 Michael Grigoryan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```
