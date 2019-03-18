#![rustfmt::skip::macros(skip_macro_mod)]

mod foo;

#[rustfmt::skip::macros(html, skip_macro)]
fn main() {
    let macro_result1 = html! { <div>
Hello</div>
    }
    .to_string();

    let macro_result2 = not_skip_macro! { <div>
    Hello</div>
        }
    .to_string();

    skip_macro! {
this is a skip_macro here
};

    foo();
}

fn foo() {
    // should be mangled
    let macro_result1 = html! { <div>
    Hello</div>
        }
    .to_string();
}

fn bar() {
    let macro_result1 = skip_macro_mod! { <div>
    Hello</div>
        }
    .to_string();
}
