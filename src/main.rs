#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};
use std::collections::HashMap;

trait GtkUtils: GtkUtilsFn {
    fn new() -> GtkConf;
    fn init(&self);
    fn define_topWin(&mut self, title: &str, xsize: i32, ysize: i32);
    fn add_box(&mut self, name: &'static str, space: i32);
    fn add_widget(&mut self, winName: &'static str, boxName: &'static str, labelName: &'static str, widgetType: &'static str, expand: bool, fill: bool, padding: u32);
    fn add_labelWidget(&mut self, winName: &'static str, boxName: &'static str, labelName: &'static str, expand: bool, fill: bool, padding: u32);
    fn make_labelWidget(&mut self, name: &'static str, text: &str);
    fn make_buttonWidget(&mut self, name: &'static str, text: &str);
    fn add_buttonWidget(&mut self, winName: &'static str, boxName: &'static str, buttonName: &'static str, expand: bool, fill: bool, padding: u32);
    fn add_winCloseEvent(&self, winName: &'static str, closure: std::ops::Fn(gtk::Window, gdk::Event));
}

trait GtkUtilsFn {
    type WindowInTrait;
    type EventInTrait;
    fn closure(&self, window: gtk::Window, event: gdk::Event);
}

struct GtkConf {
    wins: HashMap<&'static str, gtk::Window>,
    boxes: HashMap<&'static str, gtk::Box>,
    labels: HashMap<&'static str, gtk::Label>,
    buttons: HashMap<&'static str, gtk::Button>,
    addedBoxes: HashMap<&'static str, bool>,
}

impl<F> GtkUtilsFn for F where F: std::ops::Fn(gtk::Window, gdk::Event) {
    type WindowInTrait = gtk::Window;
    type EventInTrait = gdk::Event;
    fn closure(&self, window: Self::WindowInTrait, event: Self::EventInTrait) {
        (self)(window, event);
    }
}

impl GtkUtils for GtkConf {
    fn new() -> GtkConf {
        let tmpWins = HashMap::new();
        let tmpBoxes = HashMap::new();
        let tmpWidgets = HashMap::new();
        let tmpButtons = HashMap::new();
        let tmpAddedBoxes = HashMap::new();
        GtkConf {
            wins: tmpWins,
            boxes: tmpBoxes,
            labels: tmpWidgets,
            buttons: tmpButtons,
            addedBoxes: tmpAddedBoxes,
        }
    }
    fn init(&self) {
        if gtk::init().is_err() {
            println!("Faild to initialize GTK");
            return;
        }
    }
    fn define_topWin(&mut self, title: &str, xsize: i32, ysize: i32) {
        let topWin = Window::new(WindowType::Toplevel);
        topWin.set_title(title);
        topWin.set_default_size(xsize, ysize);
        self.wins.insert("top", topWin);
    }
    fn add_box(&mut self, boxName: &'static str, space: i32) {
        let newBox = gtk::Box::new(gtk::Orientation::Vertical, space);
        self.boxes.insert(boxName, newBox);
    }
    fn add_widget(&mut self, winName: &'static str, boxName: &'static str, widgetName: &'static str, widgetType: &'static str, expand: bool, fill: bool, padding: u32) {
        match widgetType {
            "label" => {
                self.boxes[boxName].pack_start(&self.labels[widgetName], expand, fill, padding);
            },
            "button" => {
                self.boxes[boxName].pack_start(&self.buttons[widgetName], expand, fill, padding);
            },
            other => {
                println!("{} is not implemented", other);
                std::process::exit(1);
            }
        }
        match self.addedBoxes.get(boxName) {
            Some(is_added) => {
                if *is_added == true {
                    self.wins[winName].remove(&self.boxes[boxName]);
                }
            },
            None => {
            }
        }
        self.addedBoxes.insert(boxName, true);
        self.wins[winName].add(&self.boxes[boxName]);
    }
    fn add_labelWidget(&mut self, winName: &'static str, boxName: &'static str, labelName: &'static str, expand: bool, fill: bool, padding: u32) {
        self.add_widget(winName, boxName, labelName, "label", expand, fill, padding);
    }
    fn add_buttonWidget(&mut self, winName: &'static str, boxName: &'static str, buttonName: &'static str, expand: bool, fill: bool, padding: u32) {
        self.add_widget(winName, boxName, buttonName, "button", expand, fill, padding);
    }
    fn make_labelWidget(&mut self, name: &'static str, text: &str) {
        let label = gtk::Label::new(text);
        self.labels.insert(name, label);
    }
    fn make_buttonWidget(&mut self, name: &'static str, text: &str) {
        let button = gtk::Button::new_with_label(text);
        self.buttons.insert(name, button);
    }
    fn add_winCloseEvent(&self, winName: &'static str, event: Self::EventInTrait) {
        //self.wins[winName].connect_delete_event();
        println!("hoge");
    }
}

fn main(){
    // 管理構造体の用意
    let mut g = GtkConf::new();

    // 初期化
    g.init();

    // 最上位のウィンドウを作成
    g.define_topWin("count up", 300, 400);
    
    // 複数のwidgetを格納するBoxを作成
    g.add_box("vbox", 2);

    // 各widgetの作成
    g.make_labelWidget("label", "0"); // labelの作成
    g.add_labelWidget("top", "vbox", "label", true, true, 3); // labelの追加
    
    // 初期表示
    g.wins["top"].show_all();

    // Windowを閉じたときのeventの設定
    // ここを関数化したい！
    // しかし、クロージャを引数にとるため実装が難しい！
    // 例えば次のような実装にしたい。
    // g.add_winCloseEvent("top", |_, _| {
    //     println!("hoge");
    // });
    g.wins["top"].connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // buttonをクリックしたときのeventの設定
    let label_c = g.labels["label"].clone();
    g.buttons["countUp_button"].clone().connect_clicked(move |_| {
        let old_num: u16 = label_c.get_text().unwrap().to_string().parse::<u16>().unwrap();
        g.labels["label"].set_text(&(old_num + 1).to_string());
        println!("カウント+1");
    });

    // GUIの実行
    gtk::main();
}

