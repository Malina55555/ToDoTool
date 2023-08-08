extern crate chrono;
use chrono::{Datelike, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct ToDoTask {
	pub description: String,
	pub is_compl: bool,
	pub id_bd: usize,
	pub date: Option<Date>,
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

pub fn update_cb_shown_todo(all_task: Vec<ToDoTask>, old_vec: Vec<ToDoTask>) -> Vec<ToDoTask> {
	let mut result: Vec<ToDoTask> = vec![];
	for old_todo in old_vec {
		result.push(all_task[old_todo.id_bd.clone()].clone());
	};
	return result
}

pub fn get_complited_bd(all_task: Vec<ToDoTask>) -> Vec<ToDoTask> {
	let mut result = vec![];
	for task in all_task {
		if task.is_compl {
			result.push(task);
		};
	};
	result
}

pub fn get_uncomplited_bd(all_task: Vec<ToDoTask>) -> Vec<ToDoTask> {
	let mut result = vec![];
	for task in all_task {
		if !task.is_compl {
			result.push(task);
		};
	};
	result
}

pub fn get_late_bd(all_task: Vec<ToDoTask>) -> Vec<ToDoTask> {
	let mut result = vec![];
	for task in all_task {
		if is_late(task.clone()) {
			result.push(task);
		};
	};
	result
}

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
	pub day: u32,
	pub month: u32,
	pub year: u32,
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

pub fn is_late(given: ToDoTask) -> bool {
	if given.is_compl {
		return false
	} else {
		match given.date {
			None => return false,
			Some(given_date) => return date_is_earlier_then_today(given_date) ,
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

pub fn get_now_day() -> String {
	let now = Utc::now();
	let (_, year) = now.year_ce();
	let now_day = format!("{:02}/{:02}/{}", now.day(), now.month(), year);
	return now_day
}