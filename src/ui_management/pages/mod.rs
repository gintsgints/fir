use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::state_management::State;

use self::{editor_page::EditorPage, help_line::HelpLine, main_page::MainPage};

use super::components::{Component, ComponentRender};

mod help_line;
mod main_page;
mod editor_page;

enum ActivePage {
    MainPage,
    EditorPage,
}

struct Props {
    active_page: ActivePage,
}

impl From<&State> for Props {
    fn from(state: &State) -> Self {
        Props {
            active_page: if state.editor_file.is_none() {
                ActivePage::MainPage
            } else {
                ActivePage::EditorPage
            },
        }
    }
}

pub struct AppRouter<'a> {
    props: Props,
    main_page: MainPage<'a>,
    editor_page: EditorPage<'a>,
    help_line: HelpLine,
}

impl<'a> AppRouter<'a> {
    fn get_active_page_component(&self) -> &dyn Component {
        match self.props.active_page {
            ActivePage::MainPage => &self.main_page,
            ActivePage::EditorPage => &self.editor_page,
        }
    }

    fn get_active_page_component_mut(&mut self) -> &mut dyn Component {
        match self.props.active_page {
            ActivePage::MainPage => &mut self.main_page,
            ActivePage::EditorPage => &mut self.editor_page,
        }
    }
}

impl<'a> Component for AppRouter<'a> {
    fn new(
        state: &crate::state_management::State,
        action_tx: tokio::sync::mpsc::UnboundedSender<crate::state_management::action::Action>,
    ) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            props: Props::from(state),
            main_page: MainPage::new(state, action_tx.clone()),
            editor_page: EditorPage::new(state, action_tx.clone()),
            help_line: HelpLine::new(state, action_tx.clone()),
        }
        .move_with_state(state)
    }

    fn move_with_state(self, state: &crate::state_management::State) -> Self
    where
        Self: Sized,
    {
        AppRouter {
            props: Props::from(state),
            main_page: self.main_page.move_with_state(state),
            editor_page: self.editor_page.move_with_state(state),
            help_line: self.help_line.move_with_state(state),
        }
    }

    fn name(&self) -> &str {
        self.get_active_page_component().name()
    }

    fn handle_key_event(&mut self, key: crossterm::event::KeyEvent) {
        self.get_active_page_component_mut().handle_key_event(key)
    }
    
    fn check(&mut self) {
        self.main_page.check();
        self.editor_page.check();
        self.help_line.check();
    }
}

impl<'a> ComponentRender<()> for AppRouter<'a> {
    fn render(&self, frame: &mut Frame, _props: ()) {
        let [page_area, help_line_rec] = *Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(1)])
            .split(frame.size())
        else {
            panic!("The main layout should have 2 chunks")
        };
        match self.props.active_page {
            ActivePage::MainPage => self
                .main_page
                .render(frame, main_page::RenderProps { area: page_area }),
            ActivePage::EditorPage => self
                .editor_page
                .render(frame, editor_page::RenderProps { area: page_area })
        }
        self.help_line.render(
            frame,
            help_line::RenderProps {
                area: help_line_rec,
            },
        );
    }
}
