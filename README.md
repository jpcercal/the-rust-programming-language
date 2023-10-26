# the-rust-programming-language

Affectionately nicknamed “the book,” The Rust Programming Language will give you an overview of the language from first principles. You’ll build a few projects along the way, and by the end, you’ll have a solid grasp of the language.

## Notes

### 1. Getting Stated

#### Hello, Cargo!

##### Note about the chapter recap

Let’s recap what we’ve learned so far about Cargo:

We can create a project using cargo new.
We can build a project using cargo build.
We can build and run a project in one step using cargo run.
We can build a project without producing a binary to check for errors using cargo check.
Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

##### Note about Building for Release

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations.

### 2. Programming a Guessing Game

#### Note about the Result type values

Values of the Result type, like values of any type, have methods defined on them. An instance of Result has an expect method that you can call. If this instance of Result is an Err value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an Err, it would likely be the result of an error coming from the underlying operating system. If this instance of Result is an Ok value, expect will take the return value that Ok is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.
If you don’t call expect, the program will compile, but you’ll get a warning:

#### Note about the using a crate

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code that is intended to be used in other programs and can’t be executed on its own.

#### Note about the using a dependency

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the registry, which is a copy of data from Crates.io. Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the [dependencies] section and downloads any crates listed that aren’t already downloaded. In this case, although we only listed rand as a dependency, Cargo also grabbed other crates that rand depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

#### Note about the generation of documentation locally for your app

Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser. If you’re interested in other functionality in the rand crate, for example, run cargo doc --open and click rand in the sidebar on the left.

#### Note about the Ordering enum type used with cmp (comparison) and the match statement

The Ordering type is another enum and has the variants Less, Greater, and Equal. These are the three outcomes that are possible when you compare two values.

```
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features: they let you express a variety of situations your code might encounter and they make sure you handle them all. These features will be covered in detail in Chapter 6 and Chapter 18, respectively.

When the code compares 50 to 38, the cmp method will return Ordering::Greater because 50 is greater than 38. The match expression gets the Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is Ordering::Greater, which does match Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The match expression ends after the first successful match, so it won’t look at the last arm in this scenario.

### 3. Common Programming Concepts

#### Data Types

##### Note about the syntax of numbers in thousands

Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.

#### Functions

##### Note about the naming conventions

Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

##### Note about the explicit data type definition

In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.

### 4. Understanding Ownership

#### What Is Ownership?

##### Note about the Rust's goal

Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. Therefore Rust does not check at runtime whether a variable is defined before being used. Instead, Rust checks at compile-time.

##### Note about the place where variables live

Frames are organized into a stack of currently-called-functions. For example, at L2 the frame for main sits above the frame for the called function plus_one. After a function returns, Rust deallocates the function's frame. (Deallocation is also called freeing or dropping, and we use those terms interchangeably.) This sequence of frames is called a stack because the most recent frame added is always the next frame freed.


```
fn main() {
    let n = 5;
    let y = plus_one(n);
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

#### References and Borrowing

##### Note about the Simultaneous Aliasing and Mutation

Pointers are a powerful and dangerous feature because they enable aliasing. Aliasing is accessing the same data through different variables. On its own, aliasing is harmless. But combined with mutation, we have a recipe for disaster. One variable can "pull the rug out" from another variable in many ways, for example:

- By deallocating the aliased data, leaving the other variable to point to deallocated memory.
- By mutating the aliased data, invalidating runtime properties expected by the other variable.
- By concurrently mutating the aliased data, causing a data race with nondeterministic behavior for the other variable.

##### Note about the Pointer Safety Principle

```
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
v.push(4);
println!("Third element is {}", *num);
```

Initially, v points to an array with 3 elements on the heap. Then num is created as a reference to the third element, as seen at L1. However, the operation v.push(4) resizes v. The resize will deallocate the previous array and allocate a new, bigger array. In the process, num is left pointing to invalid memory. Therefore at L3, dereferencing *num reads invalid memory, causing undefined behavior.

In more abstract terms, the issue is that the vector v is both aliased (by the reference num) and mutated (by the operation v.push(4)). So to avoid these kinds of issues, Rust follows a basic principle:

Pointer Safety Principle: data should never be aliased and mutated at the same time.

Data can be aliased. Data can be mutated. But data cannot be both aliased and mutated. For example, Rust enforces this principle for boxes (owned pointers) by disallowing aliasing. Assigning a box from one variable to another will move ownership, invalidating the previous variable. Owned data can only be accessed through the owner — no aliases.

##### Note about the 



##### Note about the 

##### Note about the 

##### Note about the 
