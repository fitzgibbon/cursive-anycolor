extern crate cursive;

use cursive::prelude::*;

fn main() {

    let mut siv = Cursive::new();

    siv.menubar()
       .add("File",
            MenuTree::new()
                .leaf("New", |s| s.add_layer(Dialog::info("New file!")))
                .subtree("Recent", MenuTree::new().with(|tree| {
                    for i in 1..100 {
                        tree.add_leaf(&format!("Item {}", i), |_| ())
                    }
                }))
                .delimiter()
                .with(|tree| {
                    for i in 1..10 {
                        tree.add_leaf(&format!("Option {}", i), |_| ());
                    }
                })
                .delimiter()
                .leaf("Quit", |s| s.quit()))
       .add("Help",
            MenuTree::new()
                .subtree("Help",
                         MenuTree::new()
                             .leaf("General", |s| {
                                 s.add_layer(Dialog::info("Help message!"))
                             })
                             .leaf("Online", |s| {
                                 s.add_layer(Dialog::info("Google it \
                                                           yourself!\nKids, \
                                                           these days..."))
                             }))
                .leaf("About",
                      |s| s.add_layer(Dialog::info("Cursive v0.0.0"))));

    // siv.set_autohide_menu(false);

    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::new(TextView::new("Hit <Esc> to show the menu!")));

    siv.run();
}