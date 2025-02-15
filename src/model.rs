use std::{
    rc::Rc, process,
};
use rand::{
    prelude::SliceRandom,
    rngs::StdRng,
    SeedableRng,
};
use chrono::prelude::*;

use iced::widget::scrollable::Id;

use crate::combination::{
    self, Combination, Distance, Defense, Faint, Body
};

const PATH: &str = "./combinations.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceSelection {
    Short,
    Long,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenceSelection {
    Yes,
    No,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaintSelection {
    Yes,
    No,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodySelection {
    Yes,
    No,
    All,
}

#[derive(Debug, Clone)]
pub struct Model {
    number: usize,
    current: usize,
    distance_selection: Option<DistanceSelection>,
    defence_selection: Option<DefenceSelection>,
    faint_selection: Option<FaintSelection>,
    body_selection: Option<BodySelection>,
    combinations: Vec<Rc<Combination>>,
    data: Vec<Rc<Combination>>,
    scrollable_id: iced::widget::scrollable::Id,
}

impl Model {

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn scrollable_id(&self) -> iced::widget::scrollable::Id {
        self.scrollable_id.clone()
    }

    pub fn number(&self) -> String {
        format!("{}.", self.number)
    }
   
    pub fn combination(&self) -> String {
        if self.combinations.is_empty() {
            return "None".to_owned();
        }
        self.combinations[self.current].description.clone()
    }

    pub fn combinations(&self) -> &Vec<Rc<Combination>> {
        &self.combinations
    }

    pub fn distance_selection(&self) -> Option<DistanceSelection> {
        self.distance_selection
    }

    pub fn set_distance_selection(&mut self, option:  DistanceSelection ) {
        self.distance_selection = Some(option);
        self.update_filter();
    }

    pub fn defence_selection(&self) -> Option<DefenceSelection> {
       self.defence_selection 
    }

    pub fn set_defence_selection(&mut self, option: DefenceSelection) {
        self.defence_selection = Some(option);
        self.update_filter();
    }

    pub fn faint_selection(&self) -> Option<FaintSelection> {
        self.faint_selection
    }

    pub fn set_faint_selection(&mut self, option: FaintSelection) {
        self.faint_selection = Some(option);
        self.update_filter();
    }

    pub fn body_selection(&self) -> Option<BodySelection> {
        self.body_selection
    }

    pub fn set_body_selection(&mut self, option: BodySelection) {
        self.body_selection = Some(option);
        self.update_filter();
    }

    pub fn reset(&mut self) {
        self.number = 1;
        self.current = 0;
    }

    pub fn reset_in_random_order(&mut self) {
        self.reset();
     
        let seed = Utc::now().timestamp_millis() as u64;
        let mut rng = StdRng::seed_from_u64(seed);
        self.combinations.shuffle(&mut rng);
    }

    pub fn reset_in_order(&mut self) {
        self.reset();
        self.update_filter();
    }

    pub fn next(&mut self) {
        if self.combinations.is_empty() {
            return;
        }
        self.number += 1;
        self.current = (self.current+1) % self.combinations.len();
    }

    pub fn previous(&mut self) {
        if self.combinations.is_empty() {
            return;
        }
        self.number += 1;

        self.current = if self.current == 0 {
                            self.combinations.len()-1
                        } else {
                            self.current-1
                        }
    }

    pub fn set(&mut self, index: usize) {
        self.number += 1;
        self.current = index;
    }

    fn update_filter(&mut self) {
        self.combinations = filter(
            &self.data, 
            self.distance_selection.unwrap(),
            self.defence_selection.unwrap(),
            self.faint_selection.unwrap(),
            self.body_selection.unwrap());
        self.reset()
    }

    pub fn reload(&mut self) {
        let data = combination::load_data(PATH);
        if data.is_err() {
            eprintln!("ERROR {}", data.unwrap_err());
            process::exit(1);
        }
        self.data = data.unwrap();
        self.reset_in_random_order();
    }
   
}

impl Default for Model {
    fn default() -> Self { 
        let data = combination::load_data(PATH);
        if data.is_err() {
            eprintln!("ERROR {}", data.unwrap_err());
            process::exit(1);
        }
        let data = data.unwrap();
        let mut s = Self {
            number: 1,
            current: 0,
            distance_selection: Some(DistanceSelection::All),
            defence_selection: Some(DefenceSelection::All),
            faint_selection: Some(FaintSelection::All),
            body_selection: Some(BodySelection::All),
            combinations: filter(
                &data, 
                DistanceSelection::All,
                DefenceSelection::All,
                FaintSelection::All,
                BodySelection::All,
            ),
            data,
            scrollable_id: Id::unique()
        };
        s.reset_in_random_order();
        s
    }
}

fn filter(
    data: &Vec<Rc<Combination>>, 
    distance: DistanceSelection,
    defence: DefenceSelection,
    faint: FaintSelection,
    body: BodySelection) -> Vec<Rc<Combination>> {
        let mut result: Vec<Rc<Combination>> =  vec![];   
        for c in data {
            if filter_combination(c, distance, defence, faint, body) {
                result.push(c.clone());
            }
        };
        result
}

fn filter_combination(
    com: &Combination, 
    distance: DistanceSelection,
    defence: DefenceSelection,
    faint: FaintSelection,
    body: BodySelection) -> bool {
        let distance_result = match com.distance {
            Distance::Long => { 
                !matches!(distance, DistanceSelection::Short) 
            },
            Distance::Short => {
                !matches!(distance, DistanceSelection::Long)
            }
        };

        let defense_result = match com.defense {
            Defense::Yes => {
                !matches!(defence, DefenceSelection::No)
            },
            Defense::No => {
                !matches!(defence, DefenceSelection::Yes)
            }
        };

        let faint_result = match com.faint {
            Faint::Yes => {
                !matches!(faint, FaintSelection::No)
            },
            Faint::No => {
                !matches!(faint, FaintSelection::Yes)
            }

        };

        let body_result = match com.body {
            Body::Yes => {
                !matches!(body, BodySelection::No)
            },
            Body::No => {
                !matches!(body, BodySelection::Yes)
            }

        };

        distance_result && defense_result && faint_result && body_result
}

