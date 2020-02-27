//extern crate clap;
//extern crate unskew;
//extern crate convolution;
//extern crate face_detection;

/*extern crate termion;
use termion::color;
use termion::raw::IntoRawMode;
use std::io::{Read, Write, stdout, stdin};*/

/*use unskew::Unskew;
use face_detection::FaceDetection;

use clap::{
  crate_version, 
  crate_authors, 
  Arg, 
  App, 
  SubCommand, 
  AppSettings
};
*/

extern crate termion;
extern crate regex;
extern crate liner;

use liner::{Completer, Context, FilenameCompleter};
use regex::Regex;
use termion::color;
use std::io;

struct CommentCompleter {
  inner: Option<FilenameCompleter>,
}


// This prints out the text back onto the screen
fn highlight_dodo(s: &str) -> String {
  let reg_exp = Regex::new("(?P<k>dodo)").unwrap();
  let format = format!("{}$k{}", color::Fg(color::Red), color::Fg(color::Reset));
  reg_exp.replace_all(s, format.as_str()).to_string()
}


impl Completer for CommentCompleter {
  fn completions(&mut self, start: &str) -> Vec<String> {
      if let Some(inner) = &mut self.inner {
          inner.completions(start)
      } else {
          Vec::new()
      }
  }
}


fn main() {

  let mut con = Context::new();
    let mut completer = CommentCompleter { inner: None };


    loop {
      let res = con.read_line(
        "[prompt]\n% ",
        Some(Box::new(highlight_dodo)),
        &mut completer,
      );

      match res {
        Ok(res) => {
            match res.as_str() {
                "emacs" => {
                    con.key_bindings = liner::KeyBindings::Emacs;
                    println!("emacs mode");
                }
                "vi" => {
                    con.key_bindings = liner::KeyBindings::Vi;
                    println!("vi mode");
                }
                "exit" | "" => {
                    println!("exiting...");
                    break;
                }
                // If all else fails, do nothing
                _ => {}
            }

            // If we typed nothing, don't continue down to pushing to history
            if res.is_empty() {
                break;
            }

            con.history.push(res.into()).unwrap();
        }
        // If there was an error, get what type it was(remember, we still are in the match{}
        // from waaay above)
        Err(e) => {
            match e.kind() {
                // ctrl-c pressed
                io::ErrorKind::Interrupted => {}
                // ctrl-d pressed
                io::ErrorKind::UnexpectedEof => {
                    println!("exiting...");
                    break;
                }
                _ => {
                    // Ensure that all writes to the history file
                    // are written before exiting due to error.
                    panic!("error: {:?}", e)
                }
            }
        }
    }

    }


  /*let stdout = stdout();
  let mut stdout = stdout.lock().into_raw_mode().unwrap();
  let stdin = stdin();
  let stdin = stdin.lock();

  write!(stdout,
    "{}{}{}Vizion{}{}",
    termion::clear::All,
    termion::cursor::Goto(3, 3),
    termion::style::Bold,
    termion::style::Reset,
    termion::cursor::Goto(3, 5))
     .unwrap();
  stdout.flush().unwrap();

  let mut bytes = stdin.bytes();
  loop {
    let b = bytes.next().unwrap().unwrap();

    match b {
          // Quit
          b'q' => return,
          // Clear the screen
          b'c' => write!(stdout, "{}", termion::clear::All),
          // Set red color
          b'r' => write!(stdout, "{}", color::Fg(color::Rgb(5, 0, 0))),
          // Write it to stdout.
          a => write!(stdout, "{}", a),
      }
     .unwrap();

  stdout.flush().unwrap();

  }*/

  /*let matches = App::new("Vizion")
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::ColoredHelp)
    .version(crate_version!())
    .author(crate_authors!())
    .about("Computer vision library")

    .subcommand(SubCommand::with_name("unskew")
      .about("Unskews an image with text")
      .version("0.1.0")
      .author(crate_authors!())
      .arg(Arg::with_name("SRC")
        .help("Source file")
        .required(true)
        .index(1))
      .arg(Arg::with_name("DST")
          .help("Destination file")
          .required(true)
          .index(2)))
    
    .subcommand(SubCommand::with_name("face-detection")
      .about("Facedetector")
      .version("0.1.0")
      .author(crate_authors!())
      .arg(Arg::with_name("SRC")
        .help("Input image")
        .required(true)
        .index(1))
      .arg(Arg::with_name("DST")
        .help("Destination file")
        .required(true)
        .index(2)))

    .subcommand(SubCommand::with_name("convolution")
        .about("Convolution")
        .version("0.1.0")
        .author(crate_authors!())
        .arg(Arg::with_name("SRC")
          .help("Input image")
          .required(true)
          .index(1))
        .arg(Arg::with_name("DST")
          .help("Destination file")
          .required(true)
          .index(2)))
    .get_matches();*/

    /*if let Some(ref matches) = matches.subcommand_matches("unskew") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();

      let image = Unskew::new(&src, &dst)
      .grayscale()
      .invert()
      .pad()
      .unskew()
      .save();

      println!("Result: {:?}", image);
    }

    if let Some(ref matches) = matches.subcommand_matches("face-detection") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();
     
      let face = FaceDetection::new(&src, &dst)
        .detect()
        .draw()
        .save();

        println!("Result: {:?}", face);
    }

    if let Some(ref matches) = matches.subcommand_matches("convolution") {
      let src = matches.value_of("SRC").unwrap();
      let dst = matches.value_of("DST").unwrap();

      let img = convolution::Img::new(src);

      let conv2d = convolution::Conv2d::new(img.clone());
      let runner = convolution::Runner::new(img, conv2d);
      let result = runner.run().save(dst);
    
      println!("Result: {:?}", result);
    }*/
}
