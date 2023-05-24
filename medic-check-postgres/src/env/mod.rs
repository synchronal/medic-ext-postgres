pub fn expand_str(string: &str) -> String {
    let cwd = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut context = std::collections::HashMap::new();
    context.insert("CWD".to_string(), cwd);
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }

    envsubst::substitute(string, &context).unwrap()
}

pub fn expand_string(string: String) -> String {
    let cwd = std::env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    let mut context = std::collections::HashMap::new();
    context.insert("CWD".to_string(), cwd);
    for (key, value) in std::env::vars() {
        context.insert(key, value);
    }

    envsubst::substitute(string, &context).unwrap()
}
