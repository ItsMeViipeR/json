# JSON

JSON is a json specialized tool to write json files in a more readable way.

## Installation

To install json, you need to have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Then, you can install json with the following command:

```bash
cargo add json
```

## Usage

To use json, you need to add the following line to your code:

```rust
use json::JsonEditor;
```

Then, you can create a json object with the following syntax:

```rust
let mut json = JsonEditor::open_from_file("json_file.json").expect("Failed to create from file.");
```

After it, you can edit the json object with the following syntax:

```rust
json.add_key("int", 1).expect("Failed to add key");
json.add_key("float", 1.0).expect("Failed to add key");
json.add_key("string", "Hello World!").expect("Failed to add key");
json.add_key("bool", true).expect("Failed to add key");
```

Finally, you can save the json object with the following syntax:

```rust
json.save_to_file("json_file.json").expect("Failed to save to file.");
```

You can also read the json object with the following syntax:

```rust
let mut json = JsonEditor::open_from_file("json_file.json").expect("Failed to create from file.");
let values = json.json_data.as_object().expect("Failed to get object");
```

The values will be a `&Map<String, Value>`.

But you can also get it as `Vec` with the following syntax:

```rust
json.json_data.as_vec().expect("Failed to get vec");
```

The value will be an `Option<&Vec<Value>>`.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

json is licensed under the [MIT](https://choosealicense.com/licenses/mit/) license.