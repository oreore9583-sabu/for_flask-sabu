use pyo3::prelude::*;
fn parentheses(let mut result: Vec<String>)->Vec<String>{
 
    let mut zi = 0
    while zi < len(data){
        if result[zi] == "-"{
            if zi == 0 || result[zi - 1].contains(&"+".to_string())||result[zi - 1].contains(&"-".to_string())||result[zi - 1].contains(&"*".to_string())||result[zi - 1].contains(&"/".to_string())||result[zi - 1].contains(&"(".to_string()){
                            result[zi : zi + 2] = [result[zi] + result[zi + 1]]
                zi = 0
                continue
    }
            }zi += 1
}return result;}
fn tokenize(result: &str) -> Vec<String> {
  let mut note: String = String::new();
  let mut af_result: Vec<String> = Vec::new();
  let chars: Vec<char> = result.chars().collect();
  let mut next: bool = true;
  for (i, c) in result.chars().enumerate() {
      //enumerate()は一個目にindex2個目にvalue
      if next {
          match c {
              '*' => {
                  if let Some(next_char) = chars.get(i + 1) {
                      if next_char == &'*' {
                          next = false;
                          if !note.is_empty(){
                          af_result.push(note.to_string());}
                          af_result.push("**".to_string());

                          note.clear();
                      } else {
                          if !note.is_empty(){
                          af_result.push(note.to_string());}
                          af_result.push(c.to_string());
                          note.clear();
                      }
                  } else {
                      if !note.is_empty(){
                      af_result.push(note.to_string());}
                      af_result.push(c.to_string());
                      note.clear();
                  }
              }
              '+' | '-' | '/' | '(' | ')' => {
                  if !note.is_empty(){
                  af_result.push(note.to_string());}
                  af_result.push(c.to_string());
                  note.clear();
              }

              _ => note.push_str(&c.to_string()),
          }
      } else {
          next = true;
      }

      // 30
  }

  if !note.is_empty() {
      af_result.push(note.to_string());
  }

  af_result
}
fn caco(mut result: Vec<String>) -> Vec<String> {
  let mut how_deep = 0;
  let mut deep: Vec<(i32, &str)> = Vec::new();

  for c in &result {
      if c == "(" {
          how_deep += 1;

          deep.push((how_deep, c.as_str()));
      } else if c == ")" {
          deep.push((how_deep, c.as_str()));
          how_deep -= 1;
      } else {
          deep.push((how_deep, c.as_str()));
      }
      if how_deep < 0 {
          break;
      }
  }
  if how_deep != 0 {

          return result;



  } else {
      let max_value = deep.iter().map(|x| x.0).max();

      // 2. 提示されたチェーンにそのまま組み込む
      if result.iter().any(|x| x == "(") && result.iter().any(|x| x == ")") {
          let indexes: Vec<usize> = deep
              .iter()
              .enumerate()
              .filter(|(_, x)| Some(x.0) == max_value)
              .map(|(i, _)| i)
              .collect();
          let start = indexes[0]; // この位置から右へ探す

          // position を使うことで、start に一番近い ")" が見つかる
          let index = result[start..]
              .iter()
              .position(|x| x == ")")
              .map(|pos| pos + start);

          result.splice(
              indexes[0]..index.unwrap()+1,
              caco(result[indexes[1]..index.unwrap()].to_vec()).into_iter(),
          );


          return caco(result);
      } else {
          return calculator(result);
      }
  }
}

fn calculator(mut result: Vec<String>) -> Vec<String> {
  println!("{:?}", result);
  let index = result.iter().rposition(|x| x == "**");
  match index {
      //この上で複数の演算子を読み込むかどうか
      Some(i) => {
          let ansewer = result[i - 1]
              .parse::<f32>()
              .unwrap()
              .powf(result[i + 1].parse::<f32>().unwrap());
          let ansewer = ansewer.to_string();
          result.splice(i - 1..i + 2, std::iter::once(ansewer));
          return calculator(result);
      }
      None => {
          let pos_mul = result.iter().position(|x| x == "*");
          let pos_div = result.iter().position(|x| x == "/");
          match (pos_mul, pos_div) {
              // 両方あるなら、インデックスが小さい方を優先
              (Some(m), Some(d)) => {
                  if m < d {
                      let ansewer = result[m - 1].parse::<f32>().unwrap()
                          * result[m + 1].parse::<f32>().unwrap();
                      let ansewer = ansewer.to_string();
                      result.splice(m - 1..m + 2, std::iter::once(ansewer));
                      return calculator(result);
                      // * を計算
                  } else {
                      // / を計算
                      let ansewer = result[d - 1].parse::<f32>().unwrap()
                          / result[d + 1].parse::<f32>().unwrap();
                      let ansewer = ansewer.to_string();
                      result.splice(d - 1..d + 2, std::iter::once(ansewer));
                      return calculator(result);
                  }
              }
              _ => (),
          }
          let index = result.iter().position(|x| x == "*");
          match index {
              //この上で複数の演算子を読み込むかどうか
              Some(i) => {
                  let ansewer = result[i - 1].parse::<f32>().unwrap()
                      * result[i + 1].parse::<f32>().unwrap();
                  let ansewer = ansewer.to_string();
                  result.splice(i - 1..i + 2, std::iter::once(ansewer));
                  return calculator(result);
              }
              None => {
                  let index = result.iter().position(|x| x == "/");
                  match index {
                      //この上で複数の演算子を読み込むかどうか
                      Some(i) => {
                          let ansewer = result[i - 1].parse::<f32>().unwrap()
                              / result[i + 1].parse::<f32>().unwrap();
                          let ansewer = ansewer.to_string();
                          result.splice(i - 1..i + 2, std::iter::once(ansewer));
                          return calculator(result);
                      }
                      None => {
                          let pos_add = result.iter().position(|x| x == "+");
                          let pos_min = result.iter().position(|x| x == "-");
                          match (pos_add, pos_min) {
                              // 両方あるなら、インデックスが小さい方を優先
                              (Some(m), Some(d)) => {
                                  if m < d {
                                      let ansewer = result[m - 1].parse::<f32>().unwrap()
                                          + result[m + 1].parse::<f32>().unwrap();
                                      let ansewer = ansewer.to_string();
                                      result.splice(m - 1..m + 2, std::iter::once(ansewer));
                                      return calculator(result);
                                      // * を計算
                                  } else {
                                      // / を計算
                                      let ansewer = result[d - 1].parse::<f32>().unwrap()
                                          - result[d + 1].parse::<f32>().unwrap();
                                      let ansewer = ansewer.to_string();
                                      result.splice(d - 1..d + 2, std::iter::once(ansewer));
                                      return calculator(result);
                                  }
                              }
                              _ => (),
                          }
                          let index = result.iter().position(|x| x == "+");
                          match index {
                              //この上で複数の演算子を読み込むかどうか
                              Some(i) => {
                                  let ansewer = result[i - 1].parse::<f32>().unwrap()
                                      + result[i + 1].parse::<f32>().unwrap();
                                  let ansewer = ansewer.to_string();
                                  result.splice(i - 1..i + 2, std::iter::once(ansewer));
                                  return calculator(result);
                              }
                              None => {
                                  let index = result.iter().position(|x| x == "-");
                                  match index {
                                      //この上で複数の演算子を読み込むかどうか
                                      Some(i) => {
                                          let ansewer = result[i - 1].parse::<f32>().unwrap()
                                              - result[i + 1].parse::<f32>().unwrap();
                                          let ansewer = ansewer.to_string();
                                          result.splice(i - 1..i + 2, std::iter::once(ansewer));
                                          return calculator(result);
                                      }
                                      None => return result,
                                  }
                              }
                          };
                      }
                  }
              }
          }
      }
  }
  result
}

#[pyfunction]
fn calc(expr: String) -> PyResult<String> {
        

    

    // unwrap() で String を取り出し、& をつけて &str として渡す
   

    // 修正後（値が入っていない場合にパニックさせる）
    let result: String = expr.chars().filter(|c| !c.is_whitespace()).collect();

    let af_result = tokenize(&result);
    let anser=caco(af_result);
    Ok(anser[0].clone())
}

#[pymodule]
fn my_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc, m)?)?;
    Ok(())
}
