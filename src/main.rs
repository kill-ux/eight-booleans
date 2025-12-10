use bitwise_operators::BitWise;

fn main() {
    let mut booleans = BitWise::new();

    println!("###### Set #######");
    booleans.desplay();
    booleans.set(1.into(), true);
    booleans.desplay();
    println!("####################");

    for i in 0..=7 {
        println!(" index {i} = {}", booleans.read(i.into()));
    }

    println!("###### Toggle ######");
    booleans.desplay();
    booleans.toggle(1.into());
    booleans.desplay();
    println!("####################");

    println!("###### Clear ######");
    booleans.desplay();
    booleans.clear(1.into());
    booleans.desplay();
    println!("####################");
}
