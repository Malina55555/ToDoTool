use iced::{Settings, Sandbox, TextInput, Length, Element, Font, window, Color};
use iced::widget::{Row, Column, Container, Button, Text, Checkbox, Scrollable}; 
use std::fs;
use iced::alignment::{self, Alignment};
extern crate chrono;

pub mod tdtask; // второй файл для облегчения читаемости кода
use crate::tdtask::*; // 

pub const FONT_TANTULAR: Font = Font::External {
	name: "TantulaR",
	bytes: include_bytes!("../fonts/Tantular.ttf"),
};

fn main() -> Result<(), iced::Error> {
    ToDoApp::run(Settings {
		window: window::Settings {
			size: (1850, 1000),
			..window::Settings::default()
		},
		..Settings::default()
	})
}

#[derive(Debug, Clone, Default)]
struct ToDoApp { //расчитано на 10
	vec_list_todo: Vec<ToDoTask>, 
	shown_todo: Vec<ToDoTask>,
	saved_todo: Vec<ToDoTask>,
	input_field: String,
	input_field_date: String,
	input_state: iced::widget::text_input::State,
	input_state_date: iced::widget::text_input::State,
	cb_importance_bool: bool, 
	b_all_state: iced::widget::button::State,
	b_com_state: iced::widget::button::State,
	b_unc_state: iced::widget::button::State,
	b_late_state: iced::widget::button::State,
	b_imp_state: iced::widget::button::State,
	b_add_state: iced::widget::button::State,
	b_src_state: iced::widget::button::State,
	list_of_todo: usize,
	b_nazad_state: iced::widget::button::State,
	b_vpered_state: iced::widget::button::State,
	b_red_1_state: iced::widget::button::State,
	b_red_2_state: iced::widget::button::State,
	b_red_3_state: iced::widget::button::State,
	b_red_4_state: iced::widget::button::State,
	b_red_5_state: iced::widget::button::State,
	b_red_6_state: iced::widget::button::State,
	b_red_7_state: iced::widget::button::State,
	b_red_8_state: iced::widget::button::State,
	b_red_9_state: iced::widget::button::State,
	b_red_10_state: iced::widget::button::State,
	text_of_error: String,
	redact_mode: Mode,
	input_field_reduct: String,
	input_state_reduct: iced::widget::text_input::State,
	input_field_reduct_date: String,
	input_state_reduct_date: iced::widget::text_input::State,
	b_save_state: iced::widget::button::State,
	b_delete_state: iced::widget::button::State,
	b_cancel_state: iced::widget::button::State,
	b_help_state: iced::widget::button::State,
	sl: iced::widget::scrollable::State,
	t_help: String,
}

impl Sandbox for ToDoApp {
	type Message = TDStatus;
	
	fn new() -> Self { //инициализация окна при запуске
		ToDoApp {
			shown_todo: get_all_bd(),
			saved_todo: get_all_bd(),
			vec_list_todo: update_vec_list(get_all_bd(), 0),
			list_of_todo: 0,
			redact_mode: Mode::Default,
			cb_importance_bool: false,
			t_help: return_help(),
			..ToDoApp::default()
		}
    }

    fn title(&self) -> String {  // наименование окна
        String::from("ToDoTool")
    }

    fn update(&mut self, message: Self::Message) { // получает сообщение и изменяет текущее состояние
        match message {
			
			TDStatus::GetFiltred(some_filter) => {
				match some_filter {
					Filter::All => self.shown_todo = self.saved_todo.clone(),
					Filter::Complited => self.shown_todo = get_complited_bd(self.saved_todo.clone()),
					Filter::Uncomplited => self.shown_todo = get_uncomplited_bd(self.saved_todo.clone()),
					Filter::Late => self.shown_todo = get_late_bd(self.saved_todo.clone()),
					Filter::Important => self.shown_todo = get_importance_bd(self.saved_todo.clone()),
				};
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.text_of_error = String::from("");
			},
			
			TDStatus::ChangeTextField(value) => {
				self.input_field = value.clone();
				self.input_field_reduct = value;
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeDateField(value) => {
				self.input_field_date = value.clone();
				self.input_field_reduct_date = value;
				self.text_of_error = String::from("");
			},
			TDStatus::NewToDoGet(new_todo, new_date, new_importance) => {
				let result = add_new_todo(&new_todo, &new_date, new_importance);
				match result {
					true => {
						self.saved_todo = get_all_bd();
						self.shown_todo = get_all_bd();
						self.vec_list_todo = update_vec_list(self.saved_todo.clone(), 0);
						self.input_field = String::from("");
						self.input_field_date = String::from("");
						self.list_of_todo = 0;
						self.text_of_error = String::from("");
						self.cb_importance_bool = false;
					},
					false => {
						self.text_of_error = String::from("Поля заполнены неверно!");
					},
				}
			},
			TDStatus::NewtoFound(old_todo) => { //?
				self.shown_todo = search_todo(&old_todo);
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);	
				self.text_of_error = String::from("");
			},	
			
			TDStatus::ChangeList(some_napravlenie) => {
				match some_napravlenie {
					Napravlenie::Vpered => self.list_of_todo += 1,
					Napravlenie::Nazad => self.list_of_todo -= 1,
				};
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			
			
			TDStatus::Delete(i_in_list) => {
				delete_from_bd(self.vec_list_todo[i_in_list].clone().id_bd);
				self.saved_todo = get_all_bd();
				self.shown_todo = get_all_bd();
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.list_of_todo = 0;
				self.text_of_error = String::from("");
				self.redact_mode = Mode::Default;
			},
			TDStatus::Redact(i_in_list) => {
				self.redact_mode = Mode::Redact(i_in_list);
				self.input_field_reduct = self.vec_list_todo[i_in_list].clone().description;
				self.cb_importance_bool = self.vec_list_todo[i_in_list].clone().importance;
				match self.vec_list_todo[i_in_list].clone().date {
					None => {self.input_field_reduct_date = String::from("");},
					Some(s) => self.input_field_reduct_date = from_date_to_string(Some(s)),
				};
			},
			TDStatus::Resave(i_in_list) => {
				let q = resave_todo(self.vec_list_todo[i_in_list].id_bd, self.input_field_reduct.clone(), self.input_field_reduct_date.clone(), self.cb_importance_bool.clone());
				match q {
					false => {self.text_of_error = String::from("Поля заполнены неверно!");},
					true => {
						self.redact_mode = Mode::Default; 
						self.input_field = String::from(""); 
						self.input_field_date = String::from("");
						self.saved_todo = get_all_bd();
						self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
						self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
						self.text_of_error = String::from("");
						self.cb_importance_bool = false;
					},
				};
			},
			TDStatus::ChangeCBImportance(b) => {
				self.cb_importance_bool = b;
			},
			TDStatus::Cancel => {
				self.redact_mode = Mode::Default; 
				self.input_field = String::from(""); 
				self.input_field_date = String::from("");
				self.text_of_error = String::from("");
				self.cb_importance_bool = false;
			},
			TDStatus::Help => {
				self.redact_mode = Mode::Help;
			},
			// расчитано на 10
			// сообщения от ЧекБоксов
			TDStatus::ChangeCB1(_) => { 
				change_bd_cb(self.vec_list_todo[0].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB2(_) => {
				change_bd_cb(self.vec_list_todo[1].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB3(_) => {
				change_bd_cb(self.vec_list_todo[2].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB4(_) => {
				change_bd_cb(self.vec_list_todo[3].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB5(_) => {
				change_bd_cb(self.vec_list_todo[4].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB6(_) => { 
				change_bd_cb(self.vec_list_todo[5].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB7(_) => {
				change_bd_cb(self.vec_list_todo[6].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB8(_) => {
				change_bd_cb(self.vec_list_todo[7].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB9(_) => {
				change_bd_cb(self.vec_list_todo[8].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeCB10(_) => {
				change_bd_cb(self.vec_list_todo[9].clone().id_bd); 
				self.saved_todo = get_all_bd();
				self.shown_todo = update_cb_shown_todo(self.saved_todo.clone(), self.shown_todo.clone());
				self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
		}
    }

    fn view(&mut self) -> Element<'_, Self::Message> { // отрисовка графического интерфейса
		match self.redact_mode {
			Mode::Default => {
				// --- отдел с тудушками //расчитано на 10
				let found = Text::new(format!("Найдено результов по вашему запросу: {}", self.shown_todo.clone().len())).color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(35).width(Length::Units(1300)).horizontal_alignment(alignment::Horizontal::Center);
				let found_row = Row::new().push(found).width(iced::Length::Fill).align_items(Alignment::Center); 
				let mut show_result_col = Column::new().push(found_row).spacing(5).align_items(Alignment::Start).height(Length::Units(650)).width(Length::Units(1400)); //.explain(Color::from_rgb(0.98, 0.65, 0.0));
				if self.vec_list_todo.clone().len() == 0 {
				show_result_col = show_result_col.push(Text::new("Результатов по вашему запросу не найдено(")
					.font(FONT_TANTULAR).size(35)
					.height(Length::Units(200))
					.width(Length::Units(1100))
					.horizontal_alignment(alignment::Horizontal::Center)
					.vertical_alignment(alignment::Vertical::Center));
				} else {
				// формирование строчек с задачами
					if self.vec_list_todo.clone().len() >= 1 {
						let cb = Checkbox::new(self.vec_list_todo[0].is_compl, "", TDStatus::ChangeCB1).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[0].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_1_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(0)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[0].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[0].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[0].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);
					};
					if self.vec_list_todo.clone().len() >= 2 {
						let cb = Checkbox::new(self.vec_list_todo[1].is_compl, "", TDStatus::ChangeCB2).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[1].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_2_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(1)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[1].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[1].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[1].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);
					};
					if self.vec_list_todo.clone().len() >= 3 {
						let cb = Checkbox::new(self.vec_list_todo[2].is_compl, "", TDStatus::ChangeCB3).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[2].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_3_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(2)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[2].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[2].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[2].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);			
					};
					if self.vec_list_todo.clone().len() >= 4 {
						let cb = Checkbox::new(self.vec_list_todo[3].is_compl, "", TDStatus::ChangeCB4).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[3].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_4_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(3)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[3].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[3].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[3].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
					if self.vec_list_todo.clone().len() >= 5 {
						let cb = Checkbox::new(self.vec_list_todo[4].is_compl, "", TDStatus::ChangeCB5).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[4].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_5_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(4)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[4].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[4].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[4].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
					if self.vec_list_todo.clone().len() >= 6 {
						let cb = Checkbox::new(self.vec_list_todo[5].is_compl, "", TDStatus::ChangeCB6).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[5].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_6_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(5)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[5].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[5].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[5].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
					if self.vec_list_todo.clone().len() >= 7 {
						let cb = Checkbox::new(self.vec_list_todo[6].is_compl, "", TDStatus::ChangeCB7).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[6].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_7_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(6)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[6].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[6].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[6].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
					if self.vec_list_todo.clone().len() >= 8 {
						let cb = Checkbox::new(self.vec_list_todo[7].is_compl, "", TDStatus::ChangeCB8).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[7].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_8_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(7)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[7].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[7].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[7].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};	
					if self.vec_list_todo.clone().len() >= 9 {
						let cb = Checkbox::new(self.vec_list_todo[8].is_compl, "", TDStatus::ChangeCB9).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[8].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_9_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(8)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[8].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[8].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[8].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
					if self.vec_list_todo.clone().len() >= 10 {
						let cb = Checkbox::new(self.vec_list_todo[9].is_compl, "", TDStatus::ChangeCB10).font(FONT_TANTULAR).size(35);
						let dscr = Text::new(&self.vec_list_todo[9].clone().description).size(40).font(FONT_TANTULAR).width(Length::Units(1001));
						let b_d = Button::new(&mut self.b_red_10_state, Text::new("...").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Redact(9)).width(Length::Units(50));
						let mut date = Text::new(from_date_to_string(self.vec_list_todo[9].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
						let imp = Text::new(if self.vec_list_todo[9].importance {"!!!"} else {""}).font(FONT_TANTULAR).size(40).width(Length::Units(25)).color(Color::from_rgb(1.0,0.0,0.0));
						if is_late(self.vec_list_todo[9].clone()) { date = date.color(Color::from_rgb(1.0, 0.0, 0.0)); };
						let r = Row::new().push(cb).push(dscr).push(imp).push(date).push(b_d).spacing(15);	
						show_result_col = show_result_col.push(r);	
					};
				};	
				let err = Text::new(self.text_of_error.clone()).font(FONT_TANTULAR).size(40).color(Color::from_rgb(1.0, 0.0, 0.0)).horizontal_alignment(alignment::Horizontal::Center);
				show_result_col = show_result_col.push(err);
				
				// отдел листоперевёртывания
				let mut max_list = 0;
				match self.shown_todo.clone().len() { // расчёт max_list расчитано на 10
					0 => max_list = 1,
					_ if self.shown_todo.clone().len() % 10 == 0 => max_list = self.shown_todo.clone().len() / 10,
					_ => max_list = (self.shown_todo.clone().len() / 10) + 1,	
				};
				let t_less2 = Text::new("").width(Length::Units(300)).size(40);
				let number_of_list = Text::new(format!("{}/{}", self.list_of_todo.clone() + 1, max_list)).font(FONT_TANTULAR).size(35).width(Length::Units(610)).horizontal_alignment(alignment::Horizontal::Center);
				let number_of_list_row = Row::new().push(number_of_list);
				let b_nazad = Button::new(&mut self.b_nazad_state, Text::new("<-").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::ChangeList(Napravlenie::Nazad)).width(Length::Units(50));
				let b_vpered = Button::new(&mut self.b_vpered_state, Text::new("->").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::ChangeList(Napravlenie::Vpered)).width(Length::Units(50));
				let mut list_row = Row::new().spacing(10).align_items(Alignment::Center).push(t_less2);
		
				let t_less = Text::new("").size(40).width(Length::Units(50));
				if self.shown_todo.clone().len() < 11 { // настройка отображения <- _ ->
					list_row = list_row.push(t_less.clone()).push(number_of_list_row).push(t_less.clone()); // только номер страницы, т.к. результатов максимум 10
				} else if self.list_of_todo.clone() > 0 && (self.list_of_todo.clone() + 1) != max_list {
					list_row = list_row.push(b_nazad).push(number_of_list_row).push(b_vpered); // <- _ -> лист не первый, но и не последний
				} else if (self.list_of_todo.clone() + 1) == max_list {
					list_row = list_row.push(b_nazad).push(number_of_list_row).push(t_less.clone()); // <- _ лист последний
				} else {
					list_row = list_row.push(t_less.clone()).push(number_of_list_row).push(b_vpered); // _ -> всё остальное
				};
				// --- отдел фильтрации
				let filt_text = Text::new("Выберите фильтр:").font(FONT_TANTULAR).size(35);
				let show_all = Button::new(&mut self.b_all_state, Text::new("Все").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::All)); 
				let show_com = Button::new(&mut self.b_com_state, Text::new("Завершённые").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Complited)); 
				let show_unc = Button::new(&mut self.b_unc_state, Text::new("Незавершённые").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Uncomplited)); 
				let show_late = Button::new(&mut self.b_late_state, Text::new("Просроченные").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Late)); 
				let show_imp = Button::new(&mut self.b_imp_state, Text::new("Важные").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Important)); 
				let today_text =Text::new("Сегодня:").font(FONT_TANTULAR).size(35).width(Length::Units(230)).height(Length::Units(35)).horizontal_alignment(alignment::Horizontal::Center);
				let today = Text::new(get_now_day()).color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).height(Length::Units(348)).size(40).width(Length::Units(230)).horizontal_alignment(alignment::Horizontal::Center);
				let help_button = Button::new(&mut self.b_help_state, Text::new("Помощь").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40)).on_press(TDStatus::Help);
				let f_col = Column::new().push(filt_text).push(show_all).push(show_com).push(show_unc).push(show_late).push(show_imp).push(today_text).push(today).push(help_button).spacing(5).width(Length::Units(300)).align_items(Alignment::Center);
		
				// --- отдел с инпутом
				let input = TextInput::new(&mut self.input_state, "Введите текст новой задачи / текст для её поиска", &self.input_field, TDStatus::ChangeTextField).font(FONT_TANTULAR).size(40).width(Length::Units(800)).padding(5);
				let input_date = TextInput::new(&mut self.input_state_date, "31/12/2023", &self.input_field_date, TDStatus::ChangeDateField).font(FONT_TANTULAR).size(40).width(Length::Units(160)).padding(5);
				let cb_importance = Checkbox::new(self.cb_importance_bool, "!!!", TDStatus::ChangeCBImportance).font(FONT_TANTULAR).size(45).text_size(40);
				let add_button = Button::new(&mut self.b_add_state, Text::new("Сохранить").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::NewToDoGet(self.input_field.clone(), self.input_field_date.clone(), self.cb_importance_bool.clone())).width(Length::Units(170));
				let search_button = Button::new(&mut self.b_src_state, Text::new("Поиск").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::NewtoFound(self.input_field.clone())).width(Length::Units(120)).padding(5);
				let inp_row = Row::new().push(input).push(input_date).push(cb_importance).push(add_button).push(search_button).spacing(20);			
		
				// --- отдел нейминга
				let name = Text::new("ToDoTʘʘl").size(150).color(Color::from_rgb(0.3, 0.3, 0.65)).width(Length::Units(1800)).horizontal_alignment(alignment::Horizontal::Center).font(FONT_TANTULAR);
				//0.0, 0.0, 0.6   0.01, 0.56, 0.61
				// --- отдел формирования интерфейса
				let right_col = Column::new().push(inp_row).push(show_result_col).push(list_row).spacing(5);
				let main_row = Row::new().push(f_col).push(right_col).spacing(10);
				let main_col = Column::new().push(name).push(main_row).spacing(10);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into() //отцентровка и преобразование в Элемент с помощью инто			
			},
			Mode::Redact(i_in_list) => {
				// --- отдел фильтрации
				let filt_text = Text::new("Выберите фильтр:").font(FONT_TANTULAR).size(35);
				let show_all = Button::new(&mut self.b_all_state, Text::new("Все").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::All)); 
				let show_com = Button::new(&mut self.b_com_state, Text::new("Завершённые").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Complited)); 
				let show_unc = Button::new(&mut self.b_unc_state, Text::new("Незавершённые").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Uncomplited)); 
				let show_late = Button::new(&mut self.b_late_state, Text::new("Просроченные").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Late)); 
				let show_imp = Button::new(&mut self.b_imp_state, Text::new("Важные").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Important)); 
				let today_text =Text::new("Сегодня:").font(FONT_TANTULAR).size(35).width(Length::Units(230)).height(Length::Units(35)).horizontal_alignment(alignment::Horizontal::Center);
				let today = Text::new(get_now_day()).color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).height(Length::Units(348)).size(40).width(Length::Units(230)).horizontal_alignment(alignment::Horizontal::Center);
				let help_button = Button::new(&mut self.b_help_state, Text::new("Помощь").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40)).on_press(TDStatus::Help);
				let f_col = Column::new().push(filt_text).push(show_all).push(show_com).push(show_unc).push(show_late).push(show_imp).push(today_text).push(today).push(help_button).spacing(5).width(Length::Units(300)).align_items(Alignment::Center);
		
				//--- отдел с инпутом
				let input = TextInput::new(&mut self.input_state_reduct, "Введите текст обновлённой задачи", &self.input_field_reduct, TDStatus::ChangeTextField).font(FONT_TANTULAR).size(40).width(Length::Units(800)).padding(5);
				let input_date = TextInput::new(&mut self.input_state_reduct_date, "31/12/2023", &self.input_field_reduct_date, TDStatus::ChangeDateField).font(FONT_TANTULAR).size(40).width(Length::Units(160)).padding(5);
				let cb_importance = Checkbox::new(self.cb_importance_bool, "!!!", TDStatus::ChangeCBImportance).font(FONT_TANTULAR).size(45).text_size(40);
				let resave_button = Button::new(&mut self.b_save_state, Text::new("Сохранить").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Resave(i_in_list)).width(Length::Units(170));
				let delete_button = Button::new(&mut self.b_delete_state, Text::new("Удалить").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Delete(i_in_list)).width(Length::Units(170));
				let err = Text::new(self.text_of_error.clone()).font(FONT_TANTULAR).size(40).color(Color::from_rgb(1.0, 0.0, 0.0)).horizontal_alignment(alignment::Horizontal::Center);
				let inp_row = Row::new().push(input).push(input_date).push(cb_importance).push(resave_button).push(delete_button).spacing(20);	
		
				// --- отдел нейминга
				let name = Text::new("ToDoTʘʘl").size(150).color(Color::from_rgb(0.01, 0.56, 0.61)).width(Length::Units(1800)).horizontal_alignment(alignment::Horizontal::Center).font(FONT_TANTULAR);
		
				// --- отдел формирования интерфейса
				let cancel_button = Button::new(&mut self.b_cancel_state, Text::new("Отмена").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Cancel).width(Length::Units(150));
				let c_row = Row::new().push(Text::new("").width(Length::Units(1132))).push(cancel_button).width(Length::Units(1400)).align_items(Alignment::End);
				
				let right_col = Column::new().push(inp_row).push(err).push(c_row).spacing(5);
				let main_row = Row::new().push(f_col).push(right_col).spacing(10);
				let main_col = Column::new().push(name).push(main_row).spacing(10);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into() //отцентровка и преобразование в Элемент с помощью инто
			},
			Mode::Help => {
				let t_help = Text::new(self.t_help.clone()).font(FONT_TANTULAR).size(35).width(Length::Units(1400)).horizontal_alignment(alignment::Horizontal::Center);
				let cancel_button = Button::new(&mut self.b_cancel_state, Text::new("Назад к программе").color(Color::from_rgb(0.0, 0.0, 0.6)).font(FONT_TANTULAR).size(35).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::Cancel).width(Length::Units(300));
				let c_row = Row::new().push(Text::new("").width(Length::Units(750))).push(cancel_button).width(Length::Units(1880)).align_items(Alignment::End);
				let t_less3 = Text::new("").height(Length::Units(30));
				let t_col_help = Column::new().push(t_help).push(c_row).push(t_less3).width(Length::Units(1800)).spacing(20).align_items(Alignment::Center);
				let scrol = Scrollable::new(&mut self.sl).push(t_col_help);
				
				let main_col = Column::new().push(scrol).spacing(15).width(Length::Units(1800)).align_items(Alignment::Center);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
			},
		}
    }
}

#[derive(Debug, Clone)]
enum TDStatus {    /// расчитано на 10
	GetFiltred(Filter),
	ChangeTextField(String),
	ChangeDateField(String),
	NewToDoGet(String, String, bool),
	NewtoFound(String),
	ChangeList(Napravlenie),
	ChangeCB1(bool),
	ChangeCB2(bool),
	ChangeCB3(bool),
	ChangeCB4(bool),
	ChangeCB5(bool),
	ChangeCB6(bool),
	ChangeCB7(bool),
	ChangeCB8(bool),
	ChangeCB9(bool),
	ChangeCB10(bool),
	Delete(usize),
	Redact(usize),
	Resave(usize),
	ChangeCBImportance(bool),
	Cancel,
	Help,
}

#[derive(Debug, Clone)]
enum Napravlenie {
	Vpered,
	Nazad,
}

#[derive(Debug, Clone, Copy)]
enum Filter {
	All,
	Complited,
	Uncomplited,
	Late,
	Important,
}

#[derive(Debug, Clone, Default)]
enum Mode {
	#[default]
	Default,
	Redact(usize),
	Help,
}

fn get_all_bd() -> Vec<ToDoTask> {
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

fn add_new_todo(stroka: &String, str_date: &String, importance: bool) -> bool {
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

fn search_todo(stroka: &String) -> Vec<ToDoTask> { //?
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

fn change_bd_cb(i: usize) {
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

fn delete_from_bd(i: usize) {
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

fn check_date_string(stroka: String) -> bool {
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


fn resave_todo(given_i: usize, new_str: String, new_date: String, new_importance: bool) -> bool {
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

fn return_help() -> String {
	let content_help = fs::read_to_string("help.txt").expect("Что-то не так с help.txt");
	return content_help
}
