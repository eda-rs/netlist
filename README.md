# netlist
Low level library-independent data structure for VLSI design

## Purpose

netlist is a common structure in VLSI design, especially in logical synthesis, P&R, formal verification, STA.

This crate wants to abstract netlist to a generic style for more common use.


## Limitation
* Not full verilog2001 featured verilog parser. Currently only support single module design and bus-based wire/port declare not supported
