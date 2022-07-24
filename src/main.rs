use livid::{document::Document, enums::*, widget::Widget, widgets::*};

mod literals;
use literals::*;

fn expand(_: &Widget) {
    let nav_menu = Widget::from_id("navmenu").unwrap();
    let is_active = nav_menu.class_name().contains("is-active");
    if is_active {
        nav_menu.set_class_name("navbar-menu");
    } else {
        nav_menu.set_class_name("navbar-menu is-active");
    }
}

fn my_index(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    main_div.set_inner_html("");
    main_div.append(&{
        let d = div();
        d.set_class_name("card");
        d.append(&{
            let d = div();
            d.set_class_name("card-content");
            d.append(&{
                let d = div();
                d.set_inner_html(ABOUT_ME);
                d
            });
            d
        });
        d
    });
}

fn blogs(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    main_div.set_inner_html("");
    main_div.append(&{
        let d = div();
        d.set_class_name("card");
        d.append(&{
            let d = div();
            d.set_class_name("card-content");
            d.append(&{
                let h = h1();
                h.set_text_content(Some("Blogs"));
                h
            });
            d.append(&{
                let p = p();
                p.set_text_content(Some("My blogs:"));
                p
            });
            d.append(&{
                let ul = ul();
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/MoAlyousef/blob/main/blogs/2022-07-18-objc-runtime.md").unwrap();
                        a.set_inner_html("<span class='fa fa-github'></span>
                         Programming against the Objective-C runtime");
                        a
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/MoAlyousef/blob/main/blogs/2021-05-04-cargo.md").unwrap();
                        a.set_inner_html("<span class='fa fa-github'></span>
                        Cargo as a tool to distribute C/C++ executables");
                        a
                    });
                    li
                });
                ul
            });
            d
        });
        d
    });
}

fn contact(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    main_div.set_inner_html("");
    main_div.append(&{
        let d = div();
        d.set_class_name("card");
        d.append(&{
            let d = div();
            d.set_class_name("card-content");
            d.append(&{
                let h = h1();
                h.set_text_content(Some("Contact"));
                h
            });
            d.append(&{
                let p = p();
                p.set_text_content(Some("I can be contacted using the following media:"));
                p
            });
            d.append(&{
                let ul = ul();
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://twitter.com/MoAlyousef").unwrap();
                        a.set_inner_html("<span class='fa fa-twitter'></span>
                        https://twitter.com/MoAlyousef");
                        a
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://sa.linkedin.com/in/moalyousef").unwrap();
                        a.set_inner_html("<span class='fa fa-linkedin-square'></span>
                            https://sa.linkedin.com/in/moalyousef");
                        a
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef").unwrap();
                        a.set_inner_html("<span class='fa fa-github'></span>
                            https://github.com/MoAlyousef");
                        a
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "mailto:mohammed.alyousef@neurosrg.com").unwrap();
                        a.set_inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;
                         direction: rtl;'>moc.grsoruen@fesuoyla.demmahom</span>");
                        a
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "mailto:maalyousef@kau.edu.sa").unwrap();
                        a.set_inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;
                         direction: rtl;'>as.ude.uak@fesuoylaam</span>");
                        a
                    });
                    li
                });
                ul
            });
            d
        });
        d
    });
}

fn projs(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    main_div.set_inner_html("");
    main_div.append(&{
        let d = div();
        d.set_class_name("card");
        d.append(&{
            let d = div();
            d.set_class_name("card-content");
            d.append(&{
                let h = h1();
                h.set_text_content(Some("Projects"));
                h
            });
            d.append(&{
                let p = p();
                p.set_text_content(Some("Some of my projects:"));
                p
            });
            d.append(&{
                let ul = ul();
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/fltk-rs/fltk-rs")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/fltk-rs/fltk-rs",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("Rust bindings for the FLTK gui library."));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/build-cpp/cmkr")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/build-cpp/cmkr",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("A CMakeLists.txt generator from TOML."));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/cfltk")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/cfltk",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("A C89 wrapper for FLTK."));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/soloud-rs")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/soloud-rs",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some(
                            "Rust bindings for the Soloud audio playback/synth library.",
                        ));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/livid")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/livid",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some(
                            "A single header C++ wasm frontend library leveraging Emscripten.",
                        ));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/livid-rs")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/livid-rs",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("A Rust wasm framework leveraging websys."));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/floui")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/floui",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("A single header C++ mobile gui library."));
                        p
                    });
                    li
                });
                ul.append(&{
                    let li = li();
                    li.append(&{
                        let a = a();
                        a.set_attribute("target", "_blank").unwrap();
                        a.set_attribute("href", "https://github.com/MoAlyousef/floui-rs")
                            .unwrap();
                        a.set_inner_html(
                            "<span class='fa fa-github'></span>
                        https://github.com/MoAlyousef/floui-rs",
                        );
                        a
                    });
                    li.append(&{
                        let p = p();
                        p.set_text_content(Some("A Rust mobile gui library."));
                        p
                    });
                    li
                });
                ul
            });
            d
        });
        d
    });
}

fn resume(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    main_div.set_inner_html("");
    main_div.append(&{
        let d = div();
        d.set_class_name("card");
        d.append(&{
            let d = div();
            d.set_class_name("card-content");
            d.append(&{
                let d = div();
                d.set_inner_html(MY_RESUME);
                d
            });
            d
        });
        d
    });
}

fn about(_: &Widget) {
    let main_div = Widget::from_id("maindiv").unwrap();
    // We clear the div from all it's text, including appending elements
    main_div.set_inner_html("");
    main_div.append(&{
        let a = article();
        a.set_class_name("message is-info");
        a.append(&{
            let d = div();
            d.set_class_name("message-header");
            d.append(&{
                let p = p();
                p.set_text_content(Some("About"));
                p
            });
            d
        });
        a.append(&{
            let d = div();
            d.set_class_name("message-body");
            d.set_inner_html(
                "This site is a single page app. It was created using 
            <a href='https://github.com/MoAlyousef/livid-rs'>Livid-rs</a>, 
            a Rust wasm frontend library. And it uses <a href='https://bulma.io/'>Bulma</a> 
            and <a href='https://fontawesome.com/'>fontawesome</a> for the CSS. 
            The source code can be found <a href='https://github.com/MoAlyousef/site'>here</a>.",
            );
            d
        });
        a
    });
}

// Creates the main navbar
fn create_navbar() {
    let n = nav();
    n.set_class_name("navbar bd-navbar");
    n.append(&{
        let d = div();
        d.set_class_name("navbar-brand");
        d.append(&{
            let a = a();
            a.set_attribute("target", "_blank").unwrap();
            a.set_class_name("navbar-item");
            a.append(&{
                let i = img();
                i.set_attribute("src", "assets/brand.png").unwrap();
                i
            });
            a.add_callback(Event::Click, my_index);
            a
        });
        d.append(&{
            let a = a();
            a.set_class_name("navbar-burger");
            a.append(&{ span() });
            a.append(&{ span() });
            a.append(&{ span() });
            a.set_attribute("data-target", "navmenu").unwrap();
            a.add_callback(Event::Click, expand);
            a
        });
        d
    });
    n.append(&{
        let d = div();
        d.set_class_name("navbar-menu");
        d.set_id("navmenu");
        d.append(&{
            let d = div();
            d.set_class_name("navbar-start");
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Home"));
                a.add_callback(Event::Click, my_index);
                a
            });
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Projects"));
                a.add_callback(Event::Click, projs);
                a
            });
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Blogs"));
                a.add_callback(Event::Click, blogs);
                a
            });
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Resumé"));
                a.add_callback(Event::Click, resume);
                a
            });
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("Contact"));
                a.add_callback(Event::Click, contact);
                a
            });
            d.append(&{
                let a = a();
                a.set_attribute("target", "_blank").unwrap();
                a.set_class_name("navbar-item");
                a.set_text_content(Some("About"));
                a.add_callback(Event::Click, about);
                a
            });
            d
        });
        d.append(&{
            let d = div();
            d.set_class_name("navbar-end");
            d
        });
        d
    });
}

fn main() {
    // grab css and fonts
    Document::add_css_link("https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css");
    Document::add_css_link(
        "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css",
    );
    // Create the navbar
    create_navbar();
    // Create our main div
    let d = div();
    d.set_class_name("content");
    d.set_id("maindiv");
    // create our index
    my_index(&d);
}
