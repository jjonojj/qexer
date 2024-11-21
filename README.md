# qexer
this is the lexer i use for my ~~programming languages~~ parser experiments. very basic, its probably not suited for your use case.
handling of string literals is very bad yet, since i didnt need that yet for my languages.
## example

src/main.rs
```rust
use qexer;

fn main() {
  let mut q =
    qexer::Tokenizer::new("int main(int argc, char** argv) { printf("hello world!"); }");
  let mut tok = qexer::Token::None;
  whlie tok != qexer::Token::EOF {
    tok = q.next();
    printf("{}", tok);
  }
}

```

outputs this:
```
string(value: int)
string(value: main)
left parenthesis
string(value: int)
string(value: argc)
comma
string(value: char)
star
star
string(value: argv)
right parenthesis
left brace
string(value: printf)
left parenthesis
double quote
string(value: hello)
string(value: world)
exclamation mark
double quote
right parenthesis
semicolon
right brace
end of file
```


Token implements Display, so it can be printed nicely :)
