# Bevy Hierarchical Tags

This simple crate implements a hierarchical tag system for the bevy game engine.  It is very easy to use.  It provides a TagRegistry Resource and some simple apis for querying against it.  See example.

It uses Bitvec to store a bitmask for each tag node for fastest tag matching.  This does come at the cost of ram, but should by much faster until you hit an unreasonably large number of tags.

There is a const generic to determine the max number of tags the resource can support.  I recommend using a type alias for this so you don't have to track it all over the place, as I did in the example.