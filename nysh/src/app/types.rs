#[derive(PartialEq)]
pub enum CurrentScreen {
    Main,
    NewSshIdentity,
}

#[derive(PartialEq)]
pub enum CurrentModal {
    NoModal,
    ConfirmExit,
}
