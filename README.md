# SUBIMPL

Derive macros for generating required impls for a struct to redirect all calls to a field.

Currently only supports HashMaps.

This is an incredibly dirty hack, and it only exists to make my life a little easier while maintaining my main project, [nodetastic](https://github.com/aksu560/nodetastic). Feel free to steal this, and make it into an actually maintained crate, or if one already exists/this is an existing feature in rust that I do not know about, please do tell. This repository does not spark joy.