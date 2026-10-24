pub fn topbar(args: &[String]) -> String {
    let command = args.get(0).cloned();

    // if &command == &Some("toggle-floating".into()) {
    //     let mut bar = Topbar::write();

    //     let next = !bar.floating;
    //     bar.set_floating(next);

    //     return if bar.floating {
    //         "Set to floating".into()
    //     } else {
    //         "Set to not floating".into()
    //     };
    // }

    // if &command == &Some("set-floating".into()) {
    //     let mut bar = Topbar::write();

    //     let value = match args.get(1).map(String::as_str) {
    //         Some("true") => true,
    //         Some("false") => false,
    //         arg => return format!("Invalid argument for set-floating: {arg:?}"),
    //     };
    //     bar.set_floating(value);

    //     return if bar.floating {
    //         "Set to floating".into()
    //     } else {
    //         "Set to not floating".into()
    //     };
    // }

    return format!("Invalid command: {command:?}").into();
}
