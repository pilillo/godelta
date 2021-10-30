extern crate anyhow;
extern crate deltalake;

async fn open_table_example(){
    let table = deltalake::open_table("./tests/data/simple_table").await.unwrap();
    let files = table.get_files();
    println!("{}", files.len());
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {

    println!("Hello, world!");
    
    let ote = open_table_example();

    Ok(())
}
