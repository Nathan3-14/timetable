use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Lesson {
    pub subject: String,
    pub teacher_name: String,
    pub time: String,
    pub room: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lessons {
    pub mon: Vec<Lesson>,
    pub tue: Vec<Lesson>,
    pub wed: Vec<Lesson>,
    pub thu: Vec<Lesson>,
    pub fri: Vec<Lesson>,
}

pub struct LessonsIterator {
    collection: Lessons,
    index: usize,
}

/// An individual timetable
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timetable {
    pub subjects: Vec<String>,
    pub id: String,
    pub lessons: Lessons,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalStorage {
    /// The field for colour preference data.
    /// Stored in the form `<lesson: String>: <color: String>`
    pub colors: LinkedHashMap<String, String>,

    /// The id of the timetable to open to.
    pub default_id: String,

    /// The field for Timetable objects.
    /// These are in the form `<id: String>: <timetable: Timetable>`
    pub timetables: LinkedHashMap<String, Timetable>,
}

impl Index<usize> for Lessons {
    type Output = Vec<Lesson>;
    fn index(&self, i: usize) -> &Vec<Lesson> {
        match i {
            0 => &self.mon,
            1 => &self.tue,
            2 => &self.wed,
            3 => &self.thu,
            4 => &self.fri,
            _ => panic!("unknown field: {}", i),
        }
    }
}

impl IntoIterator for Lessons {
    type Item = Vec<Lesson>;
    type IntoIter = LessonsIterator;

    fn into_iter(self) -> Self::IntoIter {
        LessonsIterator {
            collection: self,
            index: 0,
        }
    }
}

impl Iterator for LessonsIterator {
    type Item = Vec<Lesson>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => self.collection.mon.clone(),
            1 => self.collection.tue.clone(),
            2 => self.collection.wed.clone(),
            3 => self.collection.thu.clone(),
            4 => self.collection.fri.clone(),
            _ => return None,
        };

        self.index += 1;
        Some(result)
    }
}
