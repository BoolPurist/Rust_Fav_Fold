use crate::cli_args::GetParams;

#[derive(Debug, PartialEq, Eq)]
pub struct DrawParam {
    clipboard: bool,
    ask_number: bool,
}

impl From<&GetParams> for DrawParam {
    fn from(value: &GetParams) -> Self {
        let (clipboard, ask_number) = (value.copy_has_clipboard(), value.copy_ask_number());
        Self {
            clipboard,
            ask_number,
        }
    }
}
impl From<GetParams> for DrawParam {
    fn from(value: GetParams) -> Self {
        let (clipboard, ask_number) = (value.copy_has_clipboard(), value.copy_ask_number());
        Self {
            clipboard,
            ask_number,
        }
    }
}

impl DrawParam {
    pub fn new(clipboard: bool, ask_number: bool) -> Self {
        Self {
            clipboard,
            ask_number,
        }
    }

    pub fn new_for_clipboard(ask_number: bool) -> Self {
        Self {
            clipboard: true,
            ask_number,
        }
    }

    pub fn clipboard(&self) -> bool {
        self.clipboard
    }

    pub fn ask_number(&self) -> bool {
        self.ask_number
    }
}
