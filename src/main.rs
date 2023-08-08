use iced::{Settings, Sandbox, TextInput, Length, Element, Font, window, Color};
use iced::widget::{Row, Column, Container, Button, Text, Checkbox}; 
use std::fs;
use iced::alignment::{self, Alignment};
extern crate chrono;

pub mod tdtask; // второй файл для облегчения читаемости кода
use crate::tdtask::{ToDoTask, update_vec_list, update_cb_shown_todo, get_complited_bd, get_uncomplited_bd, from_date_to_string, is_late, get_late_bd, from_string_to_date, get_now_day};

pub const FONT_TANTULAR: Font = Font::External {
	name: "TantulaR",
	bytes: include_bytes!("../fonts/Tantular.ttf"),
};

fn main() -> Result<(), iced::Error> {
    ToDoApp::run(Settings {
		window: window::Settings {
			size: (1800, 1000),
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
	b_all_state: iced::widget::button::State,
	b_com_state: iced::widget::button::State,
	b_unc_state: iced::widget::button::State,
	b_late_state: iced::widget::button::State,
	b_add_state: iced::widget::button::State,
	b_src_state: iced::widget::button::State,
	list_of_todo: usize,
	b_nazad_state: iced::widget::button::State,
	b_vpered_state: iced::widget::button::State,
	b_del_1_state: iced::widget::button::State,
	b_del_2_state: iced::widget::button::State,
	b_del_3_state: iced::widget::button::State,
	b_del_4_state: iced::widget::button::State,
	b_del_5_state: iced::widget::button::State,
	b_del_6_state: iced::widget::button::State,
	b_del_7_state: iced::widget::button::State,
	b_del_8_state: iced::widget::button::State,
	b_del_9_state: iced::widget::button::State,
	b_del_10_state: iced::widget::button::State,
	text_of_error: String,
}

impl Sandbox for ToDoApp {
	type Message = TDStatus;
	
	fn new() -> Self { //инициализация окна при запуске
		ToDoApp {
			shown_todo: get_all_bd(),
			saved_todo: get_all_bd(),
			vec_list_todo: update_vec_list(get_all_bd(), 0),
			list_of_todo: 0,
			text_of_error: String::from(""),
			..ToDoApp::default()
		}
    }

    fn title(&self) -> String {  // наименование окна
        String::from("ToDoTool")
    }

    fn update(&mut self, message: Self::Message) { // получает сообщение и изменяет текущее состояние
        match message {
			TDStatus::GetFiltred(Filter::All) => {
				self.shown_todo = self.saved_todo.clone();
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.text_of_error = String::from("");
			},
			TDStatus::GetFiltred(Filter::Complited) => {
				self.shown_todo = get_complited_bd(self.saved_todo.clone());
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.text_of_error = String::from("");
			},
			TDStatus::GetFiltred(Filter::Uncomplited) => {
				self.shown_todo = get_uncomplited_bd(self.saved_todo.clone());
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.text_of_error = String::from("");
			},
			TDStatus::GetFiltred(Filter::Late) => {
				self.shown_todo = get_late_bd(self.saved_todo.clone());
				self.list_of_todo = 0;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), 0);
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeTextField(value) => {
				self.input_field = value;
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeDateField(value) => {
				self.input_field_date = value;
				self.text_of_error = String::from("");
			},
			TDStatus::NewToDoGet(new_todo, new_date) => {
				let result = add_new_todo(&new_todo, &new_date);
				match result {
					true => {
						self.saved_todo = get_all_bd();
						self.shown_todo = get_all_bd();
						self.vec_list_todo = update_vec_list(self.saved_todo.clone(), 0);
						self.input_field = String::from("");
						self.input_field_date = String::from("");
						self.list_of_todo = 0;
						self.text_of_error = String::from("");
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
			TDStatus::ChangeList(Napravlenie::Vpered) => {
				self.list_of_todo += 1;
				self.vec_list_todo = update_vec_list(self.shown_todo.clone(), self.list_of_todo.clone());
				self.text_of_error = String::from("");
			},
			TDStatus::ChangeList(Napravlenie::Nazad) => {
				self.list_of_todo -= 1;
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
		// --- отдел с тудушками //расчитано на 10
		let found = Text::new(format!("Найдено результов по вашему запросу: {}", self.shown_todo.clone().len())).font(FONT_TANTULAR).size(35).width(Length::Units(1100)).horizontal_alignment(alignment::Horizontal::Center);
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
				let cb_1 = Checkbox::new(self.vec_list_todo[0].is_compl, &self.vec_list_todo[0].clone().description, TDStatus::ChangeCB1).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_1 = Button::new(&mut self.b_del_1_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(0));
				if is_late(self.vec_list_todo[0].clone()) {
					let date1 = Text::new(from_date_to_string(self.vec_list_todo[0].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r1 = Row::new().push(cb_1).push(date1).push(b_d_1).spacing(30);
					show_result_col = show_result_col.push(r1);
				} else {
					let date1 = Text::new(from_date_to_string(self.vec_list_todo[0].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r1 = Row::new().push(cb_1).push(date1).push(b_d_1).spacing(30);
					show_result_col = show_result_col.push(r1);
				};
			};
			if self.vec_list_todo.clone().len() >= 2 {
				let cb_2 = Checkbox::new(self.vec_list_todo[1].is_compl, &self.vec_list_todo[1].clone().description, TDStatus::ChangeCB2).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_2 = Button::new(&mut self.b_del_2_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(1));
				if is_late(self.vec_list_todo[1].clone()) {
					let date2 = Text::new(from_date_to_string(self.vec_list_todo[1].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r2 = Row::new().push(cb_2).push(date2).push(b_d_2).spacing(30);
					show_result_col = show_result_col.push(r2);
				} else {
					let date2 = Text::new(from_date_to_string(self.vec_list_todo[1].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r2 = Row::new().push(cb_2).push(date2).push(b_d_2).spacing(30);
					show_result_col = show_result_col.push(r2);};
				
			};
			if self.vec_list_todo.clone().len() >= 3 {
				let cb_3 = Checkbox::new(self.vec_list_todo[2].is_compl, &self.vec_list_todo[2].clone().description, TDStatus::ChangeCB3).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_3 = Button::new(&mut self.b_del_3_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(2));
				if is_late(self.vec_list_todo[2].clone()) {
					let date3 = Text::new(from_date_to_string(self.vec_list_todo[2].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r3 = Row::new().push(cb_3).push(date3).push(b_d_3).spacing(30);
					show_result_col = show_result_col.push(r3);
				} else {
					let date3 = Text::new(from_date_to_string(self.vec_list_todo[2].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r3 = Row::new().push(cb_3).push(date3).push(b_d_3).spacing(30);
					show_result_col = show_result_col.push(r3);
				};				
			};
			if self.vec_list_todo.clone().len() >= 4 {
				let cb_4 = Checkbox::new(self.vec_list_todo[3].is_compl, &self.vec_list_todo[3].clone().description, TDStatus::ChangeCB4).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_4 = Button::new(&mut self.b_del_4_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(3));
				if is_late(self.vec_list_todo[3].clone()) {
					let date4 = Text::new(from_date_to_string(self.vec_list_todo[3].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r4 = Row::new().push(cb_4).push(date4).push(b_d_4).spacing(30);
					show_result_col = show_result_col.push(r4);
				} else {
					let date4 = Text::new(from_date_to_string(self.vec_list_todo[3].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r4 = Row::new().push(cb_4).push(date4).push(b_d_4).spacing(30);
					show_result_col = show_result_col.push(r4);
				};
			};
			if self.vec_list_todo.clone().len() >= 5 {
				let cb_5 = Checkbox::new(self.vec_list_todo[4].is_compl, &self.vec_list_todo[4].clone().description, TDStatus::ChangeCB5).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_5 = Button::new(&mut self.b_del_5_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(4));
				if is_late(self.vec_list_todo[4].clone()) {
					let date5 = Text::new(from_date_to_string(self.vec_list_todo[4].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r5 = Row::new().push(cb_5).push(date5).push(b_d_5).spacing(30);
					show_result_col = show_result_col.push(r5);
				} else {
					let date5 = Text::new(from_date_to_string(self.vec_list_todo[4].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r5 = Row::new().push(cb_5).push(date5).push(b_d_5).spacing(30);
					show_result_col = show_result_col.push(r5);
				};
			};
			if self.vec_list_todo.clone().len() >= 6 {
				let cb_6 = Checkbox::new(self.vec_list_todo[5].is_compl, &self.vec_list_todo[5].clone().description, TDStatus::ChangeCB6).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_6 = Button::new(&mut self.b_del_6_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(5));
				if is_late(self.vec_list_todo[5].clone()) {
					let date6 = Text::new(from_date_to_string(self.vec_list_todo[5].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r6 = Row::new().push(cb_6).push(date6).push(b_d_6).spacing(30);
					show_result_col = show_result_col.push(r6);
				} else {
					let date6 = Text::new(from_date_to_string(self.vec_list_todo[5].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r6 = Row::new().push(cb_6).push(date6).push(b_d_6).spacing(30);
					show_result_col = show_result_col.push(r6);
				};
			};
			if self.vec_list_todo.clone().len() >= 7 {
				let cb_7 = Checkbox::new(self.vec_list_todo[6].is_compl, &self.vec_list_todo[6].clone().description, TDStatus::ChangeCB7).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_7 = Button::new(&mut self.b_del_7_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(6));
				
				if is_late(self.vec_list_todo[6].clone()) {
					let date7 = Text::new(from_date_to_string(self.vec_list_todo[6].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r7 = Row::new().push(cb_7).push(date7).push(b_d_7).spacing(30);
					show_result_col = show_result_col.push(r7);
				} else {
					let date7 = Text::new(from_date_to_string(self.vec_list_todo[6].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r7 = Row::new().push(cb_7).push(date7).push(b_d_7).spacing(30);
					show_result_col = show_result_col.push(r7);
				};
			};
			if self.vec_list_todo.clone().len() >= 8 {
				let cb_8 = Checkbox::new(self.vec_list_todo[7].is_compl, &self.vec_list_todo[7].clone().description, TDStatus::ChangeCB8).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_8 = Button::new(&mut self.b_del_8_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(7));
				if is_late(self.vec_list_todo[7].clone()) {
					let date8 = Text::new(from_date_to_string(self.vec_list_todo[7].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r8 = Row::new().push(cb_8).push(date8).push(b_d_8).spacing(30);
					show_result_col = show_result_col.push(r8);
				} else {
					let date8 = Text::new(from_date_to_string(self.vec_list_todo[7].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r8 = Row::new().push(cb_8).push(date8).push(b_d_8).spacing(30);
					show_result_col = show_result_col.push(r8);
				};
			};
			if self.vec_list_todo.clone().len() >= 9 {
				let cb_9 = Checkbox::new(self.vec_list_todo[8].is_compl, &self.vec_list_todo[8].clone().description, TDStatus::ChangeCB9).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_9 = Button::new(&mut self.b_del_9_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(8));
				if is_late(self.vec_list_todo[8].clone()) {
					let date9 = Text::new(from_date_to_string(self.vec_list_todo[8].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r9 = Row::new().push(cb_9).push(date9).push(b_d_9).spacing(30);
					show_result_col = show_result_col.push(r9);
				} else {
					let date9 = Text::new(from_date_to_string(self.vec_list_todo[8].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r9 = Row::new().push(cb_9).push(date9).push(b_d_9).spacing(30);
					show_result_col = show_result_col.push(r9);
				};
				
			};
			if self.vec_list_todo.clone().len() >= 10 {
				let cb_10 = Checkbox::new(self.vec_list_todo[9].is_compl, &self.vec_list_todo[9].clone().description, TDStatus::ChangeCB10).font(FONT_TANTULAR).size(35).text_size(35).width(Length::Units(972));
				let b_d_10 = Button::new(&mut self.b_del_10_state, Text::new("Удалить").font(FONT_TANTULAR).size(40)).on_press(TDStatus::Delete(9));
				if is_late(self.vec_list_todo[9].clone()) {
					let date10 = Text::new(from_date_to_string(self.vec_list_todo[9].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(1.0, 0.0, 0.0));
					let r10 = Row::new().push(cb_10).push(date10).push(b_d_10).spacing(30);
					show_result_col = show_result_col.push(r10);
				} else {
					let date10 = Text::new(from_date_to_string(self.vec_list_todo[9].date.clone())).font(FONT_TANTULAR).size(40).width(Length::Units(175)).horizontal_alignment(alignment::Horizontal::Center);
					let r10 = Row::new().push(cb_10).push(date10).push(b_d_10).spacing(30);
					show_result_col = show_result_col.push(r10);
				};
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
		let number_of_list = Text::new(format!("{}/{}", self.list_of_todo.clone() + 1, max_list)).font(FONT_TANTULAR).size(35).width(Length::Units(1210)).horizontal_alignment(alignment::Horizontal::Center);
		let number_of_list_row = Row::new().push(number_of_list);
		let b_nazad = Button::new(&mut self.b_nazad_state, Text::new("<-").font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::ChangeList(Napravlenie::Nazad)).width(Length::Units(50));
		let b_vpered = Button::new(&mut self.b_vpered_state, Text::new("->").font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::ChangeList(Napravlenie::Vpered)).width(Length::Units(50));
		let mut list_row = Row::new().spacing(10).align_items(Alignment::Center);
		
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
		let show_all = Button::new(&mut self.b_all_state, Text::new("Все").font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::All)); 
		let show_com = Button::new(&mut self.b_com_state, Text::new("Завершённые").font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Complited)); 
		let show_unc = Button::new(&mut self.b_unc_state, Text::new("Незавершённые").font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Uncomplited)); 
		let show_late = Button::new(&mut self.b_late_state, Text::new("Просроченные").font(FONT_TANTULAR).size(40).width(Length::Units(230))).on_press(TDStatus::GetFiltred(Filter::Late)); 
		let today_text =Text::new("Сегодня:").font(FONT_TANTULAR).size(35).width(Length::Units(230)).height(Length::Units(35)).horizontal_alignment(alignment::Horizontal::Center);
		let today = Text::new(get_now_day()).font(FONT_TANTULAR).size(40).width(Length::Units(230)).height(Length::Units(40)).horizontal_alignment(alignment::Horizontal::Center).color(Color::from_rgb(0.0, 0.0, 1.0));
		let f_col = Column::new().push(filt_text).push(show_all).push(show_com).push(show_unc).push(show_late).push(today_text).push(today).spacing(5).width(Length::Units(300)).align_items(Alignment::Center);
		
		// --- отдел с инпутом
		let input = TextInput::new(&mut self.input_state, "Введите текст новой задачи / текст для её поиска", &self.input_field, TDStatus::ChangeTextField).font(FONT_TANTULAR).size(40).width(Length::Units(800)).padding(5);
		let input_date = TextInput::new(&mut self.input_state_date, "31/12/2023", &self.input_field_date, TDStatus::ChangeDateField).font(FONT_TANTULAR).size(40).width(Length::Units(160)).padding(5);
		let add_button = Button::new(&mut self.b_add_state, Text::new("Сохранить").font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::NewToDoGet(self.input_field.clone(), self.input_field_date.clone())).width(Length::Units(170));
		let search_button = Button::new(&mut self.b_src_state, Text::new("Поиск").font(FONT_TANTULAR).size(40).horizontal_alignment(alignment::Horizontal::Center)).on_press(TDStatus::NewtoFound(self.input_field.clone())).width(Length::Units(120)).padding(5);
		let inp_row = Row::new().push(input).push(input_date).push(add_button).push(search_button).spacing(20);	
		
		// --- отдел нейминга
		let name = Text::new("ToDoTool").size(150).color(Color::from_rgb(0.01, 0.56, 0.61)).width(Length::Units(1800)).horizontal_alignment(alignment::Horizontal::Center);
		
		// --- отдел формирования интерфейса
		let right_col = Column::new().push(inp_row).push(show_result_col).push(list_row).spacing(5);
		let main_row = Row::new().push(f_col).push(right_col).spacing(10);
		let main_col = Column::new().push(name).push(main_row).spacing(10);
		Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into() //отцентровка и преобразование в Элемент с помощью инто
    }
}

#[derive(Debug, Clone)]
enum TDStatus {    /// расчитано на 10
	GetFiltred(Filter),
	ChangeTextField(String),
	ChangeDateField(String),
	NewToDoGet(String, String),
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
		});
		id_tracker += 1;
	};
	result
}

fn add_new_todo(stroka: &String, str_date: &String) -> bool {
	let content_buf = fs::read_to_string("buf.txt").expect("Что-то не так с buf.txt");
	if stroka.trim() == "" {
		return false //строка пуста
	}
	let stroka_trim = stroka.trim();
	match str_date.clone().len() {
		0 => {
			let _w = fs::write("buf.txt", content_buf + "f --/--/---- " + stroka_trim + "\n");
			return true
		},
		10 => {
			if check_date_string(str_date.clone()) {
				
				let _w = fs::write("buf.txt", content_buf + "f " + str_date + " " + stroka_trim + "\n");
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

