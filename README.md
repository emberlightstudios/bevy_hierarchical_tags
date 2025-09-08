# Rust Hierarchical Tags

This simple crate implements a hierarchical tag system for the bevy game engine.  It is very easy to use.  It provides a TagRegistry Resource and some simple apis for querying against it.  See example.

It uses Smallbitvec to store a bitmask for each tag node for extremely fast tag matching.  This comes at the cost of ram.  I'm using smallbitvec to allocate 128 bits inline on each tag node, assuming just under 128 individual tags will be in use.  You can of course go beyond this number but it will involve heap allocations.  Consider adjusting this number for your individual needs.
