#[derive(Debug, PartialEq, Clone)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  Icelandic,
  Arabic,
  Ukrainian,
  German,
}

#[derive(Clone)]
struct Greeting {
    message: String,
    lang: Lang,
}

fn main() {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Icelandic, message: String::from("Halló WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Arabic, message: String::from("!WasmEdge مرحبًا") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Ukrainian, message: String::from("Привіт WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Hallo WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Guten Tag WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("Guten Abend WasmEdge!") };
  v.push(g);

  let cloned_v: Vec<Greeting>  = v.clone();

  for e in v {
    println!("{:?} {}", e.lang, e.message);
  }

  let query = Lang::Icelandic;
  let m: Greeting = cloned_v.clone().into_iter().find(|g| g.lang == query).unwrap();
  println!("\nQuery: {:?} Response: {}", query, m.message);
  
  let query = Lang::German;
  let m: Vec<Greeting> = cloned_v.into_iter().filter(|g| g.lang == query).collect();
  println!("\nQuery: {:?}", query);
  for e in m{
    println!("Greeting: {}", e.message);
  }


}
