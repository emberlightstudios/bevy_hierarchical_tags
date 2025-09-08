# Bevy Hierarchical Tags

This simple crate implements a hierarchical tag system for the bevy game engine.  It is very easy to use.  It provides a TagRegistry Resource and some simple apis for querying against it.  See example.

It uses Bitvec to store a bitmask for each tag node for fastest tag matching.  This does come at the cost of ram, but should by much faster until you hit an unreasonably large number of tags.  You need to specify the size of the bitmask when you create the resource.  

I'm also using smallvec to allocate 128 node structs inline on the resource, assuming at least 128 individual tags.  It is a const at the top of lib.rs.  You can of course go beyond this number but it will involve heap allocations.  Consider adjusting this number for your individual needs.  It should improve performance by avoiding pointer chasing.  Ideally it would be equal to max_nodes, but there isn't really a good way to do this because it needs to be known at compile time.  The only way around it is to use const generics on the resource itself, but this is painful in other ways, e.g. every system which accesses the resource must specify the number as a generic.  I'm open to any advice on how to improve this.
