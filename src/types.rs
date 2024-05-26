pub struct Args<'a> {
    pub args: Vec<&'a str>,
    pub builtins: Vec<String>,
    pub path: String,
}
