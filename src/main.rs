use cursive::{event::Key, views::Dialog};
use cursive::menu::MenuTree;

use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let mut app = cursive::default();

    // We'll use a counter to name new files.
    let counter = AtomicUsize::new(1);

    // The menubar is a list of (label, menu tree) pairs.
    app.menubar()
        .add_subtree(
            "Файл",
            MenuTree::new()
                // Trees are made of leaves, with are directly actionable...
                .leaf("New", move |s| {
                    // Here we use the counter to add an entry
                    // in the list of "Recent" items.
                    let i = counter.fetch_add(1, Ordering::Relaxed);
                    let filename = format!("New {}", i);
                    s.menubar()
                        .find_subtree("File")
                        .unwrap()
                        .find_subtree("Recent")
                        .unwrap()
                        .insert_leaf(0, filename, |_| ());

                    s.add_layer(Dialog::info("New file!"));
                })
                // ... and of sub-trees, which open up when selected.
                .subtree(
                    "Recent",
                    // The `.with()` method can help when running loops
                    // within builder patterns.
                    MenuTree::new()/*.with(|tree| {
                        for i in 1..100 {
                            // We don't actually do anything here,
                            // but you could!
                            tree.add_item(menu::Item::leaf(format!("Item {}", i), |_| ()).with(|s| {
                                if i % 5 == 0 { s.disable(); }
                            }))
                        }
                    })*/,
                )
                .leaf("Выход", |s| s.quit()),
            // Delimiter are simple lines between items,
            // and cannot be selected.
            // .delimiter()
            /*.with(|tree| {
                for i in 1..10 {
                    tree.add_leaf(format!("Option {}", i), |_| ());
                }
            })*/
        )
        .add_subtree(
            "Help",
            MenuTree::new()
                .subtree(
                    "Help",
                    MenuTree::new()
                        .leaf("General", |s| {
                            s.add_layer(Dialog::info("Help message!"))
                        })
                        .leaf("Online", |s| {
                            let text = "Google it yourself!\n\
                                        Kids, these days...";
                            s.add_layer(Dialog::info(text))
                        }),
                )
                .leaf("About", |s| {
                    s.add_layer(Dialog::info("Cursive v0.0.0"))
                }),
        )
        .add_delimiter()
        .add_leaf("ФИС ГИА", |s| s.add_layer(Dialog::info(
            concat!(
            env!("CARGO_PKG_NAME"),
            " ",
            env!("CARGO_PKG_VERSION"))).title("212")));

    app.set_autohide_menu(false);
    app.add_global_callback(Key::Esc, |s| s.select_menubar());
    // app.add_layer(Dialog::text("Hit <Esc> to show the menu!"));
    app.run();
}


// use cursive::Cursive;
// use cursive::views::Dialog;
//
// fn main() {
//     let mut siv = cursive::default();
//
//     siv.add_layer(Dialog::text("This is a survey!\nPress <Next> when you're ready.")
//         .title("Important survey")
//         .button("Next", show_next));
//
//     siv.run();
// }
//
// fn show_next(s: &mut Cursive) {
//     s.pop_layer();
//     s.add_layer(Dialog::text("Did you do the thing?")
//         .title("Question 1")
//         .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
//         .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
//         .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
// }
//
// fn show_answer(s: &mut Cursive, msg: &str) {
//     s.pop_layer();
//     s.add_layer(Dialog::text(msg)
//         .title("Results")
//         .button("Finish", |s| s.quit()));
// }

// use cursive::views::TextView;
//
// fn main() {
//     let mut app = cursive::default();
//     app.add_global_callback('q', |s| s.quit());
//     app.add_layer(TextView::new("Hello, world!"));
//     app.run();
// }
