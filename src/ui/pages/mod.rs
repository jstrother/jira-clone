use std::{any::Any, fmt::Result, rc::Rc};
use itertools::Itertools;
use anyhow::{anyhow, Result};
use crate::{db::JiraDatabase, models::Action};

mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}

impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        println!("****************************** EPICS ******************************");
        println!("        id        |           name           |       status       ");

        let epics = self.db.read_db()?.epics;

        for id in epics.keys().sorted() {
            todo!("sort through epic ids");
        }

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        todo!("handle input for HomePage");
    }

    fn as_any(&self) -> &dyn Any {
        todo!("as_any for HomePage");
    }
}