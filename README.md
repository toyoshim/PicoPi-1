# PicoPi-1
PDP-1 emulator, running on Raspberry Pi Pico, written in Rust.
See my [tweet](https://twitter.com/toyoshim/status/1591306173504188417?s=20&t=RCzo7QGhFZNQS9Ckd0akRA) to know how it looks like.

## Repository
This repository contains a KiCAD project, schematics and gerber data, for  the
DAC to covert X/Y CRT position in 10-bits ditial to analog signals so that
oscilloscopes can show picture in the XY mode.

Also, you can find a firmware under the `firmware` directory. It is written in
Rust, and you may need some setup to build it. This document doesn't explain
details (because I remember only partial setups...).

The project expects VS Code with some Extensions to develop ARM based processors
with openocd.

## TODO
- Speed control: now the emulation core counts cycles, but doesn't synchronized
with real timer. So, it runs a little faster than the original PDP-1.
- Game controller: yes, I know what I should do, but didn't implement it yet.
See my TODO comment in pdp1.rs's IOT instruction handling.
- Snowflake support: PDP-1 emulation core may have a bug around ALU as the
snowflake shows broken images. I have a working C++ version. So, it can be fixed
soon.
- Rust readability: as I'm new to Rust, the code still doesn't look so nice.
- Documents: agreed...

*CATS: ALL YOUR `PR` ARE WELCOMED TO US.*