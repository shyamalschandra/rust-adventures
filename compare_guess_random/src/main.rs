// Author: Shyamal S. Chandra
// Date: May 12, 2018
// Modified Last: May 12, 2018

/*
There is built-in support for a random number generator (RNG) associated with each thread stored in thread-local storage. This RNG can be accessed via thread_rng, or used implicitly via random. This RNG is normally randomly seeded from an operating-system source of randomness, e.g. /dev/urandom on Unix systems, and will automatically reseed itself from this source after generating 32 KiB of random data.
Source: https://crates.io/crates/rand
*/
extern crate rand;

/*
The std::io module contains a number of common things you'll need when doing input and output. The most core part of this module is the Read and Write traits, which provide the most general interface for reading and writing input and output.
Source: https://doc.rust-lang.org/std/io/
*/
use std::io;

// An Ordering is the result of a comparison between two values.
// Source: https://doc.rust-lang.org/std/cmp/enum.Ordering.html
use std::cmp::Ordering;

// A random number generator.
// Source: https://doc.rust-lang.org/1.1.0/rand/trait.Rng.html
use rand::Rng;

// The main function is special: it is always the first code that is run for every executable Rust program. 
// Source: https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html
fn main() {

     // Macro for printing to the standard output.
     // Source: https://doc.rust-lang.org/1.0.0/std/macro.println!.html
     println!("Guess the number!");

     /*
     The rand::thread_rng function will give us the particular random number generator that we’re going to 
     use: one that is local to the current thread of execution and seeded by the operating system. Next, we 
     call the gen_range method on the random number generator. This method is defined by the Rng trait that 
     we brought into scope with the use rand::Rng statement. The gen_range method takes two numbers as arguments 
     and generates a random number between them. It’s inclusive on the lower bound but exclusive on the upper bound, 
     so we need to specify 1 and 101 to request a number between 1 and 100.
     Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
     */
     let secret_number = rand::thread_rng().gen_range(1, 101);

     /*
     This line prints the string we saved the user’s input in. The set of curly brackets, {}, is a placeholder: 
     think of {} as little crab pincers that hold a value in place. You can print more than one value using curly 
     brackets: the first set of curly brackets holds the first value listed after the format string, the second set 
     olds the second value, and so on.
     Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
     */
     println!("The secret number is: {}", secret_number);

     /*
     The second important detail is the println! call. This code is calling a Rust macro. If it were calling a 
     function instead, it would be entered as println (without the !). We’ll discuss Rust macros in more detail 
     in Appendix D, but for now you just need to know that when you see a ! that means that you’re calling a macro 
     instead of a normal function.
     Source: https://doc.rust-lang.org/book/second-edition/ch01-02-hello-world.html
     */
     println!("Please input your guess.");

     /*
     Now you know that let mut guess will introduce a mutable variable named guess. On the other side of the
     equal sign (=) is the value that guess is bound to, which is the result of calling String::new, a function
     that returns a new instance of a String. String is a string type provided by the standard library that is a
     growable, UTF-8 encoded bit of text.

     The :: syntax in the ::new line indicates that new is an associated function of the String type. An 
     associated function is implemented on a type, in this case String, rather than on a particular instance 
     of a String. Some languages call this a static method.

     This new function creates a new, empty string. You’ll find a new function on many types, because it’s a 
     common name for a function that makes a new value of some kind.

     To summarize, the let mut guess = String::new(); line has created a mutable variable that is currently 
     bound to a new, empty instance of a String. Whew!
     Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
     */
     let mut guess = String::new();

     /*
     If we hadn’t listed the use std::io line at the beginning of the program, we could have written this 
     function call as std::io::stdin. The stdin function returns an instance of std::io::Stdin, which is a 
     type that represents a handle to the standard input for your terminal.

     The next part of the code, .read_line(&mut guess), calls the read_line method on the standard input 
     handle to get input from the user. We’re also passing one argument to read_line: &mut guess.

     The job of read_line is to take whatever the user types into standard input and place that into a string, 
     so it takes that string as an argument. The string argument needs to be mutable so the method can change 
     the string’s content by adding the user input.

     The & indicates that this argument is a reference, which gives you a way to let multiple parts of your 
     code access one piece of data without needing to copy that data into memory multiple times. References 
     are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. 
     You don’t need to know a lot of those details to finish this program. For now, all you need to know is 
     that like variables, references are immutable by default. Hence, you need to write &mut guess rather 
     than &guess to make it mutable. 
     Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
     */
     io::stdin().read_line(&mut guess)
	 .expect("Failed to read line");

	 /*
	 However, one long line is difficult to read, so it’s best to divide it: two lines for two method calls. 
	 Now let’s discuss what this line does.
	 As mentioned earlier, read_line puts what the user types into the string we’re passing it, but it also 
	 returns a value—in this case, an io::Result. Rust has a number of types named Result in its standard 
	 library: a generic Result as well as specific versions for submodules, such as io::Result.

	 The Result types are enumerations, often referred to as enums. An enumeration is a type that can have 
	 a fixed set of values, and those values are called the enum’s variants. Chapter 6 will cover enums in 
	 more detail.

	 For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful, and 
	 inside Ok is the successfully generated value. The Err variant means the operation failed, and Err 
	 contains information about how or why the operation failed.

	 The purpose of these Result types is to encode error-handling information. Values of the Result type, 
	 like any type, have methods defined on them. An instance of io::Result has an expect method that you 
	 can call. If this instance of io::Result is an Err value, expect will cause the program to crash and 
	 display the message that you passed as an argument to expect. If the read_line method returns an Err, 
	 it would likely be the result of an error coming from the underlying operating system. If this instance 
	 of io::Result is an Ok value, expect will take the return value that Ok is holding and return just that 
	 value to you so you can use it. In this case, that value is the number of bytes in what the user entered 
	 into standard input.
	 Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html
	 */
     let guess: u32 = guess.trim().parse()
	 .expect("Please type a number!");

	 /*
	 This line prints the string we saved the user’s input in. The set of curly brackets, {}, is a placeholder:
	 think of {} as little crab pincers that hold a value in place. You can print more than one value using 
	 curly brackets: the first set of curly brackets holds the first value listed after the format string, the
	  second set holds the second value, and so on.
	 Source: 
	 */
     println!("You guessed: {}", guess);

     /*
     A match expression is made up of arms. An arm consists of a pattern and the code that should be run if 
     the value given to the beginning of the match expression fits that arm’s pattern. Rust takes the value 
     given to match and looks through each arm’s pattern in turn. The match construct and patterns are powerful
     features in Rust that let you express a variety of situations your code might encounter and make sure that
     you handle them all. These features will be covered in detail in Chapter 6 and Chapter 18, respectively.
     Source: https://doc.rust-lang.org/book/second-edition/ch02-00-guessing-game-tutorial.html#printing-values-with-println-placeholders
	 
     Let’s walk through an example of what would happen with the match expression used here. Say that the user 
     as guessed 50 and the randomly generated secret number this time is 38. When the code compares 50 to 38, 
     the cmp method will return Ordering::Greater, because 50 is greater than 38. The match expression gets the
     Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, 
     Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores 
     the code in that arm and moves to the next arm. The next arm’s pattern, Ordering::Greater, does match 
     Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The 
     match expression ends because it has no need to look at the last arm in this scenario.
     */
     match guess.cmp(&secret_number) {
	      Ordering::Less => println!("Too small!"),
	      Ordering::Greater => println!("Too big!"),
	      Ordering::Equal => println!("You win!"),
     }
}
