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
Import the ``read`` method:

````rust
use rust_config_reader::read;

let cfg = read("my-config-file").expect("Config file not found");
````

If the file does not exist, an ``Err`` is returned. In the example above, we
immediately ``panic`` upon encountering this error. You can go a different route, and for example build a default
config file, when catching this error.

Once the ``Configuration`` struct is successfully loaded, we can start reading the contents.

Method signature:
````rust
pub fn get(group : &str, key : &str) -> Result<String, ()>
````

As you can see the method returns a ``Result``, which allows for graceful handling
in case of missing or malformed values.

````rust
let port : String = cfg.get("server", "port").unwrap();
````
