1. Rust applications can be created using a package manager called cargo.
```bash
cargo new web_server
```

2. To run the application, we have to run the following command.
```bash
cargo new web_server
```

1. We will be using TCP as our transport level protocol.
	_Will be using **std::net** library for creating a TCP connection_


src/main.rs
```rust
use std::net::TcpListener;
fn main() {
    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8477";

    let end_point : String = HOST.to_owned() + ":" + PORT;
    println!("Web server listening at port : {}", PORT);

    let listener = TcpListener::bind(end_point).unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("The connection has been established");
    }
}
```
`

Questions :

1. what does std mean ? what is the type of this ??
	std: In Rust, std stands for "standard." It is the standard library that provides fundamental building blocks for Rust programs. [std documentation](https://doc.rust-lang.org/std/)
 
2. what does net mean ? what is the type of this ??
	net: In the context of std::net, it refers to the networking module in the standard library.  The net module provides types and functions for networking operations. [std::net documentation](https://doc.rust-lang.org/std/net/)
 
3. what does TcpListener mean ? what is the type of this ??
	TcpListener: TcpListener is a type in Rust's standard library used for handling TCP connections.  It represents a socket server that listens for incoming TCP connections. [TcpListener documentation](https://doc.rust-lang.org/std/net/struct.TcpListener.html)

4. What does :: operator do ? Is it similar to scope resolution operator in CPP ?
	:: operator: In Rust, the :: operator is used to access items (such as functions, types, or constants) defined in a module or trait.  It is similar to the scope resolution operator in C++. In the provided code, std::net::TcpListener is using :: to access the TcpListener type within the net module.
 
5. Can I use localhost here ? Would using "127.0.0.1" work with ipv6?
	We can use localhost, won't work with ipv6

6. why do we have a & in front of str ??
	&str: The &str type represents a string slice, a reference to a sequence of UTF-8 bytes. The & in front of str indicates that HOST and PORT are string slices, borrowed references to string data.

7. what does to_owned function do ?
	to_owned creates an owned (heap-allocated) String from a borrowed (&str) slice.
 
8. What are the differences between string and slice ? 
	Learn more about Strings: [String-docs](https://doc.rust-lang.org/std/string/struct.String.html)
	Learn more about slices: [Slices-docs](https://doc.rust-lang.org/std/primitive.slice.html)
```rust
fn main() {
    // String: A String is a heap-allocated, growable sequence of characters.
    let owned_string: String = String::from("Hello, String!");

    // &str (String Slice): A string slice is a reference to a sequence of UTF-8 bytes, typically a portion of a String.
    let string_slice: &str = "Hello, String Slice!";

    // The key difference is ownership and mutability:
    // - String owns its data, and you can modify it.
    // - &str is a borrowed reference to data owned by another String, and it's usually immutable.

    // You can modify a String:
    let mut mutable_string: String = String::from("Mutable String");
    mutable_string.push_str(", Appended!");
    println!("{}", mutable_string);

    // You can't easily modify a string slice:
    // Uncommenting the line below will result in a compile-time error.
    // string_slice.push_str(", Appended!"); // Error: cannot borrow `*string_slice` as mutable, as it is not declared as mutable

    // String slices are often used for borrowing parts of Strings or string literals.
    let greeting: &str = &owned_string[0..5]; // Borrowing a slice of the original String
    println!("{}", greeting); // Prints "Hello"
}

```
 
9. What is the return type of TcpListener::bind() ?
	TcpListener::bind returns a Result<TcpListener, std::io::Error>
 
10. What does the unwrap function do ?
	unwrap is used to handle the Result, panic on Err
 
11. Explain the type of _stream .. what else can I do with the _stream response ??
	_stream has type std::result::Result<std::net::TcpStream, std::io::Error>

12. What does panic mean with unwrap function ?
	`panic` is a term used to describe the program's abrupt termination due to encountering a condition that the program cannot handle. It's like a runtime crash. When a panic occurs, the program prints an error message to the standard error and then unwinds the stack, cleaning up resources along the way.

13. How to write the above function without terminating on error ? 
```rust

use std::net::TcpListener;
use std::io::{self, Write}; // Import Write for println! macro

fn main() {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    let end_point: String = format!("{}:{}", HOST, PORT); // Using format! for string concatenation
    println!("Web server listening at port : {}", PORT);

    // Using match to handle Result<TcpListener, std::io::Error>
    match TcpListener::bind(&end_point) {
        Ok(listener) => {
            for stream in listener.incoming() {
                // Using match to handle Result<std::net::TcpStream, std::io::Error>
                match stream {
                    Ok(_stream) => {
                        println!("The connection has been established");
                    }
                    Err(err) => {
                        eprintln!("Error accepting connection: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
        }
    }
}
```

14. What does match keyword stand for ?
	the `match` keyword is used for pattern matching. It allows you to compare a value against a set of patterns and execute code based on which pattern matches.
 ```rust
 fn main() {
    let number = 42;

    match number {
        0 => println!("It's zero!"),
        1 | 2 => println!("It's one or two!"),
        3..=9 => println!("It's between 3 and 9 (inclusive)!"),
        n if n % 2 == 0 => println!("It's an even number!"),
        _ => println!("It's something else!"),
    }
}
```

15. What are Ok and Err keywords ?
	In Rust, `Ok` and `Err` are not actually keywords; they are variants of the `Result` enum.
```rust
fn divide_numbers(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        // Returning Err variant for the division by zero error
        return Err("Cannot divide by zero");
    }

    // Returning Ok variant with the result of the division
    Ok(dividend / divisor)
}

fn main() {
    // Example 1: Successful division
    match divide_numbers(10, 2) {
        Ok(result) => println!("Result of division: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    // Example 2: Division by zero (error)
    match divide_numbers(5, 0) {
        Ok(result) => println!("Result of division: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

16. What are lifetime specifiers ? 
	Lifetime specifiers are a feature in Rust that help manage how long references are valid. They allow you to express the relationships between the lifetimes of various parts of your program, ensuring that references do not outlive the data they point to. Lifetimes are a critical part of Rust's ownership system, providing guarantees about memory safety without the need for garbage collection.

17. What is the difference between `println!` and `eprintln` ?
	`println!` and `eprintln!` are macros used for printing messages to the standard output (stdout) and standard error (stderr), respectively. The key difference between them lies in the destination where the output is directed:

	1. **`println!`**:
	    - This macro is used for printing messages to the standard output (stdout).
	    - Messages printed using `println!` will be displayed in the console or terminal where the Rust program is executed.
	    - Example:
	```rust
        fn main() {     println!("This message goes to stdout"); }`
	```
        
	2. **`eprintln!`**:
	    - This macro is used for printing messages to the standard error (stderr).
	    - Messages printed using `eprintln!` will also be displayed in the console or terminal, but they are typically used for error messages or other output that is considered to be separate from the normal program output.
	    - Example:
	```rust
        fn main() {     eprintln!("This is an error message sent to stderr"); }`
	```

	The primary use case for `eprintln!` is for reporting errors or other critical information to the user. When the program's standard output is redirected (e.g., to a file), messages from `eprintln!` will still be displayed in the console or terminal.
	

18. What does the & (**Ampersand**) operator do in rust ?
	1. **`&` (Ampersand) Operator**: In Rust, the `&` operator is used to create a reference to a value. It does not take ownership of the value; instead, it allows you to borrow a reference to the value.
 
19. What is TCP Stream ? 
	A TCP stream refers to a sequence of bytes sent over a TCP (Transmission Control Protocol) connection between two devices. TCP is a reliable, connection-oriented protocol that provides a reliable and ordered stream of data between two hosts.

	Here are key characteristics of a TCP stream:
	
	1. **Connection-Oriented:**
	2. **Byte Stream:**
	3. **Reliability:**
	4. **Full-Duplex Communication:**
	5. **Flow Control:**
	6. **Connection Termination:**
	7. **Stream Characteristics:**



2. Now, let's try handling the request that we get from the client .
```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
fn main() {
    const HOST : &str = "127.0.0.1";
    const PORT : &str = "8001";
    let end_point : String = format!("{}:{}", HOST, PORT);
    match TcpListener::bind(end_point) {
        Ok(listner) => {
            println!("The web-server was established over PORT :: {}", PORT);
            for stream in listner.incoming() {
                match stream {
                    Ok(stream_resp) => {
                        handle_connection(stream_resp)
                    }
                    Err(err) => {
                        eprintln!("Error establishing the connection :: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprint!("Error establishing a TCP listner :: {}", err);
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(buffer_resp) => {
            println!("Request1 {}", String::from_utf8_lossy(&buffer[..]));
            println!("Request2 {}", buffer_resp);
        }
        Err(err) => {
            eprintln!("Error reading the request :: {}", err);
        }
    }
}
```

Questions:

20. Explain the prelude module in RUST ? Is it similar to HASKELL prelude module ?
	The `std::io::prelude::*` import brings in a set of common I/O traits that are frequently used. It doesn't have the exact same connotation as Haskell's prelude but serves a similar purpose in providing a standard set of I/O traits and functionalities.

21. Is handling connections `for stream in listner.incoming()` synchronous or asynchronous ?
	The handling of clients is synchronous (blocking) here. The `for stream in listener.incoming()` loop waits for and processes incoming connections one by one. It's synchronous in the sense that it handles one client at a time before moving on to the next.

22. Why would you want to have stream as a mutable in `handle_connection(mut stream: TcpStream)`?
	The `mut` in `fn handle_connection(mut stream: TcpStream)` indicates that the `stream` variable is mutable. This allows modifying the state of the `TcpStream` within the function. In this case, it's used to read data from the stream into the `buffer`.

23. What is the difference between let and const ?
	`const` is used for true constants (values that cannot change), whereas `let` is used for variables (values that can change). In the case of `let mut stream`, it means `stream` is a mutable variable.

24. Explain this statement to me `let mut buffer = [0; 1024];` ?
	`let mut buffer = [0; 1024];` creates a mutable array (`[0; 1024]`) named `buffer`. This array is used to store the incoming data from the client, with a maximum size of 1024 bytes.

25. Explain `String.from_utf8_lossy(&buffer[..]).`
	`String` is part of the Rust prelude.
	
	`&buffer[..]` is a slice of the entire array `buffer`. The `..` specifies a range that starts at the beginning (implicit in this case) and goes up to the end of the array.
	
	`String` is a string type in Rust. `String::from_utf8_lossy(&buffer[..])` converts the content of `buffer` (assumed to be UTF-8 encoded bytes) into a `String`. `from_utf8_lossy` handles invalid UTF-8 sequences by replacing them with the Unicode replacement character.


3. Let's respond to the client now.
```rust
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Request Size: {}", bytes_read);
            let response = "HTTP/1.1 200 OK\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
        Err(err) => {
            let response = "HTTP/1.1 400 BAD-REQUEST\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            eprintln!("Error reading the request :: {}", err);
        }
    }
}
```

Questions:

26. What does the flush method do ? 
	The `flush` method ensures that all data is written to the underlying stream. It is necessary for the response to be sent immediately.
	 The `flush` method ensures that any buffered data is written to the underlying stream. In network programming, it's crucial to flush data to ensure timely delivery.

4. Let's send an HTML static file to have it render on the browser.

```rust
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::Result;
use std::fs;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8001";
const BUFFER_SIZE: usize = 1024;

fn main() {
    let end_point: String = format!("{}:{}", HOST, PORT);
    match TcpListener::bind(end_point) {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream_resp) => {
                        // let _ = list_directory(".");
                        handle_connection(stream_resp);
                    }
                    Err(err) => {
                        eprintln!("Error establishing the connection :: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            eprint!("Error establishing a TCP listener :: {}", err);
        }
    }
}


// I introduced this function to get contents of the current directory .. 
fn list_directory(path: &str) -> Result<()> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; BUFFER_SIZE];
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("Request Size: {}", bytes_read);
            match fs::read_to_string("./index.html") {
                Ok(response_contents) => {
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        response_contents.len(),
                        response_contents
                        );
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
                Err(err) => {
                    eprintln!("Error while reading the file :: {}", err);
                    let response = "HTTP/1.1 500 INTERNAL_SERVER_ERROR\r\n\r\n\r\n";
                    stream.write(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
        Err(err) => {
            eprintln!("Error Occured while reading the stream :: {}", err);
            let response = "HTTP/1.1 400 BAD_REQUEST\r\n\r\n\r\n";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
```

Questions:

27. What does the `?` operator do in RUST ?
	The `?` in Rust is like a shortcut that helps handle errors more easily. When we're doing something that might result in an error (like reading a directory), Rust uses the `Result` type to tell us if it went well or if there was a problem.
 
	Now, the `?` is like a magic trick. If there's an error, it kind of says, "Hey, something went wrong, let's not continue. Just give back this error to whoever called this function."
	
	Without `?`, we'd have to do more work to check if there's an error and handle it ourselves. The `?` makes our code shorter and cleaner.
	
	So, in short, the `?` helps Rust code deal with errors in a way that's simple and neat. If there's an issue, it says, "Stop here, I've got an error to report!"

28. What does unit return type mean ? 
	In Rust, the unit type is represented by the empty tuple `()`. It is often referred to as "unit" or "unit type." In Rust, functions that don't return a meaningful value often have a return type of `()`.

5. I want to make the web-server handle the request asynchronously . 

Questions:


28. how do I make a function asynchronous ? 
	1. Use `async` keyword.
		- Place `async` before the `fn` keyword to make a function asynchronous:
	```rust
		async fn my_async_function() -> String { // ... asynchronous code here ... }	
	```
	  2. The `async` function returns a `Future`
		- Asynchronous functions return a `Future`, a type that represents an eventual value.
		- The compiler handles this implicitly, so you don't need to specify the return type explicitly (unless you need more control).
	3.  We need `await` keyword to wait for a `Future` to complete
	 ```rust
		async fn fetch_data() -> Result<String, reqwest::Error> {
			let response = reqwest::get("https://example.com").await?;
			let text = response.text().await?;
			Ok(text) 
		}
	```
	4.  To run a `Future`:
		- You need a runtime to drive the execution of asynchronous tasks.
		- Common runtimes include <u>Tokio</u> and <u>async-std</u>.
		Example using <u>tokio</u>: 
	```rust
	#[tokio::main]
	async fn main() -> Result<(), Box<dyn std::error::Error>> {
	    let data = fetch_data().await?;
	    println!("Fetched data: {}", data);
	    Ok(())
	}
	```

6. Setting up async web-server
```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::fs;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8001";
const BUFFER_SIZE: usize = 1024;

#[tokio::main]
async fn main() {
    let end_point: String = format!("{}:{}", HOST, PORT);
    match TcpListener::bind(&end_point).await {
        Ok(listener) => {
            println!("Listening on {}", end_point);
            while let Ok((socket, _)) = listener.accept().await {
                tokio::spawn(handle_connection(socket));
            }
        }
        Err(err) => {
            eprintln!("Error binding to address: {}", err);
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0u8; BUFFER_SIZE];
    let bytes_read = stream.read(&mut buffer).await?;
    // Process the request (not implemented in this code)

    let file_content = fs::read_to_string("./index.html").await?;
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        file_content.len(),
        file_content
    );

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;

    Ok(())
}
```


Questions:

29. Why do we need to import `use tokio::io::{AsyncReadExt, AsyncWriteExt};` for the `stream.read` and `stream.write_all` functions ?
	Importing traits AsyncReadExt, AsyncWriteExt from the `tokio::io` module. These traits provide additional methods for working with asynchronous I/O.

30. What does it mean to have a value moved?
	The ownership of `end_point` is transferred (moved) it is moved to function or a variable.

31.  What does spawn function do?
	It spawns a new asynchronous task to handle the connection concurrently.

32. What is u8 type mean ?
   In Rust, `u8` is an unsigned 8-bit integer type.

33. What is the difference between write and write_all? 
	write_all ensures that the entire buffer is written, while write may not write the entire buffer at once.

34. Explain flush in detail.
	Flush ensures that all written data is sent immediately.

35. What are traits ?
	In Rust, traits are a powerful mechanism for defining shared behavior in a generic way. Traits allow you to define methods that can be implemented by various types, enabling polymorphism and code reuse.

36. how to define a trait?
	A trait defines a set of methods that can be implemented by types. It serves as a way to declare functionality without providing a default implementation.
```rust
// Example trait definition
trait Drawable {
    fn draw(&self);
}
```

37. how to implement a functionality of a struct over a trait ? 
	Types can implement traits by providing their own implementation for the methods declared in the trait.
```rust
struct Circle {
	radius: f64,
}

impl Drawable for Circle {
	fn draw(&self) {
		println!("Drawing a circle with radius {}", self.radius);
	}
}
```

38. how do traits enable polymorphism ?
	Traits enable polymorphism by allowing different types to be treated uniformly if they implement the same set of methods.
 ```rust
fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}

let circle = Circle { radius: 3.0 };
draw_shape(&circle);
```

39. How do traits provide default implementation ? 
	Traits can provide default implementations for some or all of their methods. Types implementing the trait can choose to override or use these defaults.
 ```rust
trait Greet {
    fn greet(&self) {
        println!("Hello!");
    }
}

struct Person;

impl Greet for Person {
    // No need to implement greet, using the default.
}
```

40. What are associated traits ?
	Traits can include associated types, allowing them to specify types that depend on the implementing type.
 ```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

41. how are traits used with generics ?
	Traits are often used with generics to express constraints on the types that can be used with a generic function or struct.
```rust
fn print_and_draw<T: Display + Drawable>(item: T) {
    println!("{}", item);
    item.draw();
}
```
Some traits can be automatically derived for custom types using `#[derive]` attribute.
```rust 
  #[derive(Debug)]
  struct Point { x: f64, y: f64, }
```

	 Blanket implementation
```rust
   impl<T: Display> Greet for T { fn greet(&self) { println!("Greetings, {}!", self); } }
```
