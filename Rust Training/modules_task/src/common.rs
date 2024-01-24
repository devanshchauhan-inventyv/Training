// common.rs
//! A module which contains all the structures and Enum which is used in this library 
use std::iter;

///

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
/// This is a Student Structure 
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<char>,
}
/// This is a implementation block for the student struct
impl Student {
/// This function calculates percentage of student and return an [Option<f32>]
    pub fn calculate_percentage(&self) -> Option<f32> {
        let mut sum: f32 = 0.0;
        for iter in 0..self.marks.len() {
            sum += self.marks[iter];
        }
        Some(sum / self.marks.len() as f32)
    }
/// This function calculates grades of student and return an grade in [Option<char>]
/// 
pub fn calculate_grade(&self) -> Option<char> {
        // Here if percentage is 85 up this return an 'A'  
        if self.percentage > Some(85.0) {
            Some('A')
        // For 80 up this returns a 'B'
        } else if self.percentage > Some(80.0) {
            Some('B')
            // For 70 up this returns a 'C'
        } else if self.percentage > Some(70.0) {
            Some('C')
            // For 70 and below this returns a 'D'
        } else {
            Some('D')
        }
    }
}

#[derive(Debug, Deserialize, Serialize,PartialEq,Hash,Eq)]
/// This is a Employee struct
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub skills: Vec<String>,
    pub position: Option<Position>,
    #[serde(rename = "experiance(y)")]
    pub experiance_y: Option<u8>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq,Hash,Eq)]
/// This is a Position Enum of the employee struct
pub enum Position {
    #[serde(rename = "Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename = "Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename = "Software Developer")]
    SoftwareDeveloper,
    #[serde(rename = "Team Lead")]
    TeamLead,
    #[serde(rename = "Project Manager")]
    ProjectManager,
}
