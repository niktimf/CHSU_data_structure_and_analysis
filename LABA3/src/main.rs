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
    let rock = df.filter(&genre_fight.column("genre")?.equal("rock")?)?;
    println!("{}", rock);
    println!("\n");

    // Задание 15.
    // Сохраните столбец 'total play' таблицы rock в переменной rock_time.
    println!("Задание 15.");
    let rock_time = rock.column("total play")?;
    println!("{}", rock_time);
    println!("\n");

    // Задание 16.
    // Обратитесь к новой Series c именем rock_time и посчитайте количество треков
    // жанра рок, пропущенных в течение 5 секунд. Логическим условием укажите rock_time <= 5.
    // Результат сохраните в переменной rock_haters
    println!("Задание 16.");
    let rock_haters = rock_time.filter(&rock_time.lt_eq(5.0)?)?.iter().count();
    println!("{}", rock_haters);
    println!("\n");

    // Задание 17.
    // Получите таблицу только с жанром pop и сохраните её в переменной pop.
    println!("Задание 17.");
    let pop = df.filter(&genre_fight.column("genre")?.equal("pop")?)?;
    println!("{}", pop);
    println!("\n");

    // Задание 18.
    // Сохраните столбец 'total play' таблицы pop в переменной pop_time.
    println!("Задание 18.");
    let pop_time = pop.column("total play")?;
    println!("{}", pop_time);
    println!("\n");

    // Задание 19.
    // Обратитесь к новой Series c именем pop_time и посчитайте количество треков
    // жанра поп, пропущенных в течение 5 секунд. Логическим условием укажите pop_time <= 5.
    // Результат сохраните в переменной pop_haters
    println!("Задание 19.");
    let pop_haters = pop_time.filter(&pop_time.lt_eq(5.0)?)?.iter().count();
    println!("{}", pop_haters);
    println!("\n");

    // Задание 20.
    // Для обоих жанров посчитайте долю быстро пропущенных пользователями композиций в процентах.
    // Разделите количество треков, которые пользователи пропустили
    // — соответственно rock_haters и pop_haters — на общее количество треков жанра рок и жанра поп.
    // Общее количество треков жанра равно количеству наблюдений в таблицах rock и pop,
    // т.е. значению атрибута shape[0] этих таблиц.
    // Результаты сохраните в переменных rock_skip и pop_skip (от англ. skip, «пропуск»).
    // Выведите значения новых переменных в процентах с точностью до одного знака после запятой
    println!("Задание 20.");
    let rock_skip = (rock_haters as f64 / rock.height() as f64) * 100.0;
    let pop_skip = (pop_haters as f64 / pop.height() as f64) * 100.0;
    println!("Доля пропущенных треков жанра рок: {:.1}%", rock_skip);
    println!("Доля пропущенных треков жанра поп: {:.1}%", pop_skip);
    Ok(())
}
