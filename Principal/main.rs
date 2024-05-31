const PI:f32 = 3.14; //const precisa sempre informar o valor.
//static VARIAVEL_GLOBAL:u8 = 1; -- variável global usar o static (static = const, ambas precisam ter valores definidos), como mutáveis é inseguro.
static mut VARIAVEL_GLOBAL:u8 = 1;// variável global usar o static (static = const, ambas precisam ter valores definidos), como mutáveis é inseguro.

fn soma(a:i32, b:i32) -> i32 // Parâmetros precisa informar o tipo. O retorno é "mostrado" com "->" e o tipo do retorno.
{
    println!("{} + {} = {}", a, b, a + b);
    a + b // para retornar não utiliza o ;. Quando omito o ; ele me dá o retorno, quando utilizo eu sinalizo que é o fim da expressão e ignoro. Pode utilizar return também.
}

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

    // let essa_string:&'static str= "meu nome"; // É um valor constante e não aloca memória, mas tem endereço. Não tem como pegar, por exemplo: essa_string[0]

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

    println!("Soma = {}", soma(2, 2));

    condicionais();
    repeticoes();
}

fn condicionais() {
    let idade:u8 = 18;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    if eh_maior{
        println!("Pode entrar na balada!");
    } else if  idade > 16 && responsavel_autorizou {
        println!("Pode entrar na balada com assinatura do responsável");
    }
    else{
        println!("Não pode entrar na balada!");
    }

    let condicao = if eh_maior { "maior" } else { "menor" };

    println!("É {} de idade", condicao);

    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "KOTLIN" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido"
    };

    println!("O propósito de {} é {}", linguagem, proposito);
}

fn repeticoes(){
    let multiplicador:u8 = 5;

    let mut contador:u8 = 0;

    while contador < 10 {
        contador += 1;

        if contador == 5 { 
            continue; 
        }
        
        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);
    }

    contador = 0;
    loop {
        contador += 1;

        if contador == 5 { 
            continue; 
        }

        println!("{} x {} = {}", multiplicador, contador, multiplicador * contador);

        if contador == 10 {
            break;
        }
    }

    contador = 0;
    for i in 1..11{ // ou 1..=10
        contador += 1;

        if contador == 5 { 
            continue; 
        }

        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}