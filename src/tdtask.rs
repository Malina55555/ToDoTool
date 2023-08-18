extern crate chrono;
use chrono::{Datelike, Utc};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub struct ToDoTask {
	pub description: String,
	pub is_compl: bool,
	pub id_bd: usize,
	pub date: Option<Date>,
	pub importance: bool,
}

impl ToDoTask {
	pub fn is_late(&self) -> bool {
		if self.is_compl {
			return false
		} else {
			match self.date.clone() {
				None => return false,
				Some(given_date) => return date_is_earlier_then_today(given_date) ,
			}
		}
	}
}

pub fn update_vec_list(v: Vec<ToDoTask>, i_list: usize) -> Vec<ToDoTask> { //расчитано на 10
	let mut result: Vec<ToDoTask> = vec![];
	
	if v.len() == 0 { return result };
	
	for i in 0..=9 {
		if v.len() >= i_list.clone() * 10 + i + 1 {
			result.push(v[i + i_list.clone()*10].clone());
		} else {return result};
	}
	
	return result
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
	pub day: u32,
	pub month: u32,
	pub year: u32,
}

pub fn get_now_day() -> String {
	let now = Utc::now();
	let (_, year) = now.year_ce();
	let now_day = format!("{:02}/{:02}/{}", now.day(), now.month(), year);
	return now_day
}

pub fn from_date_to_string(date: Option<Date>) -> String {
	let mut result = String::from("");
	match date {
		None => return "--/--/----".to_string(),
		Some(d) => {
			result += &(format!("{:02}/{:02}/{}", d.day, d.month, d.year))[..];
			return result
		}
	}
}

pub fn date_is_earlier_then_today(given: Date) -> bool {
	let today_date = from_string_to_date(get_now_day()).unwrap();
	if today_date.year > given.year {
		return true
	} else if today_date.month > given.month {
		return true
	} else if today_date.day > given.day {
		return true
	} else {
		return false
	}
}

pub fn from_string_to_date(stroka: String) -> Option<Date> {
	let mut v = vec![];
	for c in stroka.chars() {
		v.push(c);
	};
	let mut v2 = vec![];
	for i in 0..10 {
		if i != 2 && i != 5 {
			let q = v[i].clone().to_digit(10);
			match q {
				None => return None,
				Some(w) => v2.push(w),
			};
		};
	};
	Some(Date{
		day: (v2[0].clone()*10 + v2[1].clone()) as u32,
		month: (v2[2].clone()*10 + v2[3].clone()) as u32,
		year: (v2[4].clone()*1000 + v2[5].clone()*100 + v2[6].clone()*10 + v2[7].clone()) as u32,
	})
}



pub fn get_all_bd() -> Vec<ToDoTask> {
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	let mut result = vec![];
	let mut id_tracker: usize = 0;
	for line in content_buf.lines() {
		result.push(ToDoTask{
			id_bd: id_tracker.clone(),
			description: line[13..].to_string(),
			date: from_string_to_date(line[2..12].to_string()),
			is_compl: if &line[..1] == "t" {true} else {false},
			importance: if &line[1..2] == "!" {true} else {false},
		});
		id_tracker += 1;
	};
	result
}

pub fn return_help() -> String {
	let content_help = fs::read_to_string("help.txt").expect("Что-то не так с help.txt");
	return content_help
}

pub fn check_date_string(stroka: String) -> bool {
	let mut v = vec![];
	for c in stroka.chars() {
		v.push(c);
	};
	let mut v2 = vec![];
	for i in 0..10 {
		if i != 2 && i != 5 {
			let q = v[i].clone().to_digit(10);
			match q {
				None => return false,
				Some(w) => v2.push(w),
			};
		} else {
			if v[i] != '/' {
				return false
			}
		};
	};
	let year = v2[4].clone()*1000 + v2[5].clone()*100 + v2[6].clone()*10 + v2[7].clone();
	let month = v2[2].clone()*10 + v2[3].clone();
	let day = v2[0].clone()*10 + v2[1].clone();
	if v2[0] > 3 { //40 day
		return false
	} else if day > 31 || day == 0 { //32+ day  00 day
		return false
	}
	if month > 12 { // month 13+
		return false
	}
	//фев
	if (year % 4 == 0 && month == 2 && day > 29) || (year % 4 != 0 && month == 2 && day > 28) {
		return false
	}
	//апр, июн, сен, ноя
	if (month == 4 || month == 6 || month == 9 || month == 11) && day > 30 {
		return false
	}
	return true
}

pub fn delete_from_bd(i: usize) {
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	let mut id_tracker: usize = 0;
	let mut result = String::from("");
	for stroka in content_buf.lines() {
		if i != id_tracker {
			result += &stroka[..]; 
			result += "\n";
		};
		id_tracker += 1;
 	};
	let _w = fs::write("buf.txt", result.to_string());
}

#[derive(Debug, Clone, Default)]
pub enum Mode {
	#[default]
	Default,
	Redact(usize),
	Help,
}

pub fn resave_todo(given_i: usize, new_str: String, new_date: String, new_importance: bool) -> bool {
	if new_date.clone() != "".to_string() && !check_date_string(new_date.clone()) {
		return false
	};
	if new_str == String::from("") {return false};
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	let mut id_tracker: usize = 0;
	let mut result = String::from("");
	let mut new_stroka = String::from("");
	for stroka in content_buf.lines() {
		if given_i == id_tracker {
			new_stroka += &stroka[..1]; 
			new_stroka += if new_importance {"!"} else {" "};
			new_stroka += if new_date == "".to_string() {"--/--/----"} else {&new_date[..]};
			new_stroka += " ";
			new_stroka += &new_str[..];
			new_stroka += "\n";
			result += &new_stroka[..];
		} else {
			result += &stroka[..]; 
			result += "\n";
		};
		id_tracker += 1;
 	};
	let _w = fs::write("buf.txt", result.to_string());
	return true
}

pub fn add_new_todo(stroka: &String, str_date: &String, importance: bool) -> bool {
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	if stroka.trim() == "" {
		return false //строка пуста
	}
	let stroka_trim = stroka.trim();
	let imp = if importance {"!"} else {" "};
	match str_date.clone().len() {
		0 => {
			let _w = fs::write("buf.txt", content_buf + "f" + imp + "--/--/---- " + stroka_trim + "\n");
			return true
		},
		10 => {
			if check_date_string(str_date.clone()) {
				
				let _w = fs::write("buf.txt", content_buf + "f" + imp + str_date + " " + stroka_trim + "\n");
				return true
			} else {
				return false //не соответствует шаблону
			}
		},
		_ => return false, //дата не соответствует шаблону
	}
}

pub fn search_todo(stroka: &String) -> Vec<ToDoTask> { 
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	let mut result = vec![];
	let mut id_tracker: usize = 0;
	for line in content_buf.lines() {
		if line.contains(stroka) {
			result.push(ToDoTask{
				id_bd: id_tracker.clone(),
				description: line[13..].to_string(),
				date: from_string_to_date(line[2..12].to_string()),
				is_compl: if &line[..1] == "t" {true} else {false},
				importance: if &line[1..2] == "!" {true} else {false},
			});
		};
		id_tracker += 1;
	};
	result
}

pub fn change_bd_cb(i: usize) {
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	let mut id_tracker: usize = 0;
	let mut result = String::from("");
	let mut new_stroka = String::from("");
	for stroka in content_buf.lines() {
		if i == id_tracker {
			if &stroka[..1] == "t" {
				new_stroka += "f";
			} else {
				new_stroka += "t";
			};
			new_stroka += &stroka[1..]; 
			new_stroka += "\n";
			result += &new_stroka[..];
		} else {
			result += &stroka[..]; 
			result += "\n";
		};
		id_tracker += 1;
 	};
	let _w = fs::write("buf.txt", result.to_string());
}

#[derive(Debug, Clone)]
pub enum Napravlenie {
	Vpered,
	Nazad,
}

#[derive(Debug, Clone, Copy)]
pub enum Filter {
	All,
	Complited,
	Uncomplited,
	Late,
	Important,
}
