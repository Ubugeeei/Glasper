pub(crate) const HELP_MESSAGE: &str = r#"
Usage: gls [options] [ script.js ] [arguments]

Options:
    -                             script read from stdin (default if no file name is provided, interactive mode if a tty)
    -v, --version                 print version
    -h, --help                    print command line options (currently set)
    --vm                          run in vm mode (currently set)
"#;

pub(crate) enum ExecutionType<'a> {
    Help,
    Version,
    VMInteract,
    HostInteract,
    VM { source_path: &'a str },
    Host { source_path: &'a str },
}

pub(crate) fn get_execution_type(args: &Vec<String>) -> ExecutionType<'_> {
    match args.len() {
        1 => ExecutionType::HostInteract,
        2 => match &*args[1] {
            "-h" | "--help" => ExecutionType::Help,
            "-v" | "--version" => ExecutionType::Version,
            "--vm" => ExecutionType::VMInteract,
            arg => ExecutionType::Host { source_path: arg },
        },
        _ => {
            let help_arg = args.iter().any(|arg| arg == "-h" || arg == "--help");
            let version_arg = args.iter().any(|arg| arg == "-v" || arg == "--version");
            let vm_arg = args.iter().any(|arg| arg == "--vm");

            let file_arg = args.iter().find(|arg| !arg.starts_with('-'));

            if help_arg {
                ExecutionType::Help
            } else if version_arg {
                ExecutionType::Version
            } else if let Some(file) = file_arg {
                if vm_arg {
                    ExecutionType::VM { source_path: file }
                } else {
                    ExecutionType::Host { source_path: file }
                }
            } else if vm_arg {
                ExecutionType::VMInteract
            } else {
                ExecutionType::HostInteract
            }
        }
    }
}
