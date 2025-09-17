# Bevy Hierarchical Tags

This simple crate implements a hierarchical tag system for the bevy game engine.  It is very easy to use.  It provides a TagRegistry Resource and some simple apis for querying against it.  See example.

I am no longer using bitvec to store bitmasks for tag matching because it is heap allocated.  Now each TagNode gets an inline array of u64 to store custom bitmasks.  This should lead to better performance via fewer indirections.

The number of tags is no longer generic.  By default you can have up to 512 tags.  There is a feature flag to take this to 1024 if needed.

A 512-bit bitmask is 64 bytes.  If you use all 512 tags this adds up to a memory footprint of almost 33kB.
For 1024 tags, the footprint increases to 131kB.  