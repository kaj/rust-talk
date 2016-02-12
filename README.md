# Rust

A programming language with nice semantics, thread safety, helpful
compiler messages, depencency handling (and performance).

https://www.rust-lang.org/

!

# Featuring
* zero-cost abstractions
* move semantics
* guaranteed memory safety
* threads without data races
* trait-based generics
* pattern matching
* type inference
* minimal runtime
* efficient C bindings

!
# No null pointers
But explicitly optional values where needed.

A reference (pointer) cannot be null, but you can have an Option&lt;&T>.

It's not possible to access an the value of an optional value without
checking that it exists.

!
# No garbage – no gc!
Each value has an owner.  It can be moved (ownership transfer).
If I send a value as a function parameter, I have given it away to
that function and can't use it anymore.

When something owned goes out of scope, it is destructed.

!
# Ownership rules

1. Every value has a single owner at any given time.

2. You can borrow a reference to a value, so long as the reference
doesn’t outlive the value (or equivalently, its owner).

3. You can only modify a value when you have exclusive access to it.

Bonus: A file is always closed, a mutex is always released, etc.

!
# No buffer overruns

A pointer is not an array / slice.
Arrays and slices have known number of elemets, and access is checked.
Out of bounds access panics the thread.

!
# Multithreading

!
# Simple
Just spawn() a closure.
You can join() the result later.

A scoped() threads joins automatically when out of scope.

!
# Mutex

A mutex owns its data.
To borrow the data you have to lock it.
Locking returns a MutexGuard.
When it goes out of scope, the mutex is unlocked again.

!
# Channels

The channel function returns Sender and a Receiver of given type.
The sender can be cloned, for many-to-one messaging.
Ownership is transfered through the channel.

You can send any object, even closures!

````
for msg in reciever {
    ...
}
```

!
# Hygienic macros

!
# Algebraic types

!
# Time to code!

    cargo new --bin hello_world
