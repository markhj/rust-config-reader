## What is this?
This Crate is used to read a configuration file on the example form:

````
[server]
port = 1234
host = localhost

[group]
key = value
another = 4321
````

## Installing
Add this in your ``Cargo.toml`` file:

````
[dependencies.rust-config-reader]
git = "https://github.com/markhj/rust-config-reader"
````

## Usage
Import the ``read`` method, in order to retrieve the parsed ``Configuration`` struct, which
is the object you'll interact with, in order to get configuration items.

````rust
use rust_config_reader::read;

let reader = read("my-config-file").expect("Config file not found");
````

If the file does not exist, an ``Err`` is returned. In the example above, we
immediately ``panic`` upon encountering this error. You can go a different route, and for example build a default
config file, when catching this error.

Once the ``Configuration`` struct is successfully loaded, we can start reading the contents.
You can use either the ``get`` method which returns a ``Result``, or you
can opt for the ``get_or`` method which returns a default value, when a key/value pair is missing.

### get
````rust
pub fn get(group : &str, key : &str) -> Result<String, ()>
````

As you can see the ``get`` method returns a ``Result`` type, so you can define the behavior
in case a key/value pair is missing.

````rust
let port : String = reader.get("server", "port").expect("Port is not defined");
````

### get_or
````rust
pub fn get_or(group : &str, key : &str, default : &str) -> String
````
As an alternative, you can use the ``get_or`` method, which instead of returning
an ``Err`` when it cannot find a key/value pair, returns a fallback/default value.

````rust
let port : String = reader.get_or("server", "port", "1234");
````

## Full example
my-config-file:
````
[server]
port = 1234
````

.rs file:
````rust
use rust_config_reader::read;

let reader = read("my-config-file").expect("Config file not found");

// Panic when missing config:
let port = reader.get("server", "port").expect("Port must be specified");

// Gracefully use a default value:
let host = reader.get_or("server", "host", "127.0.0.1");
````