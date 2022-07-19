#include <livid/livid.hpp>
#include <string> // for std::string::npos

#include "literals.h" // for about_me and my_resume

using namespace livid;

// Expands the bulma navbar menu, since we don't use bulma.js
void expand(emscripten::val)
{
    auto nav_menu = Widget::from_id("navmenu");
    auto is_active = nav_menu.klass().find("is-active") != std::string::npos;
    if (is_active) {
        nav_menu.klass("navbar-menu");
    } else {
        nav_menu.klass("navbar-menu is-active");
    }
}

// clang-format off
// The About navbar item maindiv contents
void about(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    // We clear the div from all it's text, including appending elements
    main_div.inner_html("");
    main_div.append(
        Article().klass("message is-info").append(
            Div().klass("message-header").append(
                P().text("About")
            )
        ).append(
            Div().klass("message-body")
                .inner_html("This site is a single page app. It was created using "
                "<a href='https://github.com/MoAlyousef/livid'>Livid</a>, "
                "a C++ wasm frontend library. And it uses <a href='https://bulma.io/'>Bulma</a> "
                "and <a href='https://fontawesome.com/'>fontawesome</a> for the CSS. "
                "The source code can be found <a href='https://github.com/MoAlyousef/site'>here</a>.")
        )
    );
}

// The Contact navbar item maindiv contents
void contact(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                H1().text("Contact")
            ).append(
                P().text("I can be contacted using the following media:")
            ).append(
                Ul().append(
                    Li().append(
                        A().attr("target", "_blank").href("https://twitter.com/MoAlyousef")
                            .inner_html("<span class='fa fa-twitter'></span>"
                            "    http://twitter.com/MoAlyousef")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://sa.linkedin.com/in/moalyousef")
                            .inner_html("<span class='fa fa-linkedin-square'></span>"
                            "    https://sa.linkedin.com/in/moalyousef")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("mailto:mohammed.alyousef@neurosrg.com")
                            .inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;"
                            " direction: rtl;'>moc.grsoruen@fesuoyla.demmahom</span>")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("mailto:maalyousef@kau.edu.sa")
                            .inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;"
                            " direction: rtl;'>as.ude.uak@fesuoylaam</span>")
                    )
                )
            )
        )
    );
}

// The Resume navbar item maindiv contents
void resume(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                Div().inner_html(site::literals::my_resume) // We use a string literal which contains html elements
            )
        )
    );
}

// The Projects navbar item maindiv contents
void projs(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                H1().text("Projects")
            ).append(
                P().text("Some of my projects:")
            ).append(
                Ul().append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/fltk-rs/fltk-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/fltk-rs/fltk-rs")
                    ).append(
                        P().text("Rust bindings for the FLTK gui library.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/build-cpp/cmkr")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/build-cpp/cmkr")
                    ).append(
                        P().text("A CMakeLists.txt generator from TOML.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/cfltk")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/cfltk")
                    ).append(
                        P().text("A C89 wrapper for FLTK.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/soloud-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/soloud-rs")
                    ).append(
                        P().text("Rust bindings for the Soloud audio playback/synth library.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/livid")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/livid")
                    ).append(
                        P().text("A single header C++ wasm frontend library leveraging Emscripten.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/livid-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/livid-rs")
                    ).append(
                        P().text("A Rust wasm framework leveraging websys.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/floui")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/floui")
                    ).append(
                        P().text("A single header C++ mobile gui library.")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/floui-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/floui-rs")
                    ).append(
                        P().text("A Rust mobile gui library.")
                    )
                )
            )
        )
    );
}

void blogs(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                H1().text("Blogs")
            ).append(
                P().text("My blogs:")
            ).append(
                Ul().append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/MoAlyousef/blob/main/blogs/2022-07-18-objc-runtime.md")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    Programming against the Objective-C runtime")
                    )
                ).append(
                    Li().append(
                        A().attr("target", "_blank").href("https://github.com/MoAlyousef/MoAlyousef/blob/main/blogs/2021-05-04-cargo.md")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    Cargo as a tool to distribute C/C++ executables")
                    )
                )
            )
        )
    );
}

void my_index(emscripten::val) {
    auto main_div = Widget::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                Div().inner_html(site::literals::about_me) // We can use a string literal which uses html elements
            )
        )
    );
}

// Creates the main navbar
void create_navbar() {
    Nav().klass("navbar bd-navbar").append(
        Div().klass("navbar-brand").append(
            A().attr("target", "_blank").klass("navbar-item").append(
                Img().attr("src", "assets/brand.png")
            ).handle(Event::Click, my_index)
        ).append(
            A().attr("target", "_blank").klass("navbar-burger").append(
                Span()
            ).append(
                Span()
            ).append(
                Span()
            ).handle(Event::Click, expand).attr("data-target", "navmenu") // We call expand to expand the burger
        )
    ).append(
        Div().id("navmenu").klass("navbar-menu").append(
            Div().klass("navbar-start").append(
                A().attr("target", "_blank").klass("navbar-item").text("Home").handle(Event::Click, my_index)
            ).append(
                A().attr("target", "_blank").klass("navbar-item").text("Projects").handle(Event::Click, projs)
            ).append(
                A().attr("target", "_blank").klass("navbar-item").text("Blogs").handle(Event::Click, blogs)
            ).append(
                A().attr("target", "_blank").klass("navbar-item").text("Resumé").handle(Event::Click, resume)
            ).append(
                A().attr("target", "_blank").klass("navbar-item").text("Contact").handle(Event::Click, contact)
            ).append(
                A().attr("target", "_blank").klass("navbar-item").text("About").handle(Event::Click, about)
            )
        ).append(
            Div().klass("navbar-end")
        )
    );
}
// clang-format on

int main()
{
    // Create the navbar
    create_navbar();
    // Create our main div
    Div().klass("content").id("maindiv");
    // create our index
    my_index(emscripten::val::null());
}