extern crate glium;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use glium::glutin;
use glium::Surface;

fn main() {
    // Obtem os argumentos do terminal
    let args: Vec<String> = env::args().collect();

    // Abre o arquivo
    let rom_file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            panic!("Deu bosta na leitura do arquivo: {}", err)
        }
    };

    // Realiza a leitura do arquivo inteiro
    // let mut content = String::new();
    // match rom_file.read_to_string(&mut content) {
    //    Ok(size) => println!("O tamanho do arquivo é {}", size),
    //    Err(_) => println!("Whay")
    // };

    // Lê a entrada como um stream
    for byte in rom_file.bytes() {
        println!("{}", match byte {
            Ok(val) => val,
            Err(_) => 0
        });
    }

    // Instancia os objetos necessarios para tela
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().
        with_dimensions(240, 160).
        with_title("GBA Emulator");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut closed = false;

    // Muda a cor do fundo para azul
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1., 1.);
    target.finish().unwrap();

    // Loop de execução do programa
    while !closed {
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, .. } => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
