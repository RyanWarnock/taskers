fn parse_command(command: String) -> Result<Command, &'static str> {
    Err("Could not parse")
}

enum Command {
    Add,
    Remove,
}
