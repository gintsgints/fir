use custom_error::custom_error;

custom_error! {pub ProgramError
    UnknowShell {shell: String} = "Unknown shell: {shell}"
}
