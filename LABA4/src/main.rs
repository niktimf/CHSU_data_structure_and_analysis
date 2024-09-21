use anyhow::Result as AnyHowResult;
use polars::datatypes::{AnyValue, PlSmallStr};
use polars::frame::UniqueKeepStrategy;
use polars::io::SerReader;
use polars::prelude::{
    col, lit, CsvParseOptions, CsvReadOptions, Expr, IntoLazy, NullValues, SchemaExt,
};
use polars::series::ChunkCompare;
use std::sync::Arc;

fn main() -> AnyHowResult<()> {
    let df = CsvReadOptions::default()
        .with_has_header(true)
        .with_projection(Some(Arc::new((0..=4).collect())))
        .with_parse_options(CsvParseOptions::with_null_values(
            CsvParseOptions::default(),
            Some(NullValues::AllColumnsSingle(PlSmallStr::from_str("NaN"))),
        ))
        .try_into_reader_with_file_path(Some("music_log.csv".into()))?
        .finish()?;
    println!("{}", df);

    // Задание 1.
    // Просмотрите информацию о наборе данных.
    println!("Задание 1.");
    df.schema().iter_fields().for_each(|field| {
        println!("{:?}", field);
    });
    println!("\n");

    // Задание 2.
    // Выведите список столбцов.
    println!("Задание 2.");
    let columns = df.get_column_names();
    println!("{:?}", columns);
    println!("\n");

    // Задание 3.
    // Подготовьте список new_names с новыми именами для столбцов.
    // • user_id → user_id
    // • total play → total_play_seconds
    // • Artist → artist_name
    // • genre → genre_name
    // • track → track_name
    let new_names = vec![
        "user_id",
        "total_play_seconds",
        "artist_name",
        "genre_name",
        "track_name",
    ];
    println!("\n");

    // Задание 4.
    // Переименуйте столбцы таблицы.
    let mut df_renamed = df.clone();
    df_renamed.set_column_names(new_names)?;
    println!("\n");

    // Задание 5.
    // Проверьте, что получилось, запросив для нашей структуры данных df атрибут columns.
    println!("Задание 5.");
    println!("{:?}", df_renamed.get_column_names());
    println!("\n");

    // Задание 6.
    // Просмотрите информацию о наборе данных
    println!("Задание 6.");
    df_renamed.schema().iter_fields().for_each(|field| {
        println!("{:?}", field);
    });
    println!("\n");

    // Задание 7.
    // Посчитайте количество пропущенных значений и выведите его на экран.
    println!("Задание 7.");
    let missing_values = df_renamed.null_count();
    println!("Количество пропущенных значений: {}", missing_values);
    println!("\n");

    // Задание 8.
    // Заполните отсутствующие значения столбца 'track_name' строкой 'unknown'
    println!("Задание 8.");
    let df_filled_track_name = df_renamed
        .clone()
        .lazy()
        .with_columns([col("track_name").fill_null(lit("unknown"))])
        .collect()?;
    println!("{}", df_filled_track_name);
    println!("\n");

    // Задание 9.
    // Заполните отсутствующие значения столбца 'artist_name' словом unknown.
    println!("Задание 9.");
    let df_filled_artist_name = df_filled_track_name
        .clone()
        .lazy()
        .with_columns([col("artist_name").fill_null(lit("unknown"))])
        .collect()?;
    println!("{}", df_filled_artist_name);
    println!("\n");

    // Задание 10.
    // Удалите пропущенные значения из столбца 'genre_name'.
    println!("Задание 10.");
    let df_dropped_genre_name = df_filled_artist_name
        .clone()
        .lazy()
        .drop_nulls(Some(vec![Expr::from("genre_name")]))
        .collect()?;
    println!("{}", df_dropped_genre_name);
    println!("\n");

    // Задание 11.
    // Проверьте полученный результат. Просмотрите информацию о наборе данных.
    println!("Задание 11.");
    df_dropped_genre_name
        .schema()
        .iter_fields()
        .for_each(|field| {
            println!("{:?}", field);
        });
    println!("\n");

    // Задание 12.
    // Сохраните текущий размер таблицы в переменной shape_table.
    println!("Задание 12.");
    let shape_table = df_dropped_genre_name.shape();
    println!("{:?}", shape_table);
    println!("\n");

    // Задание 13.
    // Посчитайте и выведите на экран суммарное количество дубликатов в таблице.
    println!("Задание 13.");
    let duplicates_count = df_dropped_genre_name.is_duplicated()?.sum().unwrap();
    println!("Количество дубликатов: {}", duplicates_count);
    println!("\n");

    // Задание 14.
    // Удалите дубликаты из таблицы.
    println!("Задание 14.");
    let df_dropped_duplicates =
        df_dropped_genre_name.unique_stable(None, UniqueKeepStrategy::First, None)?;
    println!("{}", df_dropped_duplicates);
    println!("\n");

    // Задание 15.
    // Сохраните в переменную shape_table_update размер таблицы после удаления дубликатов.
    println!("Задание 15.");
    let shape_table_update = df_dropped_duplicates.shape();
    println!(
        "Размер таблицы после удаления дубликатов: {:?}",
        shape_table_update
    );
    println!("\n");

    // Задание 16.
    // Сравните переменные shape_table и shape_table_update.
    // Если они равны, выведите сообщение 'Размер таблицы не изменился, текущий размер: ' и значение переменной shape_table_update.
    // В ином случае сообщение должно быть таким: 'Таблица уменьшилась, текущий размер: ' и значение переменной shape_table_update.
    println!("Задание 16.");
    if shape_table == shape_table_update {
        println!(
            "Размер таблицы не изменился, текущий размер: {:?}",
            shape_table_update
        );
    } else {
        println!(
            "Таблица уменьшилась, текущий размер: {:?}",
            shape_table_update
        );
    }
    println!("\n");

    // Задание 17.
    // Получите уникальные значения столбца 'genre_name'.
    // Просмотрите результат и найдите название жанра, которое выпадает из общего ряда.
    println!("Задание 17.");
    let unique_genre_names = df_dropped_duplicates.column("genre_name")?.unique()?;
    let genre_names_as_strings: Vec<String> = unique_genre_names
        .iter()
        .filter_map(|val| match val {
            AnyValue::String(s) => Some(s.to_string()),
            _ => None,
        })
        .collect();
    println!("{:?}", genre_names_as_strings);
    println!("\n");

    // Задание 18.
    // Оцените изменения: пересчитайте количество значений 'электроника' в столбце 'genre_name'.
    // Если удалось всё заменить, результат должен быть равен 0.
    // Сохраните этот результат в переменной genre_final_count, выведите на экран.
    // Примените к отобранным по логическому условию df['genre_name'] == 'электроника' значениям столбца 'genre_name' метод count() для подсчёта.
    // Результат сохраните в переменной genre_final_count, значение которой напечатайте на экране.
    println!("Задание 18.");
    let genre_electronics = df_dropped_duplicates.filter(
        &df_dropped_duplicates
            .column("genre_name")?
            .equal("электроника")?,
    )?;
    let genre_final_count = genre_electronics.height();
    println!(
        "Количество строк с жанром 'электроника': {}",
        genre_final_count
    );

    Ok(())
}
