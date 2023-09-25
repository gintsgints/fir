use custom_error::custom_error;
use std::io;

custom_error! {pub ProgramError
    Io {source: io::Error}              = "Application I/O error: {source}",
}

custom_error! {pub FileListError
    Io {source: io::Error}              = "Directory does not exist",
}