const PI:f32 = 3.14; //const precisa sempre informar o valor.
//static VARIAVEL_GLOBAL:u8 = 1; -- variável global usar o static (static = const, ambas precisam ter valores definidos), como mutáveis é inseguro.
static mut VARIAVEL_GLOBAL:u8 = 1;// variável global usar o static (static = const, ambas precisam ter valores definidos), como mutáveis é inseguro.

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);
        println!("dentro, a = {}", a); // tudo o que existe fora, existe dentro desse escopo

        let a = 777; //Essa váriavel a não é o mesmo a de fora. Ela meio que faz "sombra" da váriavel, ela sobrepõem a outra dentro desse escopo. {}
        println!("dentro, a = {}", a); // tudo o que existe fora, existe dentro desse escopo
    }
    //println!("b = {}", b); - Não consigo acessar pois não está no escopo "geral"

    println!("fora, a = {}", a);
}

fn escopo(){

    let essa_string:&'static str= "meu nome"; // É um valor constante e não aloca memória, mas tem endereço. Não tem como pegar, por exemplo: essa_string[0]

    println!("PI = {}", PI); // todos lugares que tiver o PI ele vai substituir tudo por 3.14 e não armazena memória.

    unsafe{ // colocando assim o sistema não acusa e deixa rodar sendo mutável global
        println!("Variável Global = {}", VARIAVEL_GLOBAL);
    }
    
    let variavel:i32 = 128;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    
    let decimal:f32 = 2.5;
    println!("variavel = {}, tamanho = {} bytes", decimal, std::mem::size_of_val(&decimal));
    
    let booleana:bool = false; // -- Por padrão não é mutável o valor que é definido ele não muda.
    // let mut booleana:bool = false; - Mut é pra definir que é mutável e pode alterar o valor dentro da função.
    //booleana = true;
    println!("variavel = {}, tamanho = {} bytes", booleana, std::mem::size_of_val(&booleana));
    
    let letra:char = 'C';
    println!("variavel = {}, tamanho = {} bytes", letra, std::mem::size_of_val(&letra));
}

fn main(){
    escopo();
    sombra();
}