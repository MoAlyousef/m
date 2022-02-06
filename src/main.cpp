#include <livid/livid.hpp>
#include <string>

#include "literals.h"

using namespace livid;

using Nav = Widget<WidgetType::Nav>;
using Div = Widget<WidgetType::Div>;
using A = Widget<WidgetType::A>;
using Span = Widget<WidgetType::Span>;
using Img = Widget<WidgetType::Img>;
using H1 = Widget<WidgetType::H1>;
using P = Widget<WidgetType::P>;
using Article = Widget<WidgetType::Article>;
using Li = Widget<WidgetType::Li>;
using Ul = Widget<WidgetType::Ul>;

void expand() {
    auto nav_menu = WidgetBase::from_id("navmenu");
    auto is_active = nav_menu.klass().find("is-active") != std::string::npos;
    if (is_active) {
        nav_menu.klass("navbar-menu");
    } else {
        nav_menu.klass("navbar-menu is-active");
    }
}

// clang-format off
void about() {
    auto main_div = WidgetBase::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Article().klass("message is-info").append(
            Div().klass("message-header").append(
                P().text("About")
            )
        ).append(
            Div().klass("message-body")
                .inner_html("This site is a single page app and was created using "
                "<a href='https://github.com/MoAlyousef/livid'>Livid</a>, "
                "a C++ wasm frontend library and uses <a href='https://bulma.io/'>Bulma</a> "
                "and <a href='https://fontawesome.com/'>fontawesome</a> for the CSS. "
                "The source code can be found <a href='https://github.com/MoAlyousef/MoAlyousef'>here</a>.")
        )
    );
}

void contact() {
    auto main_div = WidgetBase::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                H1().inner_html("Contact")
            ).append(
                P().inner_html("I can be contacted using the following media:")
            ).append(
                Ul().append(
                    Li().append(
                        A().href("https://twitter.com/MoAlyousef")
                            .inner_html("<span class='fa fa-twitter'></span>"
                            "    http://twitter.com/MoAlyousef")
                    )
                ).append(
                    Li().append(
                        A().href("https://sa.linkedin.com/in/moalyousef")
                            .inner_html("<span class='fa fa-linkedin-square'></span>"
                            "    https://sa.linkedin.com/in/moalyousef")
                    )
                ).append(
                    Li().append(
                        A().href("https://github.com/MoAlyousef")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef")
                    )
                ).append(
                    Li().append(
                        A().href("mailto:mohammed.alyousef@neurosrg.com")
                            .inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;"
                            " direction: rtl;'>moc.grsoruen@fesuoyla.demmahom</span>")
                    )
                ).append(
                    Li().append(
                        A().href("mailto:maalyousef@kau.edu.sa")
                            .inner_html("<span class='fa fa-envelope'></span><span style='unicode-bidi:bidi-override;"
                            " direction: rtl;'>as.ude.uak@fesuoylaam</span>")
                    )
                )
            )
        )
    );
}

void resume() {
    auto main_div = WidgetBase::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                Div().inner_html(my_resume)
            )
        )
    );
}

void projs() {
    auto main_div = WidgetBase::from_id("maindiv");
    main_div.inner_html("");
    main_div.append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                H1().inner_html("Projects")
            ).append(
                P().inner_html("Some of my projects:")
            ).append(
                Ul().append(
                    Li().append(
                        A().href("https://github.com/fltk-rs/fltk-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/fltk-rs/fltk-rs")
                    ).append(
                        P().text("Rust bindings for the FLTK gui library.")
                    )
                ).append(
                    Li().append(
                        A().href("https://github.com/build-cpp/cmkr")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/build-cpp/cmkr")
                    ).append(
                        P().text("A CMakeLists.txt generator from TOML.")
                    )
                ).append(
                    Li().append(
                        A().href("https://github.com/MoAlyousef/cfltk")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/cfltk")
                    ).append(
                        P().text("A C89 wrapper for FLTK.")
                    )
                ).append(
                    Li().append(
                        A().href("https://github.com/MoAlyousef/soloud-rs")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/soloud-rs")
                    ).append(
                        P().text("Rust bindings for the Soloud audio playback/synth library.")
                    )
                ).append(
                    Li().append(
                        A().href("https://github.com/MoAlyousef/livid")
                            .inner_html("<span class='fa fa-github'></span>"
                            "    https://github.com/MoAlyousef/livid")
                    ).append(
                        P().text("A single header C++ wasm frontend library leveraging Emscripten.")
                    )
                )
            )
        )
    );
}

void create_navbar() {
    Nav().klass("navbar bd-navbar").append(
        Div().klass("navbar-brand").append(
            A().klass("navbar-item").href("index.html").append(
                Img().attr("src", "assets/brand.png")
            )
        ).append(
            A().klass("navbar-burger").append(
                Span()
            ).append(
                Span()
            ).append(
                Span()
            ).handle(Event::Click, expand).attr("data-target", "navmenu")
        )
    ).append(
        Div().id("navmenu").klass("navbar-menu").append(
            Div().klass("navbar-start").append(
                A().klass("navbar-item").href("index.html").text("Home")
            ).append(
                A().klass("navbar-item").text("Projects").handle(Event::Click, projs)
            ).append(
                A().klass("navbar-item").text("Resumé").handle(Event::Click, resume)
            ).append(
                A().klass("navbar-item").text("Contact").handle(Event::Click, contact)
            ).append(
                A().klass("navbar-item").text("About").handle(Event::Click, about)
            )
        ).append(
            Div().klass("navbar-end")
        )
    );
}
// clang-format on

int main() {
    create_navbar();
    Div().klass("content").id("maindiv").append(
        Div().klass("card").append(
            Div().klass("card-content").append(
                Div().inner_html(about_me)
            )
        )
    );
}