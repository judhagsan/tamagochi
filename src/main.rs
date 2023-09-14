use std::io;
use std::thread;
use std::time::Duration;

struct Tamagotchi {
    nome: String,
    felicidade: u32,
    fome: u32,
    sede: u32,
    saude: u32,
    sujo: u32,
}

impl Tamagotchi {
    fn new(nome: &str) -> Tamagotchi {
        let capitalized_nome = Tamagotchi::capitalize_first_letter(nome);
        Tamagotchi {
            nome: capitalized_nome,
            felicidade: 50,
            fome: 50,
            sede: 50,
            saude: 50,
            sujo: 50,
        }
    }

    fn status(&self) {
        println!(
            "Estatus do(a) {}\n Felicidade: {}%\n Fome: {}%\n Sede: {}%\n Saúde: {}%\n Sujo: {}%\n",
            self.nome, self.felicidade, self.fome, self.sede, self.saude, self.sujo
        );
    }

    fn brincar(&mut self) {
        println!("Você está brincando com {}!", self.nome);
        self.felicidade += 10;
        self.fome += 5;
        self.sede += 5;
        self.saude += 5;
        self.sujo += 5;
    }

    fn dar_comida(&mut self) {
        println!("Você deu comida para {}!", self.nome);
        self.felicidade += 5;
        self.fome -= 10;
        self.sede += 5;
        self.saude += 5;
        self.sujo += 5;
    }

    fn dar_agua(&mut self) {
        println!("Você deu água para {}!", self.nome);
        self.felicidade += 5;
        self.fome += 5;
        self.sede -= 10;
        self.saude += 5;
        self.sujo += 5;
    }

    fn dar_banho(&mut self) {
        println!("Você deu um banho em {}!", self.nome);
        self.felicidade += 5;
        self.fome += 5;
        self.sede += 5;
        self.saude += 10;
        self.sujo -= 10;
    }

    fn passar_tempo(&mut self) {
        self.felicidade -= 2;
        self.fome += 3;
        self.sede += 3;
        self.sujo += 4;
        self.saude -= 2;

        if self.fome >= 100 || self.sede >= 100 || self.sujo >= 100 {
            println!("{} morreu de fome, sede ou sujeira!\n", self.nome);
        }

        if self.felicidade <= 0 {
            println!("{} morreu de tristeza!\n", self.nome);
        }
    }

    fn capitalize_first_letter(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

fn main() {
    println!("Bem-vindo ao Tamagotchi!");
    println!("Digite o nome do seu Tamagotchi:");

    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Falha ao ler o nome.");
    nome = nome.replace("\r", "").replace("\n", "♥").replace("\"", "");
    nome = nome.trim().to_string();

    let mut tamagotchi = Tamagotchi::new(&nome);

    loop {
        tamagotchi.status();
        println!("Digite a ação que deseja realizar:");
        println!("1. Brincar");
        println!("2. Dar comida");
        println!("3. Dar água");
        println!("4. Dar banho");
        println!("5. Sair");

        let mut escolha = String::new();
        io::stdin()
            .read_line(&mut escolha)
            .expect("Falha ao ler a escolha.");

        match escolha.trim().parse() {
            Ok(opcao) => match opcao {
                1 => tamagotchi.brincar(),
                2 => tamagotchi.dar_comida(),
                3 => tamagotchi.dar_agua(),
                4 => tamagotchi.dar_banho(),
                5 => {
                    println!("Saindo do Tamagotchi.");
                    break;
                }
                _ => println!("Opção inválida!\n"),
            },
            Err(_) => println!("Por favor, digite um número válido.\n"),
        }

        tamagotchi.passar_tempo();
        thread::sleep(Duration::from_secs(1));
    }
}
