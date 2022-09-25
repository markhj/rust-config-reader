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

## Usage
Import the ``read`` method, in order to retrieve the parsed ``Configuration`` struct, which
is the object you'll interact with, in order to get configuration items.

````rust
use rust_config_reader::read;

let cfg = read("my-config-file").expect("Config file not found");
````

If the file does not exist, an ``Err`` is returned. In the example above, we
immediately ``panic`` upon encountering this error. You can go a different route, and for example build a default
config file, when catching this error.

Once the ``Configuration`` struct is successfully loaded, we can start reading the contents.
You can use either the ``get`` method which returns a ``Result`` when a key/value pair doesn't exist. Or you
can use the ``get_or`` method which instead returns a default value.

### get
````rust
pub fn get(group : &str, key : &str) -> Result<String, ()>
````

As you can see the ``get`` method returns a ``Result`` type, which allows for graceful handling
in case of missing or malformed values.

````rust
let port : String = cfg.get("server", "port").unwrap();
````

### get_or
````rust
pub fn get_or(group : &str, key : &str, default : &str) -> String
````
As an alternative, you can use the ``get_or`` method, which instead of returning
an ``Err`` when it cannot find a key/value pair, returns a fallback/default value.

````rust
let port : String = cfg.get_or("server", "port", "1234");
````
