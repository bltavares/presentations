title: Uma jornada de descoberta sobre desenvolvimento
author:
  name: Bruno Tavares
  twitter: bltavares
  url: http://bltavares.com
output: basic.html
controls: true

---

## Uma jornada de descoberta sobre desenvolvimento<a id="orgheadline2"></a>

## do PHP ao Rust<a id="orgheadline1"></a>

---

### Olá!<a id="orgheadline3"></a>

Bruno Tavares

@bltavares

---

### PHP<a id="orgheadline4"></a>

    Como era simples

```php
<?php

print("Hello World");
```

---

### PHP<a id="orgheadline5"></a>

    Como era simples

-   Colecionava hosts de 250Mb
-   Copiar tudo por FTP

---

### Ruby<a id="orgheadline6"></a>

    Tá ficando complicado

```shell
rails new
```

---

### Ruby<a id="orgheadline7"></a>

    Free host 250Mb

-   Como eu coloco isso na internet?
-   Copia tudo no colocado&#x2026;

---

### Ruby<a id="orgheadline8"></a>

    VPS

-   Aluga um servidor inteiro
-   `rvm`
-   `capistrano`
-   `passenger`

---

### Projetos novos<a id="orgheadline9"></a>

    Maiores e mais complexos

-   Bancos, Cluster, Filas, Servicos
-   Linguagens diferentes, runtimes diferentes
-   Versões diferentes de linguagens e pacotes

---

### Dificuldade de iniciar em um novo projeto<a id="orgheadline10"></a>

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

### Como explicar para quem é iniciante?<a id="orgheadline11"></a>

    As coisas ficam simples depois de muita exposição

-   Complexidade de projetos web

---

### Como explicar para quem é iniciante?<a id="orgheadline12"></a>

    Todos somos iniciantes em alguma área

---

### Dá pra ser mais simples?<a id="orgheadline13"></a>

---

### Dá pra ser mais simples?<a id="orgheadline14"></a>

-   Diminuir quantidade de runtimes pra desenvolver?
-   Diminuir quantidade de runtimes pra executar?
-   Automatizar o empacotamento?
-   Execução simples em produção e desenvolvimento?

---

### Dá pra ser mais simples?<a id="orgheadline15"></a>

Sim

---

### Dá pra ser mais simples?<a id="orgheadline16"></a>

    Mapeando cada dependencia implicita

---

### Dá pra ser mais simples?<a id="orgheadline17"></a>

    DevOps

-   VMs, containers, runtimes
-   Chef, Puppet, Ansible, Bash

---

### Descendo a stack<a id="orgheadline18"></a>

    Dando uma olhada uma camada abaixo

---

### Descendo a stack<a id="orgheadline19"></a>

    Dando uma olhada uma camada abaixo

-   Sei as necessidades da camada de cima
-   Não sei sobre a camada de baixo

---

### Descendo a stack<a id="orgheadline20"></a>

    Dando uma olhada uma camada abaixo

-   Cross plataforma e ubiquoto
-   Processo de instalação simples
-   Runtime pequeno

---

### Descendo a stack<a id="orgheadline21"></a>

    Escolhas disponíveis

-   Bash
-   C

---

### Existem outras opções?<a id="orgheadline22"></a>

-   Evitar internalizar todo os problemas desse nível
-   Aprender gradualmente, de preferência com as ferramentas

---

### Existem outras opções?<a id="orgheadline23"></a>

-   Go
-   D
-   Rust

---

### Rust<a id="orgheadline24"></a>

    Motivos que me chamaram a atenção

-   Cross plataforma
-   Saída em binário
-   Compilador que grita comigo quando erro
-   Permite que eu explore o nível no meu passo
-   Ideal para fazer os utilitários do projetos

---

### Rust<a id="orgheadline25"></a>

    Hello World

```rust
fn main() {
    println!("Hello World");
}
```

---

### Rust<a id="orgheadline26"></a>

    Parece alto nível

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

### Rust<a id="orgheadline27"></a>

    Com conceitos de liguagem funcional

-   Imutabilidade por padrão
-   Sistema de tipos algebrico

---

### Rust<a id="orgheadline28"></a>

    Não deixa que eu faça erros comuns nessa camada

```rust
fn main() {
    let valor = "Hello";
    usa_e_libera_memoria(valor);
    // println!("Valor agora: {}", valor);
}
```

---

### Rust<a id="orgheadline29"></a>

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

### Rust<a id="orgheadline30"></a>

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

### Rust<a id="orgheadline31"></a>

    Extendendo outros runtimes

-   [Skylight](https://www.skylight.io/)

---

### Rust<a id="orgheadline32"></a>

    Machine learning

-   [Autumn](http://autumnai.com/)

---

### Rust<a id="orgheadline33"></a>

    IoT

-   [Tessel](https://www.tessel.io/)

---

### Rust<a id="orgheadline34"></a>

    Plataforma

-   Dropbox
-   Chef Habitat
---

### Rust<a id="orgheadline35"></a>

    Unikernel

-   [cargo-rumpbake](https://github.com/gandro/cargo-rumpbake)

---

### Rust<a id="orgheadline36"></a>

    Projetos interessantes

-   [Servo](https://github.com/servo/servo)
-   [Helix](https://github.com/rustbridge/helix)
-   [Neon](https://github.com/rustbridge/neon)
-   [coreutils](https://github.com/uutils/coreutils)
-   [redox](https://github.com/redox-os/redox) e [intermezzOs](https://intermezzos.github.io/)
-   [Xi](https://github.com/google/xi-editor) editor

---

### Rust<a id="orgheadline37"></a>

    Por onde começar

-   [rustup.rs](https://www.rustup.rs/)
-   [http://rust-br.com/](#rust-br)
-   [This Week In Rust](https://this-week-in-rust.org)
-   [rust-learnings](https://github.com/ctjhoa/rust-learning)
-   [rustfmt](https://github.com/rust-lang-nursery/rustfmt)
-   [awesome-rust](https://github.com/kud1ing/awesome-rust)

---

### Minha maior aprendizagem nessa jornada<a id="orgheadline38"></a>

    É possível transferir conhecimento entre camadas

---

### Minha maior aprendizagem nessa jornada<a id="orgheadline39"></a>

    É possível transferir conhecimento entre camadas

-   Olhar na camada de baixo te faz aprender sobre a de cima
-   Siga o seu passo
-   É possível inovar indo pra baixo
-   É possível trazer novidades para a base

---

### Obrigado!<a id="orgheadline40"></a>

    @bltavares
