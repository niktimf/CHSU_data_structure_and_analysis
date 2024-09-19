use anyhow::Result as AnyHowResult;
use polars::io::SerReader;
use polars::prelude::CsvReadOptions;
use polars::series::ChunkCompare;

fn main() -> AnyHowResult<()> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("music_log.csv".into()))?
        .finish()?;

    // Задание 5.
    // Вывести первые 5 строк таблицы.
    println!("Задание 5.");
    let music_head = df.head(Some(5));
    println!("{:?}", music_head);
    println!("\n");

    // Задание 6.
    // Вывести последние 5 строк таблицы.
    println!("Задание 6.");
    let music_tail = df.tail(Some(5));
    println!("{:?}", music_tail);
    println!("\n");

    // Задание 7.
    // Вывести размер таблицы.
    println!("Задание 7.");
    let shape_table = df.shape();
    println!("{:?}", shape_table);
    println!("\n");

    // Задание 8.
    // Вывести количество наблюдений в таблице.
    println!("Задание 8.");
    let observations_table = shape_table.0;
    println!("Количество наблюдений: {}", observations_table);
    println!("\n");

    // Задание 9.
    // Не выполнить задание, так как observations_info_table не инициализирована.

    // Задание 10.
    // Получение таблицы со столбцами genre и Artist.
    println!("Задание 10.");
    let genre_fight = df.select(["genre", "Artist"])?;
    println!("{}", genre_fight);
    println!("\n");

    // Задание 11.
    // Посчитайте число прослушанных треков в жанре поп
    println!("Задание 11.");
    let genre_pop = genre_fight.filter(&genre_fight.column("genre")?.equal("pop")?)?;
    let count_pop_tracks = genre_pop.height();
    println!("Количество треков в жанре 'pop': {}", count_pop_tracks);
    println!("\n");

    // Задание 12.
    // Посчитайте число прослушанных треков в жанре рок
    println!("Задание 12.");
    let genre_rock = genre_fight.filter(&genre_fight.column("genre")?.equal("rock")?)?;
    let count_rock_tracks = genre_rock.height();
    println!("Количество треков в жанре 'rock': {}", count_rock_tracks);
    println!("\n");

    // Задание 13.
    // Напишите условную конструкцию, которая сравнивает полученные значения и
    // выводит информацию о победителе в этом бою! Если победил жанр рок, то выведите сообщение
    // "Рок победил!", а если победил жанр поп - сообщение "Попса forever!"
    println!("Задание 13.");
    if count_rock_tracks > count_pop_tracks {
        println!("Рок победил!");
    } else {
        println!("Попса forever!");
    }
    println!("\n");

    // Задание 14.
    // Получите таблицу только с жанром rock и сохраните её в переменной rock.
    println!("Задание 14.");
    let rock = genre_fight.filter(&genre_fight.column("genre")?.equal("rock")?)?;
    println!("{}", rock);
    println!("\n");


    Ok(())
}
