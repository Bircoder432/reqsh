pub fn get_help() -> &'static str {
    "
Available Commands:

-------------------------------------------------
    Requests:
        Method <path>
        [Headers]

        [Body]

-------------------------------------------------
        Methods -- GET, POST, PUT, DELETE

        [Headers] -- <key> <value>
        [Body] -- raw, json
-------------------------------------------------
    Commands:
        base <url>
        header <key> <value>
        exit
        help
-------------------------------------------------
    "
}
