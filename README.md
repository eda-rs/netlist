# netlist
Low level library-independent data structure for VLSI design


## Purpose

netlist is a common structure in VLSI design, especially in logical synthesis, P&R, formal verification, STA.

This crate wants to abstract netlist to a generic style for more common use.



## Feature
**1. graph-like data structure**


**2. verilog parser**
The verilog parser in this crate is a minimal subset of verilog-2001, which can parse structural verilog syntax into netlist. 

**3. verilog saver**
Save netlist as verilog.

## Limitation
* 
