use anyhow::{bail, ensure, Context, Result};

pub struct Calculator(usize);
impl Calculator {
    pub fn new(verbose: usize) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        // 空白で分割、Vec型へ変換
        // rev()はVecの中身をpopで取り出す際、末尾から取り出すことから取り出す値を入力順にするための反転処理
        let mut tokens = formula.split_whitespace().rev().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
        let mut stack: Vec<i32> = Vec::new();
        let mut position = 0;

        while let Some(token) = tokens.pop() {
            position += 1;

            if let Ok(x) = token.parse::<i32>() {
                // 数値ならばスタックへプッシュ
                stack.push(x);
            } else {
                // スタックから2つ取り出す
                let y = stack.pop().context(format!("invalid syntax at {}.", position))?;
                let x = stack.pop().context(format!("invalid syntax at {}.", position))?;

                let result = match token {
                    "+" => x + y,
                    "-" => x - y,
                    "*" => x * y,
                    "/" => x / y,
                    "%" => x % y,
                    _ => bail!("invalid token at {}.", position),
                };

                stack.push(result)
            }

            // 無名のフィールドは数値で指定可能
            if self.0 == 1 {
                // 計算過程を出力
                println!("{:?} {:?}", tokens, stack);
            }
        }

        ensure!(stack.len() == 1, "invalid token.");

        Ok(stack[0])
    }
}

// 正常系
#[test]
fn test_ok() {
    let calc: Calculator = Calculator::new(0);
    assert_eq!(calc.eval("5").unwrap(), 5);
    assert_eq!(calc.eval("50").unwrap(), 50);
    assert_eq!(calc.eval("-50").unwrap(), -50);
    assert_eq!(calc.eval("2 3 +").unwrap(), 5);
    assert_eq!(calc.eval("2 3 *").unwrap(), 6);
    assert_eq!(calc.eval("2 3 -").unwrap(), -1);
    assert_eq!(calc.eval("2 3 /").unwrap(), 0);
    assert_eq!(calc.eval("2 3 %").unwrap(), 2);
}

// 異常系
#[test]
fn test_ng() {
    let calc: Calculator = Calculator::new(0);
    assert!(calc.eval("1 1 ~").is_err());
    assert!(calc.eval("1 1 1 +").is_err());
    assert!(calc.eval("+ 1 1").is_err());
}
