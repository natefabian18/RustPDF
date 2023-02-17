use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};
use std::{time::Instant};
use std::fs;

mod types;
extern crate genpdf;

#[get("/heartBeat")]
async fn greet() -> impl Responder {
	let mut dummy: Vec<i32> = Vec::new();

	for x in 1..100000000 {
		dummy.push(x + 2);
	}

	format!("Hello, {}!", dummy.len())
}

#[post("/JSON/ReportData")]
async fn process_report(info: web::Json<types::ReportData>) -> HttpResponse {
	println!("/JSON/ReportData received");
	let generate_doc = Instant::now();

	let font_family = genpdf::fonts::from_files("./fonts", "Roboto", None)
	.expect("Failed to load font family");

	let mut doc = genpdf::Document::new(font_family);

	let mut decorator = genpdf::SimplePageDecorator::new();
	decorator.set_margins(10);
	doc.set_page_decorator(decorator);

	let col_count = 8;

	let mut table = genpdf::elements::TableLayout::new(vec![1,1,1,1,1,1,1,1]);
	table.set_cell_decorator(genpdf::elements::FrameCellDecorator::new(true, true, false));

	for row in &info.data {
		let mut table_row = table.row();

		for cell in row {
			let text = cell.value.to_string();
			let paragraph = genpdf::elements::Paragraph::new(text);
			let element = genpdf::elements::PaddedElement::new(
				paragraph,
				genpdf::Margins::trbl(1, 1, 1, 1)
			);
			table_row.push_element(element);
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

	let file_name = (String::from("./") + &info.name + ".pdf").to_string();


	//This could be refactored to return a stream and avoid writting files in this service but getting that to work is outside of my ability right now

	doc.render_to_file(&file_name).expect("Failed to render document");

	let time_to_write = write_doc.elapsed();
	println!("time to write: {:#?}", time_to_write);

	let duff = fs::read(&file_name).unwrap();

	//return fileBuffer;
	let res = HttpResponse::Ok().body(duff);

	return res;
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
		.service(process_report)
    })
    .bind((bind, port))?
    .run()
    .await
}
