title: Rust - Uma linguagem interessante
author:
  name: Bruno Tavares
  twitter: bltavares
  url: http://bltavares.com
output: index.html
controls: true

---

# Rust

## Uma linguagem interessante

---

### Rust
#### Uma linguagem interessante

Venha conhecer essa nova linguagem desenvolvida pela Mozilla, que foca
em baixo nível, mas com a ergonomia que parece mais alto nível. "Essa
palestra seria uma primeira introdução ao Rust, uma linguagem
desenvolvida pela Mozilla. Rust é uma linguagem de programação para
sistemas de baixo nível, com enfoque a velocidade, prevencão de erros
de memória e segurança entre threads. Apesar de ser uma linguagem mais
procedural que permite acesso baixo nivel, ideal para pequenos devices
e programas gráficos, ela possui muitos dos beneficios de uma
linguagem de alto nível. Muitos das construções que ela usa é bem
familiar para quem está acostumado com o paradigma funcional, usando
um sistema de tipos algebrico, não tem nulo e diversas outras
coisas. Em breve, teremos partes do Firefox se beneficiando de todas
essas novidades. Essa palestra seria uma introdução a linguagem,
explicando o que eles buscam, um pouco de syntax, casos de uso atuais,
ferramentas, comunidade, e todo o potencial que a ferramenta tem. Mais
informações sobre a linguagem em <https://www.rust-lang.org/>

---

## Olá!

Bruno Tavares

    Consultor na ThoughtWorks

@bltavares

---

## Antes de tudo...

O que me levou a me interessar por Rust?

---

## Um pouco sobre os projetos que já passei

-   PHP
-   Ruby
-   Java
-   Python
-   JavaScript no navegador
-   JavaScript no servidor

---

## Projetos mais recentes

    Maiores e mais complexos

-   Bancos, Cluster, Filas, Servicos
-   Linguagens diferentes, runtimes diferentes
-   Versões diferentes de linguagens e pacotes

---

## Dificuldade de iniciar em um novo projeto

    Meu laptop: Uma coleção de boostraps incompletos

-   gem -> scons -> gulp -> bower -> npm -> brew/apt
-   1 VM, 3 Containeres
-   3 serviços Python
-   2 serviços Java
-   2 serviços Ruby
-   1 Bash, 2 Bancos
-   3 frameworks JS, 2 Compiladores de JavaScript
-   Chef, Jenkins, 4 bots (Ruby, Node e Clojure)

---

## Dificuldade de iniciar em um novo projeto

    Meu laptop: Uma coleção de boostraps incompletos

Projeto mais maduro em estrutura de entrega.
Runtime complexo.

---

## Como explicar para quem é iniciante?

    As coisas ficam simples depois de muita exposição

---

## Como explicar para quem é iniciante?

    Todos somos iniciantes em alguma área

---

## Dá pra ser mais simples?

---

## Dá pra ser mais simples?

-   Diminuir quantidade de runtimes pra desenvolver?
-   Diminuir quantidade de runtimes pra executar?
-   Automatizar o empacotamento?
-   Execução simples em produção e desenvolvimento?

---

## Dá pra ser mais simples?

Sim

---

## Descendo a tech stack

    Dando uma olhada uma camada abaixo

---

## Descendo a tech stack

    Dando uma olhada uma camada abaixo

-   Sei as necessidades da camada de cima
-   Não sei sobre a camada de baixo

---

## Descendo a tech stack

    Dando uma olhada uma camada abaixo

-   Cross plataforma e facilmente disponível
-   Processo de instalação simples
-   Runtime pequeno

---

## Descendo a tech stack

    Escolhas disponíveis

-   Bash
-   C

---

## Existem outras opções?

-   Evitar internalizar todo os problemas desse nível
-   Aprender gradualmente, de preferência com as ferramentas

---

## Existem outras opções?

-   Go
-   D
-   Rust

---

## Rust

    Motivos que me chamaram a atenção

-   Cross plataforma
-   Saída em binário
-   Compilador que impede possíveis errors
-   Permite que eu explore o nível no meu passo
-   Ideal para fazer os utilitários do projetos

---

## Rust

    Hello World

```rust
fn main() {
    println!("Hello World");
}
```

---

## Rust

    Principios da linguagem

> Rust is a systems programming language that
> runs blazingly fast,
> prevents segfaults,
> and guarantees thread safety.

<https://www.rust-lang.org/>

---

## Rust

    Principios da linguagem

> Rust é uma linguagem de programação para sistemas
> que executa rápido,
> previne erros acesso de memória,
> e garante segurança entre threads

---

## Rust

    Com conceitos novos e antigos

-   Macros
-   Inferência de tipos
-   Verificação de tempo de vida de valores
-   Verificação de empréstimo de valores
-   Gerenciamento de memória sem GC
    (mas com ajuda do compilador)

---

## Rust

    Com conceitos de liguagem funcional

-   Imutabilidade por padrão
-   Sistema de tipos algébrico
-   Closures
-   Pattern matching
-   Separação de comportamento e dados

---

## Rust

    Imutabilidade por padrão

```rust
fn main() {
  let x = 1;
  x = 2;
}
```

    error: re-assignment of immutable variable `x` [E0384]

---

## Rust

    Imutabilidade por padrão

```rust
fn main() {
  let mut x = 1;
  x = 2;
}
```

---

## Rust

    Sistema de tipos algébricos

```rust
enum Color {
    Red,
    Blue,
    Yellow,
}
```

---

## Rust

    Erros são explicitos

```rust
impl File {
    fn open(path: Path) -> Result<File>
}

enum Result<T> {
   Ok(T),
   Err(io::Error)
}
```

---

## Rust

    Pattern matching
    Não há null

```rust
fn main() {
    let data : Vec<i32> = vec!();
    match data.first() {
        None => println!("Nope"),
        Some(x) => println!("Here: {}", x),
    }
}
```

---

## Rust

    Baixo que parece alto nível

```rust
for line in content.lines().filter(|x| x.is_empty()) {
  let words: Vec<String> = line.split(PHRASE_TERMINATOR)
    .flat_map(|x| x.split_whitespace())
    .map(|x| x.to_owned())
    .collect();

  chain.feed(words);
}
let words: Vec<String> = chain.str_iter_for(100).collect();
```

---

## Rust

    Não deixa que eu faça erros comuns nessa camada

```rust
fn main() {
    let valor = "Hello";
    usa_e_libera_memoria(valor);
    // println!("Valor agora: {}", valor);
}
```

---

## Rust

    Com conceitos de liguagem funcional

-   Imutabilidade por padrão
-   Sistema de tipos algébrico
-   Closures
-   Pattern matching
-   Separação de comportamento e dados

---

## Rust

    O que eu tenho visto que me mantém

-   Progressão estável
    `Stability without stagination`
-   Evolução transparente
    Processo de RFCs
-   Comunidade amigável
    `Somos todos iniciantes em alguma área`
-   Possibilidades
    Utilitários, Extensões de outras linguagens
    Sistemas Operacionais, Unikernel, Mobile

---

## Rust

- [Skylight](https://www.skylight.io/) Extendendo outros runtimes

- [Autumn](http://autumnai.com/) Machine learning

- [Tessel](https://www.tessel.io/) IoT

- [Dropbox](https://blogs.dropbox.com/tech/2016/06/lossless-compression-with-brotli/) Plataforma
- [Chef Habitat](https://news.ycombinator.com/item?id=11905560) Plataforma

- [cargo-rumpbake](https://github.com/gandro/cargo-rumpbake) Unikernel

---

## Rust

    Projetos interessantes

-   [Servo](https://github.com/servo/servo)
-   [Helix](https://github.com/rustbridge/helix)
-   [Neon](https://github.com/rustbridge/neon)
-   [coreutils](https://github.com/uutils/coreutils)
-   [redox](https://github.com/redox-os/redox) e [intermezzOs](https://intermezzos.github.io/)
-   [Xi](https://github.com/google/xi-editor) editor
-   [habitat](https://github.com/habitat-sh/habitat)

---

## Rust

    Por onde começar

-   [rustup.rs](https://www.rustup.rs/)
-   [rust-br.com](http://rust-br.com/)
-   [This Week In Rust](https://this-week-in-rust.org)
-   [rust-learnings](https://github.com/ctjhoa/rust-learning)
-   [rustfmt](https://github.com/rust-lang-nursery/rustfmt)
-   [awesome-rust](https://github.com/kud1ing/awesome-rust)

---

## Rust

### Uma linguagem interessante

-   Vários conceitos funcionais
-   Poucas dependencias em runtime
-   Novas estratégias para gerenciamento de memória
-   Muitas possibilidades!

---

## Obrigado!

    @bltavares
