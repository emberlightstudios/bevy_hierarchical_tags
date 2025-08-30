# Rust Hierarchical Tags

This simple crate implements a hierarchical tag system using only rust's type system to represent nodes in the tag tree.
This should be more performant than using things like string tags with tries.
It is very simple to use.  See the example.

Max depth of a tag is 6 levels, although it's easily extensible.
TagLists allocate space for 16 tags on the stack before deferring to heap.
