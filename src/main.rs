use iced::{Settings, Sandbox, Length, Element, Font, window, Color, theme};
use iced::widget::{Row, Column, Container, Button, Text, Checkbox, TextInput, Scrollable,};  
use iced::alignment::{self, Alignment};
extern crate chrono;
use iced::theme::Theme;

pub mod tdtask; // второй файл для облегчения читаемости кода
use crate::tdtask::*;

pub const FONT_TANTULAR: Font = Font::External { name: "TantulaR", bytes: include_bytes!("../fonts/Tantular.ttf"), };

pub fn bluecurrasao() -> Theme {
	Theme::custom(theme::Palette{
					background: Color::from_rgb(0.89, 0.96, 0.95),
					text: Color::BLACK,
					primary: Color::from_rgb(0.62,0.84,0.85),
					success: Color::from_rgb(0.0, 1.0, 0.0),
					danger: Color::from_rgb(1.0, 0.0, 0.0),
	})
}
pub fn nigthsand() -> Theme {
	Theme::custom(theme::Palette{ 
					background: Color::from_rgb(0.0, 0.0, 0.0),
					text: Color::from_rgb(1.0, 0.65, 0.0),
					primary: Color::from_rgb(0.62,0.84,0.85),
					success: Color::from_rgb(0.0, 1.0, 0.0),
					danger: Color::from_rgb(1.0, 0.0, 0.0),
	})
}


fn main() -> Result<(), iced::Error> {
    ToDoApp::run(Settings {
		window: window::Settings {
			size: (1295, 700),
			..window::Settings::default()
		},
		..Settings::default()
	})
}

#[derive(Debug, Clone, Default)]
struct ToDoApp {
	vec_list_todo: Vec<ToDoTask>, 
	shown_todo: Vec<ToDoTask>,
	saved_todo: Vec<ToDoTask>,
	input_field: String,
	input_field_date: String,
	cb_importance_bool: bool, 
	list_of_todo: usize,
	text_of_error: String,
	redact_mode: Mode,
	input_field_reduct: String,
	input_field_reduct_date: String,
	t_help: String,
	theme: Theme,
}

impl ToDoApp {
	pub fn get_complited_bd(&mut self) {
		let mut result = vec![];
		for task in self.saved_todo.clone() {
			if task.is_compl {
				result.push(task);
			};
		};
		self.shown_todo = result;
	}
	
	pub fn get_uncomplited_bd(&mut self) {
		let mut result = vec![];
		for task in self.saved_todo.clone() {
			if !task.is_compl {
				result.push(task);
			};
		};
		self.shown_todo = result;
	}
	
	pub fn get_late_bd(&mut self) {
		let mut result = vec![];
		for task in self.saved_todo.clone() {
			if task.clone().is_late() {
				result.push(task);
			};
		};
		self.shown_todo = result;
	}
	
	pub fn get_importance_bd(&mut self) {
		let mut result = vec![];
		for task in self.saved_todo.clone() {
			if task.importance {
				result.push(task);
			};
		};
		self.shown_todo = result;
	}
	
	pub fn update_cb_shown_todo(&mut self) { 
		let mut result: Vec<ToDoTask> = vec![];
		for old_todo in self.shown_todo.clone() {
			result.push(self.saved_todo[old_todo.id_bd.clone()].clone());
		};
		self.shown_todo = result;
	}
	
	pub fn change_cb_message(&mut self, i: usize) {
		change_bd_cb(self.vec_list_todo[i].clone().id_bd); 
		self.saved_todo = get_all_bd();
		self.update_cb_shown_todo();
		self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
		self.text_of_error = String::from("");
	}
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
			theme: bluecurrasao(),
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
					Filter::Complited => self.get_complited_bd(), 
					Filter::Uncomplited => self.get_uncomplited_bd(),
					Filter::Late => self.get_late_bd(),
					Filter::Important => self.get_importance_bd(),
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
			TDStatus::NewtoFound(old_todo) => { 
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
				self.input_field = String::from("");
				self.input_field_date = String::from("");
				self.text_of_error = String::from("");
				self.redact_mode = Mode::Default;
				self.cb_importance_bool = false;
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
				let q = resave_todo(
					self.vec_list_todo[i_in_list].id_bd, 
					self.input_field_reduct.clone(), 
					self.input_field_reduct_date.clone(), 
					self.cb_importance_bool.clone()
					);
				match q {
					false => {self.text_of_error = String::from("Поля заполнены неверно!");},
					true => {
						self.redact_mode = Mode::Default; 
						self.input_field = String::from(""); 
						self.input_field_date = String::from("");
						self.saved_todo = get_all_bd();
						self.update_cb_shown_todo();
						self.vec_list_todo = update_vec_list(self.saved_todo.clone(), self.list_of_todo.clone());
						self.text_of_error = String::from("");
						self.cb_importance_bool = false;
					},
				};
			},
			TDStatus::ChangeCBImportance(b) => { self.cb_importance_bool = b; },
			TDStatus::Cancel => {
				self.redact_mode = Mode::Default; 
				self.input_field = String::from(""); 
				self.input_field_date = String::from("");
				self.text_of_error = String::from("");
				self.cb_importance_bool = false;
			},
			TDStatus::Help => { self.redact_mode = Mode::Help; },
			// расчитано на 10
			// сообщения от ЧекБоксов
			TDStatus::ChangeCB1(_) => { self.change_cb_message(0); },
			TDStatus::ChangeCB2(_) => { self.change_cb_message(1); },
			TDStatus::ChangeCB3(_) => { self.change_cb_message(2); },
			TDStatus::ChangeCB4(_) => { self.change_cb_message(3); },
			TDStatus::ChangeCB5(_) => { self.change_cb_message(4); },
			TDStatus::ChangeCB6(_) => { self.change_cb_message(5); },
			TDStatus::ChangeCB7(_) => { self.change_cb_message(6); },
			TDStatus::ChangeCB8(_) => { self.change_cb_message(7); },
			TDStatus::ChangeCB9(_) => { self.change_cb_message(8); },
			TDStatus::ChangeCB10(_) => { self.change_cb_message(9); },
			TDStatus::ChangeTheme => {
				if self.theme == bluecurrasao() {
				self.theme = nigthsand();
				} else {
					self.theme = bluecurrasao();
				}
			},
		}
    }

    fn view(&self) -> Element<'_, Self::Message> { // отрисовка графического интерфейса
		// --- РАЗДЕЛ ОБЩЕЕ ДЛЯ ВСЕХ МОДОВ
		// --- отдел нейминга
		let mut name = Text::new("ToDoTʘʘl").size(105).width(1260).horizontal_alignment(alignment::Horizontal::Center).font(FONT_TANTULAR);
		if self.theme == bluecurrasao() {
			name = name.style(Color::from_rgb(0.63,0.46,0.68));
		};

		// --- отдел фильтрации
				let filt_text = Text::new("Выберите фильтр:").font(FONT_TANTULAR).size(24);
				let show_all = Button::new(Text::new("Все").font(FONT_TANTULAR).size(28).width(160)).on_press(TDStatus::GetFiltred(Filter::All)); 
				let show_com = Button::new(Text::new("Завершённые").font(FONT_TANTULAR).size(28).width(160)).on_press(TDStatus::GetFiltred(Filter::Complited)); 
				let show_unc = Button::new(Text::new("Незавершённые").font(FONT_TANTULAR).size(28).width(160)).on_press(TDStatus::GetFiltred(Filter::Uncomplited)); 
				let show_late = Button::new(Text::new("Просроченные").font(FONT_TANTULAR).size(28).width(160)).on_press(TDStatus::GetFiltred(Filter::Late)); 
				let show_imp = Button::new(Text::new("Важные").font(FONT_TANTULAR).size(28).width(160)).on_press(TDStatus::GetFiltred(Filter::Important)); 
				let today_text =Text::new("Сегодня:").font(FONT_TANTULAR).size(24).width(161).height(24).horizontal_alignment(alignment::Horizontal::Center);
				let mut today = Text::new(get_now_day()).font(FONT_TANTULAR).height(85).size(28).width(160).horizontal_alignment(alignment::Horizontal::Center);
				if self.theme == bluecurrasao() {
					today = today.style(Color::from_rgb(0.63,0.46,0.68));
				};
				let theme_button = Button::new(Text::new(if self.theme == bluecurrasao() {"Тёмная тема"} else {"Светлая тема"})
					.font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center).width(160)).on_press(TDStatus::ChangeTheme);
				let t_less4 = Text::new("").height(120);
				let help_button = Button::new(Text::new("Помощь").font(FONT_TANTULAR).size(28)).on_press(TDStatus::Help);
				let f_col = Column::new()
					.push(filt_text).push(show_all).push(show_com).push(show_unc).push(show_late).push(show_imp)
					.push(today_text).push(today).push(theme_button).push(t_less4).push(help_button)
					.spacing(4).width(210).align_items(Alignment::Center);
		
		
		match self.redact_mode {
			// --- ОСНОВНОЙ РЕЖИМ
			Mode::Default => {
				// --- отдел с тудушками //расчитано на 10
				let found = Text::new(format!("Найдено результов по вашему запросу: {}", self.shown_todo.clone().len()))
					.font(FONT_TANTULAR).size(24).width(910).horizontal_alignment(alignment::Horizontal::Center);
				let found_row = Row::new().push(found).width(iced::Length::Fill).align_items(Alignment::Center); 
				// формирование строчек с задачами
				let mut show_result_col = Column::new().push(found_row)
					.spacing(4).align_items(Alignment::Start).height(473).width(980);
				if self.vec_list_todo.clone().len() == 0 {
				show_result_col = show_result_col.push(Text::new("Результатов по вашему запросу не найдено(")
					.font(FONT_TANTULAR).size(24)
					.height(130)
					.width(770)
					.horizontal_alignment(alignment::Horizontal::Center)
					.vertical_alignment(alignment::Vertical::Center));
				} else {
					for i in 0..=9 {
						if self.vec_list_todo.clone().len() >= (i.clone()+1) {
							let mut r = Row::new().spacing(11);
							match i.clone() {
								0 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB1)
										.size(24);
									r = r.push(cb);
								},
								1 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB2).size(24);
									r = r.push(cb);
								},
								2 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB3).size(24);
									r = r.push(cb);
								},
								3 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB4).size(24);
									r = r.push(cb);
								},
								4 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB5).size(24);
									r = r.push(cb);
								},
								5 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB6).size(24);
									r = r.push(cb);
								},
								6 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB7).size(24);
									r = r.push(cb);
								},
								7 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB8).size(24);
									r = r.push(cb);
								},
								8 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB9).size(24);
									r = r.push(cb);
								},
								9 => {
									let cb = Checkbox::new("", self.vec_list_todo[i.clone()].is_compl, TDStatus::ChangeCB10).size(24);
									r = r.push(cb);
								},
								_ => {},
							}
							let task_t = Text::new(self.vec_list_todo[i.clone()].clone().description)
								.font(FONT_TANTULAR).size(29).width(687);
							let b_d = Button::new(Text::new("...").font(FONT_TANTULAR).size(28)
								.horizontal_alignment(alignment::Horizontal::Center))
								.on_press(TDStatus::Redact(i.clone())).width(35);
							let mut date = Text::new(from_date_to_string(self.vec_list_todo[i.clone()].date.clone()))
								.font(FONT_TANTULAR).size(28).width(123).horizontal_alignment(alignment::Horizontal::Center);
							let imp = Text::new(if self.vec_list_todo[i.clone()].importance {"!!!"} else {""})
								.font(FONT_TANTULAR).size(28).width(18);
							if self.vec_list_todo[i.clone()].clone().is_late() { 
								date = date.style(Color::from_rgb(1.0, 0.0, 0.0)); 
								};
							r = r.push(task_t).push(imp).push(date).push(b_d);	
							show_result_col = show_result_col.push(r);
						}
					}
				};	
				let err = Text::new(self.text_of_error.clone())
					.font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center)
					.style(Color::from_rgb(1.0, 0.0, 0.0));
				show_result_col = show_result_col.push(err);
				
				// отдел листоперевёртывания
				let max_list: usize;
				match self.shown_todo.clone().len() { // расчёт max_list расчитано на 10
					0 => max_list = 1,
					_ if self.shown_todo.clone().len() % 10 == 0 => max_list = self.shown_todo.clone().len() / 10,
					_ => max_list = (self.shown_todo.clone().len() / 10) + 1,	
				};
				let t_less2 = Text::new("").width(210).size(28);
				let number_of_list = Text::new(format!("{}/{}", self.list_of_todo.clone() + 1, max_list))
					.font(FONT_TANTULAR).size(24).width(427).horizontal_alignment(alignment::Horizontal::Center);
				let number_of_list_row = Row::new().push(number_of_list);
				let b_nazad = Button::new(Text::new("<-").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::ChangeList(Napravlenie::Nazad)).width(35);
				let b_vpered = Button::new(Text::new("->").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::ChangeList(Napravlenie::Vpered)).width(35);
				let mut list_row = Row::new().spacing(7).align_items(Alignment::Center).push(t_less2);
		
				let t_less = Text::new("").size(28).width(35);
				if self.shown_todo.clone().len() < 11 { // настройка отображения <- _ ->
					list_row = list_row.push(t_less.clone()).push(number_of_list_row).push(t_less.clone()); // только номер страницы, т.к. результатов максимум 10
				} else if self.list_of_todo.clone() > 0 && (self.list_of_todo.clone() + 1) != max_list {
					list_row = list_row.push(b_nazad).push(number_of_list_row).push(b_vpered); // <- _ -> лист не первый, но и не последний
				} else if (self.list_of_todo.clone() + 1) == max_list {
					list_row = list_row.push(b_nazad).push(number_of_list_row).push(t_less.clone()); // <- _ лист последний
				} else {
					list_row = list_row.push(t_less.clone()).push(number_of_list_row).push(b_vpered); // _ -> всё остальное
				};
				
	
				// --- отдел с инпутом
				let input = TextInput::new("Введите текст новой задачи / текст для её поиска", &self.input_field)
					.on_input(TDStatus::ChangeTextField).font(FONT_TANTULAR).size(28).width(550).padding(4);
				let input_date = TextInput::new("31/12/2023", &self.input_field_date).on_input(TDStatus::ChangeDateField)
					.font(FONT_TANTULAR).size(28).width(122).padding(4);
				let cb_importance = Checkbox::new("!!!", self.cb_importance_bool, TDStatus::ChangeCBImportance).font(FONT_TANTULAR).size(32).text_size(28);
				let add_button = Button::new(Text::new("Сохранить").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::NewToDoGet(self.input_field.clone(), self.input_field_date.clone(), self.cb_importance_bool.clone())).width(119);
				let search_button = Button::new(Text::new("Найти").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::NewtoFound(self.input_field.clone())).width(84).padding(4);
				let inp_row = Row::new().push(input).push(input_date).push(cb_importance).push(add_button).push(search_button).spacing(14);			
		
				// --- отдел формирования интерфейса
				let right_col = Column::new().push(inp_row).push(show_result_col).push(list_row).spacing(4);
				let main_row = Row::new().push(f_col).push(right_col).spacing(7);
				let main_col = Column::new().push(name).push(main_row).spacing(7);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into() //отцентровка и преобразование в Элемент с помощью инто			
			},
			// --- РЕЖИМ РЕДАКТИРОВАНИЯ
			Mode::Redact(i_in_list) => {
					
				//--- отдел с инпутом
				let input = TextInput::new("Введите текст обновлённой задачи", &self.input_field_reduct)
					.on_input(TDStatus::ChangeTextField).font(FONT_TANTULAR).size(28).width(550).padding(4);
				let input_date = TextInput::new("31/12/2023", &self.input_field_reduct_date)
					.on_input(TDStatus::ChangeDateField).font(FONT_TANTULAR).size(28).width(122).padding(4);
				let cb_importance = Checkbox::new("!!!", self.cb_importance_bool, TDStatus::ChangeCBImportance)
					.font(FONT_TANTULAR).size(32).text_size(28);
				let resave_button = Button::new(Text::new("Сохранить").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::Resave(i_in_list)).width(119);
				let delete_button = Button::new(Text::new("Удалить").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::Delete(i_in_list)).width(119);
				let err = Text::new(self.text_of_error.clone()).font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center);
				let inp_row = Row::new().push(input).push(input_date).push(cb_importance).push(resave_button).push(delete_button).spacing(14);	
		
				// --- отдел формирования интерфейса
				let cancel_button = Button::new(Text::new("Отмена").font(FONT_TANTULAR).size(28).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::Cancel).width(119);
				let c_row = Row::new().push(Text::new("").width(776)).push(cancel_button).width(980).align_items(Alignment::End);
				let right_col = Column::new().push(inp_row).push(err).push(c_row).spacing(4);
				let main_row = Row::new().push(f_col).push(right_col).spacing(7);
				let main_col = Column::new().push(name).push(main_row).spacing(7);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into() //отцентровка и преобразование в Элемент с помощью инто
			},
			
			// --- РЕЖИМ РУКОВОДСТВА ПОЛЬЗОВАТЕЛЯ
			Mode::Help => {
				let t_help = Text::new(self.t_help.clone()).font(FONT_TANTULAR).size(24).width(980).horizontal_alignment(alignment::Horizontal::Center);
				let cancel_button = Button::new(Text::new("Назад к программе").font(FONT_TANTULAR).size(24).horizontal_alignment(alignment::Horizontal::Center))
					.on_press(TDStatus::Cancel).width(210);
				let c_row = Row::new().push(Text::new("").width(525)).push(cancel_button).width(1316).align_items(Alignment::End);
				let t_less3 = Text::new("").height(21);
				let t_col_help = Column::new().push(t_help).push(c_row).push(t_less3).width(1260).spacing(14).align_items(Alignment::Center);
				let scrol = Scrollable::new(t_col_help);
				
				let main_col = Column::new().push(name).push(scrol).spacing(11).width(1260).align_items(Alignment::Center);
				Container::new(main_col).width(Length::Fill).height(Length::Fill).center_x().center_y().into()
			},
		}
    }
	
	fn theme(&self) -> Theme {
        self.theme.clone()
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
	ChangeTheme,
}
