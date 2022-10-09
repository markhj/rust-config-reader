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

And accessing the properties as simply as:

````rust
let cfg: Config = read("config-file.txt").unwrap();
let host: String = cfg.group("server").unwrap().get_or("host", "localhost"); 
````

## Installing
Add this in your ``Cargo.toml`` file:

````
[dependencies.rust-config-reader]
git = "https://github.com/markhj/rust-config-reader"
````

## Usage
Import the ``read`` method, in order to retrieve the parsed ``Config`` struct, which
is the object you'll interact with, in order to get configuration items.

````rust
use rust_config_reader::read;

let reader = read("my-config-file", None).expect("Config file not found");
````

If the file does not exist, an ``Err`` is returned. In the example above, we
immediately ``panic`` upon encountering this error. You can go a different route, and for example build a default
config file, when catching this error.

Once the ``Config`` struct is successfully loaded, we can start reading the contents.

### Accessing configuration items
The first step is to access a group of items. This is achieved with the ``group`` method:

````rust
pub fn group(group: &str) -> Option<Group>
````
It returns an ``Option``, which will be ``None`` in case the group doesn't exist.
With the ``Group`` struct you can retrieve configuration items by key.

Imagine this configuration file:
````
[server]
port = 1234
````

We can retrieve the ``port`` value by doing this:
````rust
let port = reader.group("server").unwrap().get("port").unwrap().value;
````

In this example we uncritically try to unwrap the returned options.
In your real-world application you might want to approach this differently.

To gracefully use a default/fallback value when a configuration isn't defined, is to use
the ``get_or`` method.

````rust
let port = reader.group("server").unwrap().get_or("port", "8080");
````

You'll notice there's no unwrapping in this case. This is because the error handling for
when the configuration item doesn't exist, is handled implicitly by returning a default value.
Hence, there's no need for an ``Option`` struct.

## Full example
my-config-file:
````
[server]
port = 1234
````

.rs file:
````rust
use rust_config_reader::read;

let reader = read("my-config-file", None).expect("Config file not found");
let server_group = reader.group("server").expect("[server] group not defined");

// Panic when missing config:
let port: String = server_group.get("port").expect("Port must be specified");

// Gracefully use a default value:
let host: String = server_group.get_or("host", "127.0.0.1");
````

## Other methods
### Config::for_each_group
````rust
pub fn for_each_group<F: Fn(&str, &Group)>(closure: F)
````
Loop over every group contained in the configuration.

Example:

````rust
reader.for_each_group(|key: &str, group: &Group| {
    println!("Found a group called: {}", key);
});
````

### Config::groups
````rust
pub fn groups() -> Vec<String> 
````
Returns a ``Vec<String>`` collection of the group names contained in the configuration file.

### Config::has_group
````rust
pub fn has_group(group: &str) -> bool
````
Returns true, if the group exists in the configuration file.

### Group::for_each
````rust
pub fn for_each<F: Fn(&ConfigurationItem)>(closure: F)
````
Iterates through the configuration items within a group.

````rust
reader.group("server")
    .unwrap()
    .for_each(|cfg_item: &ConfigurationItem| {
        println!("Found server config item: {:?}", cfg_item);
    });
````


### Group::keys
````rust
pub fn keys() -> Vec<String>
````
Returns a ``Vec<String>`` collection of defined keys in a group.

For example, in this configuration file:

````
[server]
port = 1234
host = 127.0.0.1
````

The ``keys`` function would find ``port`` and ``host``.

### Group::has
````rust
pub fn has(key: &str)
````
Returns true, if the ``key`` exists in the group.

# Options
You can configure the behavior of the configuration file parser by injecting an instance of the ``Options`` struct into the
``read`` function. You may have noticed in the examples that we injected ``None`` as second parameter.
This means we use the default configuration.

Example usage:
````rust
let mut options = get_default_options;
options.string_strictness = Forigvable;
let cfg: Config = read("file.txt", options).unwrap();
````

The advantage to using the ``get_default_options`` function is that it provides the full struct
with all defined keys. You can, of course, set everything up yourself, but it would become a tedious process.

| Key |         Values          | Default | Description |
| :---:|:-----------------------:|:-------:| :---: |
| ``string_strictness`` | Loose, Forgivable, Very |  Loose  | Determines how string values are handled. ``Loose`` will never require quotation marks. ``Forgivable`` will allow omission of quotation marks on strings without whitespaces. ``Very`` will always require quotation marks. |
| ``string_strictness_behavior`` | Ignore, Panic | Ignore | Describes what happens when an invalid string is encountered. ``Ignore`` means the configuration item is dropped from consideration. ``Panic`` will, as the name implies, do a ``panic!`` with reference to the invalid line.

# Type casting

Values from the configuration file are always returned as ``String``.
But you can easily cast configuration items as a range of primitive data types.

Usage:
````rust
let port: i32 = cfg.group("server").unwrap().get("port").unwrap().as_i32();
````

All methods:

````rust
pub fn as_str()
pub fn as_i32()
pub fn as_u32()
pub fn as_f32()
pub fn as_i64()
pub fn as_u64()
pub fn as_f64()
````

## as_bool

The ``as_bool`` and ``as_bool_grf`` methods provides an easy short-hand to read a value as boolean.

These values will be mapped to ``true``:
* 1
* true
* on
* yes

And on the contrary these values will be mapped to ``false``:
* 0
* false
* off
* no

Any other provided value will cause the ``Err`` of type ``InvalidBoolValue`` to be returned.

You can use the method ``as_bool_grf`` to gracefully interpret invalid syntax as false.

### Example

Config file:
````
[server]
root_login = on
````

Rust file:
````rust
let root_login: bool = cfg.group("server").unwrap().get("root_login").unwrap().as_bool().unwrap();
````

