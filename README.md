# Jsn Base (JSON Base)

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

`Jsn Base` is a database, similar to `sqlite`. It supports reading and writing.

## Usage

Add this crate to your `Cargo.toml` file:

```toml
[dependencies]
jsn_base = "0.1.0"
```


```rs
use jsn_base::{Connection, Model};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
    email: String,
}

fn main() {
    let connection = Connection::new("my-app").unwrap();
    let mut model = Model::<User>::new("user", &connection);

    // load all previous datas
    model.load();

    // create a new data
    model.create(User {
        id: 0,
        name: String::from("Mahi"),
        email: String::from("email"),
    });

    // save current state of model
    model.save();

    // remove all
    model.removes(|_| true);

    // get specifice
    model.get(|user| user.id == 1);

    // get specifice users
    model.gets(|user| user.name.eq("Mahi"));

    // remove specifice
    model.remove(|user| user.name.eq("Mahi"));

    // remove some users
    model.removes(|user| user.name.eq("Me"));
}

```

## Features

- `Modal.load()`: Load previous state of data.

- `Modal.create(payload)`: Create a new record.

- `Modal.create(payloads)`: Create many new record.

- `Modal.save()`: Save the current state of data.

- `Modal.get(fn)`: Get a single item.

- `Modal.gets(fn)`: Get many item.

- `Modal.remove(fn)`: Remove a sigle item.

- `Modal.removes(fn)`: Remove many item.


## Contributing

Contributions are welcome! I would like you to contribute in this project.

## Roadmap

This project is in its early stages, and there are many missing features that need implementation. Check the [Issues](/issues) section for a list of features, enhancements, and bug fixes that are planned.

## License

This project is licensed under the MIT License - see the [LICENSE](/LICENSE.md) file for details.
