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
