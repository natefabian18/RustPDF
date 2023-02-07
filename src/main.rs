use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::fmt;

extern crate genpdf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CellValue {
    Double(f64),
    String(String),
	Null(())
}

impl Default for CellValue {
	fn default() -> Self { CellValue::Null(()) }
}

impl fmt::Display for CellValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let result: String;
		match self {
			CellValue::Double(val) => result = format!("{:?}", val),
			CellValue::String(val) => result = format!("{}", val),
			CellValue::Null(_val) => result = format!("")
		}

		write!(f, "{}", result)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Cell {
	#[serde(default)]
	value: CellValue,
	#[serde(default)]
	format: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ReportData {
	name: String,
	age: i32,
	data: Vec<Vec<Cell>>
}

#[derive(Serialize, Deserialize, Debug)]
struct SampleData {
	name: String,
	age: i32,
	data: Vec<Vec<i32>>
}

#[get("/heartBeat")]
async fn greet() -> impl Responder {
	format!("Hello Actix server working!")
}


#[post("/JSON/SampleData")]
async fn json_test(info: web::Json<ReportData>) -> String {
	println!("{:#?}", info);
	let name = &info.name;
	let age = &info.age;
	format!("Hello {name} you tell me you are {age} years old")
}

#[post("/JSON/ReportData")]
async fn process_report(info: web::Json<ReportData>) -> String {
	println!("/JSON/ReportData received");
	let generate_doc = Instant::now();

	let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
	.expect("Failed to load font family");

	let mut doc = genpdf::Document::new(font_family);

	let col_count = 9;

	let mut table = genpdf::elements::TableLayout::new(vec![1,1,1,1,1,1,1,1,1]);
	table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

	for row in &info.data {
		let mut table_row = table.row();

		for cell in row {
			let text = cell.value.to_string();
			table_row.push_element(genpdf::elements::Paragraph::new(text));
		}

		let mut cell_counter = row.len();

		while cell_counter < col_count {
			table_row.push_element(genpdf::elements::Paragraph::new("".to_string()));
			cell_counter += 1;
		}

		table_row.push().expect("Invalid table row");
	}

	doc.push(table);

	let time_to_gen = generate_doc.elapsed();
	println!("Time to generate: {:#?}", time_to_gen);
	let write_doc = Instant::now();

	doc.render_to_file("output.pdf").expect("Failed to render document");

	let time_to_write = write_doc.elapsed();
	println!("time to write: {:#?}", time_to_write);

	format!("Report data received")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
	let bind = "127.0.0.1";
	let port = 8081;

	
	println!("Server Running on {bind}:{port}");
    HttpServer::new(|| {
		let json_cfg = web::JsonConfig::default()
		.limit(1024 * 1024 * 1024);
		
        App::new()
		.app_data(json_cfg)
		.service(greet)
		.service(json_test)
		.service(process_report)
    })
    .bind((bind, port))?
    .run()
    .await
}
