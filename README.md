# Rust Hierarchical Tags

This simple crate implements a hierarchical tag system for the bevy game engine.  It is very easy to use.  It provides a TagRegistry Resource and some simple apis for querying against it.  See example.

It uses Smallbitvec to store a bitmask for each tag node for extremely fast tag matching.  This comes at the cost of ram.  I'm using smallbitvec to allocate 1024 bits per node on the stack.  This should be 128k in total, which is still less than a single texture.  You can of course go beyond this number but it will involve heap allocations.  Consider adjusting this number for your individual needs.