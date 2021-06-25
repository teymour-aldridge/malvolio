# Malvolio

Malvolio is a library with a "builder-syntax" for creating complex HTML documents with ease. It runs
both on servers (and renders to strings) or in browsers (with [Yew](https://yew.rs)).

Unlike "jsx" style solutions, Malvolio avoids climbing out what has been described as the
"macro escape hatch" [1](https://twitter.com/graydon_pub/status/1294692200916246528). This means
that you get good editor support, automatic code formatting, and easier refactoring for nicer web
applications that you actually enjoy working on, instead of just pining to hand them off to the next
unlucky soul.

## Usage

Malvolio should be relatively simple to use, especially if you have used other libraries which offer
"builder-syntax" style APIs.

A quick couple of examples (see the examples and documentation section lower down for more details):

```rust
malvolio::prelude::Form::new()
    .attribute(Method::Post)
    .child(
        Input::default()
            .attribute(Type::Text)
            .attribute(Name::new("invited-user-identifier")),
    )
    .child(
        Input::default()
            .attribute(Type::Submit)
            .attribute(Value::new("Invite teacher!")),
    )
```

```rust
/* Note that this example DOES NOT COMPILE as is because it is part of a function which it was taken
from and without which all the variables are not correctly defined. */
Html::default()
    .head(default_head("Notifications".to_string()))
    .body({
        let mut body = Body::default();
        if let Some(element) = custom_element {
            body = body.child(element);
        }
        body.child(
            Div::new()
                .attribute(malvolio::prelude::Class::from(LIST))
                .children(data.into_iter().map(|notification| {
                    Div::new()
                        .attribute(malvolio::prelude::Class::from(LIST_ITEM))
                        .child(H3::new(notification.title))
                        .child(P::with_text(notification.contents))
                        .child(
                            A::default()
                                .attribute(Href::new(format!(
                                    "/notifications/mark_read/{}",
                                    notification.id
                                )))
                                .text("Mark as read"),
                        )
                        .child(
                            A::default()
                                .attribute(Href::new(format!(
                                    "/notifications/delete/{}",
                                    notification.id
                                )))
                                .text("Delete this notification"),
                        )
                })),
        )
```

## Examples
We haven't provided any explicit examples per-se, but we are in the process of writing a web
application with Malvolio which might be helpful when trying to work out how to use the library.
The [source code is on Github](https://github.com/lovelace-ed/lovelace/tree/main/main).

## Documentation

Malvolio has API docs which are [hosted on docs.rs](https://docs.rs/malvolio).

We do our best to write comprehensive, readable and concise documentation. But, like everyone, we do
sometimes fall short. If you find this to be the case (and as you are the person we intend the
documentation for we might add that if you can't understand it then that's definitely a problem –
don't think that you're just someone who "doesn't get it" drowning in a sea of people who do)
please, please, please open an
[issue on our Github repository](https://github.com/lovelace-ed/lovelace/issues) (yes, that is the
correct repository :-) ). We won't bite – atol. The fact that you thought it was an issue makes it
an issue and we can only fix it together!
